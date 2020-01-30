const autoComplete = require('./js/auto-complete.js');
var noUiSlider = require('nouislider');

window.addEventListener('load', function () {
    import('./pkg/webgl')
        .then(async (webgl) => {
            // Start our Rust application. You can find `WebClient` in `src/lib.rs`
            let webClient = new webgl.WebClient();
            retrieveCatalog('J/A+A/566/A43/table2', colnames=['RAJ2000', 'DEJ2000', 'Bmag', 'Per'])
                .then(sources => webClient.add_catalog(sources));

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
                onSelect: function(e, term, item){
                    let hips_id = term;
    
                    let hips_url = hipsesMap[hips_id].hips_service_url;
                    let max_depth = hipsesMap[hips_id].max_depth;
                    webClient.change_hips(hips_url, max_depth);
                }
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
            
            // Load a source catalog from vizier
            // Do the Ajax query
            let catalogArray = [];
            let catalogMap = new Map();
            new autoComplete({
                selector: '#catalog-selector',
                minChars: 2,
                source: function(term, suggest) {
                    term = term.toLowerCase();
                    var choices = catalogArray;
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
                onSelect: async function(e, term, item){
                    let cat_id = term;
    
                    let parallax_column_name = await getTableColumnName(cat_id, "pos.parallax");
                    console.log('parallax column name: ', parallax_column_name);
                    let phot_mag_column_name = await getTableColumnName(cat_id, "phot.mag");
                    console.log('phot mag column name: ', phot_mag_column_name);

                    let pos_ra_column_name = await getTableColumnName(cat_id, "pos.eq.ra");
                    let pos_dec_column_name = await getTableColumnName(cat_id, "pos.eq.dec");

                    let table_obs_id = cat_id.substring(4);
                    retrieveCatalog(table_obs_id, [pos_ra_column_name, pos_dec_column_name, phot_mag_column_name, parallax_column_name])
                        .then(sources => {
                            console.log('sources: ', sources);
                            webClient.add_catalog(sources);
                        });

                }
            });
            const url_catalogs = 'https://alasky.u-strasbg.fr/MocServer/query?expr=dataproduct_type%3Dcatalog%26%26nb_rows<%3D500000%26%26nb_rows>%3D50000%26%26data_ucd%3Dpos.parallax*%26%26data_ucd%3Dphot.mag*&get=record&fmt=json';
            // Create our request constructor with all the parameters we need
            var request = {
                method: 'GET',
                headers: new Headers(),
                mode: 'cors',
                cache: 'default'
            };
            fetch(url_catalogs, request)
                .then(response => response.json())
                .then((catalogs) => {
                    for (var k = 0; k < catalogs.length; k++) {
                        var cat_id = catalogs[k].ID;

                        catalogMap[cat_id] = {
                            'obs_id': catalogs[k].obs_id,
                        };
                        catalogArray.push(cat_id);
                    }
                    console.log('catalogs found!: ', catalogArray);
                    console.log('num catalogs found!: ', catalogArray.length);
                });

            // Init the noUI range slider
            let sizeSourceSlider = document.getElementById('slider-handles');

            noUiSlider.create(sizeSourceSlider, {
                start: [0.02, 0.02],
                connect: true,
                range: {
                    'min': 0.01,
                    'max': 0.06
                },
                tooltips: [true, true],
            });
            sizeSourceSlider.noUiSlider.on('update', function (values) {
                webClient.set_range_source_size(values[0], values[1]);
            });

            // Add the UI event listeners

            let select_projection = document.getElementById("proj-select");
            let equatorial_grid = document.getElementById("enable-grid");
            let inertia = document.getElementById("enable-inertia");
            let fps_counter = document.getElementById("fps-counter");
            let grid_color_picker = document.getElementById("grid-color");
            let grid_opacity = document.getElementById("grid-alpha");
            let catalog_opacity = document.getElementById("catalog-alpha");
            let kernel_strength = document.getElementById("kernel-strength");
            let colormap_selector = document.getElementById("colormap-select");

            let canvas = document.getElementById("canvas");
            //canvas.focus();

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
            let onchange_inertia = () => {
                if (inertia.checked) {
                    webClient.enable_inertia();
                } else {
                    webClient.disable_inertia();
                }
            };

            // Projection selector
            select_projection.addEventListener("change", () => {
                let projection = select_projection.value;

                webClient = webClient.set_projection(projection);
                console.log("change projection to: ", projection);

                //onchange_equatorial_grid();
            }, false);

            // Enable equatorial grid checkbox
            equatorial_grid.addEventListener("change", () => {
                onchange_equatorial_grid()
            }, false);

            // Enable equatorial grid checkbox
            inertia.addEventListener("change", () => {
                onchange_inertia()
            }, false);

            // Change grid color
            let parse_hex_color = function(color) {
                m = color.match(/^#([0-9a-f]{6})$/i)[1];
                if(m) {
                    return {
                        'red': parseInt(m.substr(0,2),16) / 255.0,
                        'green': parseInt(m.substr(2,2),16) / 255.0,
                        'blue': parseInt(m.substr(4,2),16) / 255.0,
                    }
                }
            }

            grid_color_picker.addEventListener("input", (event) => {
                let color_hex = event.target.value;
                let color = parse_hex_color(color_hex);

                webClient.change_grid_color(color['red'], color['green'], color['blue']);
            }, false);

            // Alpha grid
            grid_opacity.addEventListener("input", (event) => {
                let opacity = event.target.value;

                webClient.change_grid_opacity(opacity);
            }, false);

            // Alpha catalog
            catalog_opacity.addEventListener("input", (event) => {
                let opacity = event.target.value;

                webClient.set_heatmap_opacity(opacity);
            }, false);

            // Alpha catalog
            kernel_strength.addEventListener("input", (event) => {
                let strength = event.target.value;

                webClient.set_kernel_strength(strength);
            }, false);

            // Alpha catalog
            colormap_selector.addEventListener("change", () => {
                let colormap = colormap_selector.value;
                webClient.set_colormap(colormap);
            }, false);
            
            // Touchpad events
            (() => {
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
                        webClient.stop_move(touche.pageX, touche.pageY);
                        console.log('stop moving');
                    }
                }, false);
            
                let handleCancel = (evt) => {
                    evt.preventDefault();
                    if (ongoingTouches.length == 1) {
                        // move event
                        // Stop moving
                        let touche = ongoingTouches[0];
                        webClient.stop_move(touche.pageX, touche.pageY);
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
            })();
            // Mouse events
            (() => { 
                canvas.addEventListener("mousedown", (evt) => {
                    webClient.initialize_move(evt.clientX, evt.clientY);
                });
                canvas.addEventListener("mouseup", (evt) => {
                    webClient.stop_move(evt.clientX, evt.clientY);
                });
                canvas.addEventListener("mousemove", (evt) => {
                    webClient.moves(evt.clientX, evt.clientY);
                });
            })();
            // Wheel events
            (() => {
                let canvas = document.getElementById("canvas");

                canvas.addEventListener("wheel", (evt) => {
                    webClient = webClient.zoom(evt.deltaY);
                }, false);
            })();

            // Resize event
            window.addEventListener('resize', () => {
                let width = window.innerWidth;
                let height = window.innerHeight;
        
                webClient.resize(width, height);
            });

            // Render
            time = Date.now();
            render()
        })
        .catch(console.error);
  })

function getTableColumnName(table_name, ucd) {
    let table_obs_id = table_name.substring(4);
    let url = 'http://tapvizier.u-strasbg.fr/TAPVizieR/tap/sync?phase=RUN&lang=adql&format=json&request=doQuery&query=SELECT%20TOP%201%20table_name%2C%20column_name%2C%20ucd%20FROM%20TAP_SCHEMA.columns%20WHERE%20table_name%3D%27' + encodeURIComponent(table_obs_id) + '%27%20AND%20ucd%20LIKE%20%27' + encodeURIComponent(ucd) + '%25%27';
    var request = {
        method: 'GET',
        headers: new Headers(),
        mode: 'cors',
        cache: 'default'
    };
    return fetch(url, request)
        .then(response => response.json())
        .then(table => {
            // Return the column name of the first row corresponding to ucd
            console.log(table_name, ucd, ": here are the data", table.data);
            return table.data[0][1];
        });
}

function retrieveCatalog(table_obs_id, colnames, max_rows="*") {
    let cols = [];
    colnames.forEach(col => {
        console.log(col);
        cols.push('"' + table_obs_id + '"."' + encodeURIComponent(col) + '"');
    });

    let cols_query = cols.join(", ");

    let sql_query = 'SELECT ' + cols_query + ' FROM "' + table_obs_id + '"';
    console.log(sql_query);
    
    let url = 'http://tapvizier.u-strasbg.fr/TAPVizieR/tap/sync?phase=RUN&lang=adql&format=json&request=doQuery&query=' + encodeURIComponent(sql_query);

    var request = {
        method: 'GET',
        headers: new Headers(),
        mode: 'cors',
        cache: 'default'
    };
    return fetch(url, request)
        .then(response => response.json())
        .then((votable) => {
            let sources = votable.data;
            return sources;
        });
}