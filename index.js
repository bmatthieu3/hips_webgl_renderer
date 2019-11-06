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
            select_projection.addEventListener("change", () => {
                let projection = select_projection.value;

                webClient.set_projection(projection);
                console.log("change projection to: ", projection);
            });
        })
        .catch(console.error);
  })