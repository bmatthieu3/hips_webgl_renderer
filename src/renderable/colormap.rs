use crate::shader::Shaderize;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;

    layout (location = 0) in vec2 position;
    layout (location = 1) in vec2 uv;

    out vec2 out_uv;

    void main() {
        gl_Position = vec4(position, 0.f, 1.f);
        out_uv = uv;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    out vec4 color;

    uniform sampler2D texture_fbo;
    uniform sampler2D colormap;
    uniform float alpha;

    float colormap_red(float x) {
        if (x < 0.1131206452846527) {
            return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;
        } else if (x < 0.5116005837917328) {
            return 0.0;
        } else if (x < 0.5705677568912506) {
            return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;
        } else if (x < 0.622047244) {
            return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;
        } else if (x < 0.7922459542751312) {
            return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;
        } else {
            return 3.34889230769210E+02 * x - 1.41006123680226E+02;
        }
    }
    
    float colormap_green(float x) {
        if (x < 0.114394336938858) {
            return 0.0;
        } else if (x < 0.4417250454425812) {
            return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;
        } else if (x < 0.4964917968308496) {
            return 3.11150000000070E+02 * x + 9.54249999999731E+01;
        } else if (x < 0.6259051214039278) {
            return -1.03272635599706E+03 * x + 7.62648586707481E+02;
        } else if (x < 0.8049814403057098) {
            return -2.92799028677160E+02 * x + 2.99524283071235E+02;
        } else {
            return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;
        }
    }
    
    float colormap_blue(float x) {
        if (x < 0.4424893036638088) {
            return 3.09636968527514E+02 * x + 9.62203074056821E+01;
        } else if (x < 0.5) {
            return -4.59921428571535E+02 * x + 4.36741666666678E+02;
        } else if (x < 0.5691165986930345) {
            return -1.81364912280674E+03 * x + 1.05392982456125E+03;
        } else if (x < 0.6279306709766388) {
            return 1.83776470588197E+02 * x - 8.28382352940910E+01;
        } else {
            return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;
        }
    }
    
    vec4 colormap_f(float x) {
        float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);
        float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);
        float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);
        return vec4(r, g, b, 1.0);
    }

    void main() {
        float opacity = texture(texture_fbo, out_uv).r;

        float o = smoothstep(0.f, 0.1f, opacity);

        //color = texture(colormap, vec2(opacity, 0.5f));
        color = colormap_f(opacity);
        color.a = alpha * o;
    }
"#]
pub struct BluePastelRed;

#[derive(Shaderize)]
#[VertexShader = r#"#version 300 es
    precision lowp float;

    layout (location = 0) in vec2 position;
    layout (location = 1) in vec2 uv;

    out vec2 out_uv;

    void main() {
        gl_Position = vec4(position, 0.f, 1.f);
        out_uv = uv;
    }
"#]
#[FragmentShader = r#"#version 300 es
    precision lowp float;

    in vec2 out_uv;
    out vec4 color;

    uniform sampler2D texture_fbo;
    uniform sampler2D colormap;
    uniform float alpha;

    float colormap_red(float x) {
        if (x < 0.4128910005092621) {
            return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;
        } else if (x < 0.5004365747118258) {
            return -1.99292307692284E+01 * x + 2.54503076923075E+02;
        } else if (x < 0.6000321805477142) {
            return -4.46903540903651E+02 * x + 4.68176638176691E+02;
        } else {
            return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;
        }
    }

    float colormap_green(float x) {
        if (x < 0.3067105114459991) {
            return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;
        } else if (x < 0.4045854562297116) {
            return 3.64978461538455E+02 * x + 8.50984615384636E+01;
        } else if (x < 0.5035906732082367) {
            return 1.25827692307720E+02 * x + 1.81855384615367E+02;
        } else {
            return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;
        }
    }

    float colormap_blue(float x) {
        if (x < 0.1012683545126085) {
            return 5.85993431855501E+01 * x + 4.56403940886700E+00;
        } else if (x < 0.2050940692424774) {
            return 3.51072173913048E+02 * x - 2.50542028985514E+01;
        } else if (x < 0.5022056996822357) {
            return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;
        } else if (x < 0.5970333516597748) {
            return -1.62299487179500E+02 * x + 3.26660512820525E+02;
        } else {
            return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;
        }
    }

    vec4 colormap_f(float x) {
        float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);
        float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);
        float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);
        return vec4(r, g, b, 1.0);
    }

    void main() {
        float opacity = texture(texture_fbo, out_uv).r;

        float o = smoothstep(0.f, 0.1f, opacity);

        //color = texture(colormap, vec2(opacity, 0.5f));
        color = colormap_f(opacity);
        color.a = alpha * o;
    }
"#]
pub struct IDL_CB_BrBG;