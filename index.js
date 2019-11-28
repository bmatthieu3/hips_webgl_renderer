const autoComplete = require('./js/auto-complete.js');
window.addEventListener('load', function () {
    import('./pkg/webgl')
        .then((webgl) => {
            let hipsesArray = [];
            let hipsesMap = new Map();
            new autoComplete({
                selector: '#hips-selector',
                minChars: 2,
                source: function(term, suggest) {
                    term = term.toLowerCase();
                    var choices = hipsesArray;
                    var matches = [];
                    for (i=0; i<choices.length; i++) {
                        if (choices[i].toLowerCase().indexOf(term)>=0) {
                            matches.push(choices[i]);
                        }
                    }
                    suggest(matches);
                },
                renderItem: function (item, search) {
                    search = search.replace(/[-\/\\^$*+?.()|[\]{}]/g, '\\$&');
                    var re = new RegExp("(" + search.split(' ').join('|') + ")", "gi");
                    return '<div class="autocomplete-suggestion" data-val="' + item + '">' + item.replace(re, "<b>$1</b>") + '</div>';
                },
            });

            const url = 'https://alasky.u-strasbg.fr/MocServer/query?hips_service_url*=*alasky*&&dataproduct_type=image&&hips_tile_format=*jpeg*&get=record&fmt=json';
            // Create our request constructor with all the parameters we need
            var request = {
                method: 'GET',
                headers: new Headers(),
                mode: 'cors',
                cache: 'default'
            };
            fetch(url, request)
                .then(response => response.json())
                .then((hipses) => {
                    for (var k = 0; k < hipses.length; k++) {
                        var hips_id = hipses[k].ID;

                        hipsesMap[hips_id] = {
                            'hips_service_url': hipses[k].hips_service_url,
                            'max_depth': hipses[k].hips_order,
                        };
                        hipsesArray.push(hips_id);
                    }
                    console.log(hipsesArray);
                });

            // Add the UI event listeners

            let select_projection = document.getElementById("proj-select");
            let equatorial_grid = document.getElementById("enable-grid");
            let hips_selector = document.getElementById("hips-selector");
            let hips_selector_validate = document.getElementById("hips-selector-validate");
            let fps_counter = document.getElementById("fps-counter");

            // Start our Rust application. You can find `WebClient` in `src/lib.rs`
            const webClient = new webgl.WebClient();

            let time = Date.now();
            let time_last_fps = time;
            function render () {
                const dt = Date.now() - time;
                // Get the FPS every second
                if (time - time_last_fps > 1000) {
                    fps_counter.innerText = String(1000.0 / dt);
                    time_last_fps = time;
                }

                webClient.update(dt)
                webClient.render()
                window.requestAnimationFrame(render)

                time = Date.now()
            }

            let onchange_equatorial_grid = () => {
                if (equatorial_grid.checked) {
                    webClient.enable_equatorial_grid();
                } else {
                    webClient.disable_equatorial_grid();
                }
            };

            // Projection selector
            select_projection.addEventListener("change", () => {
                let projection = select_projection.value;

                webClient.set_projection(projection);
                console.log("change projection to: ", projection);

                onchange_equatorial_grid();
            });

            // Enable equatorial grid checkbox
            equatorial_grid.addEventListener("change", () => {
                onchange_equatorial_grid()
            });

            // Change HiPS
            hips_selector_validate.addEventListener("click", () => {
                let hips_id = hips_selector.value;
                console.log(hips_id, hipsesMap);

                let hips_url = hipsesMap[hips_id].hips_service_url;
                let max_depth = hipsesMap[hips_id].max_depth;
                webClient.change_hips(hips_url, max_depth);
            });
            
            // Touchpad event
            touchpad_events(webClient);
            // Mouse events
            mouse_events(webClient);
            // Wheel events
            wheel_events(webClient);

            // Render
            time = Date.now();
            render()
        })
        .catch(console.error);
  })

// Mouse EVENT
function mouse_events(webClient) {
    let canvas = document.getElementById("canvas");

    canvas.addEventListener("mousedown", (evt) => {
        webClient.initialize_move(evt.clientX, evt.clientY);
    });
    canvas.addEventListener("mouseup", (evt) => {
        webClient.stop_move();
    });
    canvas.addEventListener("mousemove", (evt) => {
        webClient.moves(evt.clientX, evt.clientY);
    });
}

// Wheel EVENT
function wheel_events(webClient) {
    let canvas = document.getElementById("canvas");

    // Test via a getter in the options object to see if the passive property is accessed
    /*let supportsPassive = false;
    try {
        var opts = Object.defineProperty({}, 'passive', {
            get: function() {
                supportsPassive = true;
            }
        });
        window.addEventListener("testPassive", null, opts);
        window.removeEventListener("testPassive", null, opts);
    } catch (e) {}

    console.log("support passive: ", supportsPassive);*/
    canvas.addEventListener("wheel", (evt) => {
        webClient.zoom(evt.deltaY);
    }, false);
}

// Touchpad EVENT
function touchpad_events(webClient) {
    let canvas = document.getElementById("canvas");
    let ongoingTouches = new Array();

    function ongoingTouchIndexById(id) {
        for(let idx = 0; idx < ongoingTouches.length; idx++) {
            let touch = ongoingTouches[idx];
            if (touch.identifier == id) {
                return idx;
            }
        }

        return -1;
    }

    // Test via a getter in the options object to see if the passive property is accessed
    /*var supportsPassive = false;
    try {
        var opts = Object.defineProperty({}, 'passive', {
            get: function() {
                supportsPassive = true;
            }
        });
        window.addEventListener("testPassive", null, opts);
        window.removeEventListener("testPassive", null, opts);
    } catch (e) {}*/
    canvas.addEventListener("touchstart", (evt) => {
        evt.preventDefault();
        var touches = evt.changedTouches;

        for (var i = 0; i < touches.length; i++) {
            ongoingTouches.push(touches[i]);
        }
        let touche = ongoingTouches[0];
        if (ongoingTouches.length == 1) {
            webClient.initialize_move(touche.pageX, touche.pageY);
        } else {
            // If more touches are present, we stop the current move
            webClient.stop_move();
            console.log('stop moving');
        }
    }, false);

    let handleCancel = (evt) => {
        evt.preventDefault();
        if (ongoingTouches.length == 1) {
            // move event
            // Stop moving
            webClient.stop_move();
            console.log('stop moving');
        } else {
            // zoom event
        }

        ongoingTouches = [];
    };
    canvas.addEventListener("touchend", handleCancel, false);
    canvas.addEventListener("touchcancel", handleCancel, false);
    canvas.addEventListener("touchleave", handleCancel, false);
    canvas.addEventListener("touchmove", (evt) => {
        evt.preventDefault();
        var touches = evt.changedTouches;

        // Update the touches
        for (var i = 0; i < touches.length; i++) {
            let idx = ongoingTouchIndexById(touches[i].identifier);
            ongoingTouches.splice(idx, 1, touches[i]);
        }

        if (ongoingTouches.length == 1) {
            // Move event
            let touche = ongoingTouches[0];
            webClient.moves(touche.pageX, touche.pageY);

            console.log("move!!");
        } else {
            console.log("zoom!!");
            // zoom event
        }
    }, false);
}