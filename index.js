// For more comments about what's going on here, check out the `hello_world`
// example.
window.addEventListener('load', function () {
    import('./pkg/webgl')
        .then((webgl) => {
            // Start our Rust application. You can find `WebClient` in `src/lib.rs`
            const webClient = new webgl.WebClient();
            webClient.start();

            // Add the UI event listeners
            let select_projection = document.getElementById("proj-select");
            let equatorial_grid = document.getElementById("enable-grid");

            let onchange_equatorial_grid = () => {
                if (equatorial_grid.checked) {
                    webClient.enable_equatorial_grid();
                } else {
                    webClient.disable_equatorial_grid();
                }
            };

            // - Projection selector
            select_projection.addEventListener("change", () => {
                let projection = select_projection.value;

                webClient.set_projection(projection);
                console.log("change projection to: ", projection);

                onchange_equatorial_grid();
            });

            // - Enable equatorial grid checkbox
            equatorial_grid.addEventListener("change", () => {
                onchange_equatorial_grid()
            });
        })
        .catch(console.error);
  })