/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(Object.prototype.hasOwnProperty.call(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".index.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/webgl_bg.wasm": function() {
/******/ 			return {
/******/ 				"./webgl_bg.js": {
/******/ 					"__wbg_log_9aa6786f4afc2ed4": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_log_9aa6786f4afc2ed4"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_json_serialize": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_json_serialize"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_document_c9bb82e72b87972b": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_document_c9bb82e72b87972b"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementById_66a113a03886aac6": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getElementById_66a113a03886aac6"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_0feb941e3d100079": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_0feb941e3d100079"](p0i32);
/******/ 					},
/******/ 					"__wbg_parse_b6e057bc5004a9ad": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_parse_b6e057bc5004a9ad"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getContext_7fb33d2f2a33d083": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getContext_7fb33d2f2a33d083"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGl2RenderingContext_0f1d0ce1f564cddc": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_instanceof_WebGl2RenderingContext_0f1d0ce1f564cddc"](p0i32);
/******/ 					},
/******/ 					"__wbg_createProgram_0e01a0272abf80f2": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createProgram_0e01a0272abf80f2"](p0i32);
/******/ 					},
/******/ 					"__wbg_attachShader_703542f74d3f4655": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_attachShader_703542f74d3f4655"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_13df874c1ec00220": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_linkProgram_13df874c1ec00220"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getProgramParameter_30ca8607cd79baba": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getProgramParameter_30ca8607cd79baba"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbg_getProgramInfoLog_a7e39255ecb0047b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getProgramInfoLog_a7e39255ecb0047b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getActiveUniform_30cc8778e1ba660f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getActiveUniform_30cc8778e1ba660f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_name_3db390fc2fc091fb": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_name_3db390fc2fc091fb"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_2085f6ce304f30ee": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getUniformLocation_2085f6ce304f30ee"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_blendFuncSeparate_a1abbb6bdfc0a7dc": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_blendFuncSeparate_a1abbb6bdfc0a7dc"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_enable_a684b41864515bb5": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_enable_a684b41864515bb5"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_cullFace_bf527eb4959a8aa8": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_cullFace_bf527eb4959a8aa8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_innerWidth_32946fda40a6a23f": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_innerWidth_32946fda40a6a23f"](p0i32);
/******/ 					},
/******/ 					"__wbg_innerHeight_58f8a4f389150bf7": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_innerHeight_58f8a4f389150bf7"](p0i32);
/******/ 					},
/******/ 					"__wbg_canvas_c621ddfce1286dee": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_canvas_c621ddfce1286dee"](p0i32);
/******/ 					},
/******/ 					"__wbg_setwidth_7a7150718cc3f0e6": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setwidth_7a7150718cc3f0e6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setheight_c61e5a9990ad7131": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setheight_c61e5a9990ad7131"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_viewport_b47ae83b1f5bfbe9": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_viewport_b47ae83b1f5bfbe9"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_scissor_40f682e522ab20d2": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_scissor_40f682e522ab20d2"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_new_0599e1276155199a": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_new_0599e1276155199a"]();
/******/ 					},
/******/ 					"__wbg_setcrossOrigin_7c6eec94244d24ba": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setcrossOrigin_7c6eec94244d24ba"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_createVertexArray_cac6aa68bf3644e0": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createVertexArray_cac6aa68bf3644e0"](p0i32);
/******/ 					},
/******/ 					"__wbg_useProgram_c410661f5bd7e81b": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_useProgram_c410661f5bd7e81b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_bindVertexArray_0157543bdbffb06e": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bindVertexArray_0157543bdbffb06e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_45e7132a460d7416": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createBuffer_45e7132a460d7416"](p0i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_cc67660a7f99cf8b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bindBuffer_cc67660a7f99cf8b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_createTexture_0656936373117662": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createTexture_0656936373117662"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_setonload_f35002b2488461d3": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setonload_f35002b2488461d3"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setonerror_eb00bf5798315cb9": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setonerror_eb00bf5798315cb9"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setsrc_7e888e4ced74b27b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_setsrc_7e888e4ced74b27b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_cb_forget": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_cb_forget"](p0i32);
/******/ 					},
/******/ 					"__wbg_getExtension_59696a785661b89b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getExtension_59696a785661b89b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_cd2bdafe6ac6a0ca": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_activeTexture_cd2bdafe6ac6a0ca"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_c152014c1f590c01": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bindTexture_c152014c1f590c01"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_605a63bf3a0a4b49": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texParameteri_605a63bf3a0a4b49"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_3c5bf32a5f1e7dff": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texImage2D_3c5bf32a5f1e7dff"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_createFramebuffer_28e45abdb795ab52": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createFramebuffer_28e45abdb795ab52"](p0i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_47d9a15ce41e5299": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bindFramebuffer_47d9a15ce41e5299"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_framebufferTexture2D_58968036d9154ea5": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_framebufferTexture2D_58968036d9154ea5"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_clearColor_56271725c115c7e9": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_clearColor_56271725c115c7e9"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_clear_7a35983521c8dcf9": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_clear_7a35983521c8dcf9"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_uniform1i_4ddec7fb50640fc1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniform1i_4ddec7fb50640fc1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix4fv_007cb082e7936bd4": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniformMatrix4fv_007cb082e7936bd4"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniform1f_1d29b82642fdb790": function(p0i32,p1i32,p2f32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniform1f_1d29b82642fdb790"](p0i32,p1i32,p2f32);
/******/ 					},
/******/ 					"__wbg_drawElements_0f9b6bbedd3bebcf": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_drawElements_0f9b6bbedd3bebcf"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_width_8d70143974f53aba": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_width_8d70143974f53aba"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_5de843709a60a262": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_height_5de843709a60a262"](p0i32);
/******/ 					},
/******/ 					"__wbg_uniform2f_278c281b29e9e2c3": function(p0i32,p1i32,p2f32,p3f32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniform2f_278c281b29e9e2c3"](p0i32,p1i32,p2f32,p3f32);
/******/ 					},
/******/ 					"__wbg_drawElementsInstanced_28f7d73790509e68": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_drawElementsInstanced_28f7d73790509e68"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_uniform4f_c4d40e40e608ea3b": function(p0i32,p1i32,p2f32,p3f32,p4f32,p5f32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniform4f_c4d40e40e608ea3b"](p0i32,p1i32,p2f32,p3f32,p4f32,p5f32);
/******/ 					},
/******/ 					"__wbg_disable_e64678c1a700d912": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_disable_e64678c1a700d912"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_isArray_1a583db6177943a8": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_isArray_1a583db6177943a8"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_8dacd620a01b769a": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_length_8dacd620a01b769a"](p0i32);
/******/ 					},
/******/ 					"__wbg_get_19b6d907de84efbc": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_get_19b6d907de84efbc"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_texSubImage3D_c2face1549f0ffe2": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texSubImage3D_c2face1549f0ffe2"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32);
/******/ 					},
/******/ 					"__wbg_log_d85e484a8ba03c98": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_log_d85e484a8ba03c98"](p0i32);
/******/ 					},
/******/ 					"__wbg_performance_f7851e83824fd096": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_performance_f7851e83824fd096"](p0i32);
/******/ 					},
/******/ 					"__wbg_now_0aafc2276b5e8d61": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_now_0aafc2276b5e8d61"](p0i32);
/******/ 					},
/******/ 					"__wbg_createShader_d75402f5935812f0": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_createShader_d75402f5935812f0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_34af5e76165e6a02": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_shaderSource_34af5e76165e6a02"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_compileShader_937cb022d149ec18": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_compileShader_937cb022d149ec18"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_d526091c5166c9a7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getShaderParameter_d526091c5166c9a7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderInfoLog_c4c33d53565d30d6": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_getShaderInfoLog_c4c33d53565d30d6"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_deleteVertexArray_adb9b163093754e8": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_deleteVertexArray_adb9b163093754e8"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_disableVertexAttribArray_73c64380cd778613": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_disableVertexAttribArray_73c64380cd778613"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteBuffer_3f297711135614ba": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_deleteBuffer_3f297711135614ba"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteTexture_bd49372545cdb940": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_deleteTexture_bd49372545cdb940"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_texImage3D_2327f290d4c5740d": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texImage3D_2327f290d4c5740d"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32);
/******/ 					},
/******/ 					"__wbg_generateMipmap_77c3e72f06e7a4c2": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_generateMipmap_77c3e72f06e7a4c2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_bufferData_30e5070c8f83f307": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bufferData_30e5070c8f83f307"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_eee48ed0873e601b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_vertexAttribPointer_eee48ed0873e601b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_2fb0e0e3bd9efecc": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_enableVertexAttribArray_2fb0e0e3bd9efecc"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribDivisor_97df148c554b5fea": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_vertexAttribDivisor_97df148c554b5fea"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbg_buffer_3b2c485d32021ccc": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_buffer_3b2c485d32021ccc"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_0474cc8f3c79d5f9": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_newwithbyteoffsetandlength_0474cc8f3c79d5f9"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bufferSubData_d4cc046d2735031b": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_bufferSubData_d4cc046d2735031b"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_onload_d85a3367a2cdcbb0": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_onload_d85a3367a2cdcbb0"](p0i32);
/******/ 					},
/******/ 					"__wbg_uniform1fv_f7f36e76fc1b2c9a": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_uniform1fv_f7f36e76fc1b2c9a"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_new_31f761317592ab6f": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_new_31f761317592ab6f"](p0i32);
/******/ 					},
/******/ 					"__wbg_subarray_da17778cf9ce01c7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_subarray_da17778cf9ce01c7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_texSubImage3D_5eef3d9da8cea53b": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32,p12i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texSubImage3D_5eef3d9da8cea53b"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32,p11i32,p12i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_ee1e475e0e91b3b2": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_texImage2D_ee1e475e0e91b3b2"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_fde2ab6e9bd53d94": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_newwithbyteoffsetandlength_fde2ab6e9bd53d94"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					},
/******/ 					"__wbg_self_8a533577b0c752d3": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_self_8a533577b0c752d3"]();
/******/ 					},
/******/ 					"__wbg_window_5912543aff64b459": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_window_5912543aff64b459"]();
/******/ 					},
/******/ 					"__wbg_globalThis_8f997d48cb67f28e": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_globalThis_8f997d48cb67f28e"]();
/******/ 					},
/******/ 					"__wbg_global_69b29294e4daedff": function() {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_global_69b29294e4daedff"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_714dec97cfe3da72": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_newnoargs_714dec97cfe3da72"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_652fa4cfce310118": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_call_652fa4cfce310118"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_747b56d25bab9510": function(p0i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbg_instanceof_Window_747b56d25bab9510"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper223": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_closure_wrapper223"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper231": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/webgl_bg.js"].exports["__wbindgen_closure_wrapper231"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.name = 'ChunkLoadError';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"1":["./pkg/webgl_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/webgl_bg.wasm":"d8f6a3befb0be378e9db"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./css/auto-complete.css":
/*!*******************************!*\
  !*** ./css/auto-complete.css ***!
  \*******************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var api = __webpack_require__(/*! ../node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js */ \"./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js\");\n            var content = __webpack_require__(/*! !../node_modules/css-loader/dist/cjs.js!./auto-complete.css */ \"./node_modules/css-loader/dist/cjs.js!./css/auto-complete.css\");\n\n            content = content.__esModule ? content.default : content;\n\n            if (typeof content === 'string') {\n              content = [[module.i, content, '']];\n            }\n\nvar options = {};\n\noptions.insert = \"head\";\noptions.singleton = false;\n\nvar update = api(content, options);\n\n\n\nmodule.exports = content.locals || {};\n\n//# sourceURL=webpack:///./css/auto-complete.css?");

/***/ }),

/***/ "./css/styles.css":
/*!************************!*\
  !*** ./css/styles.css ***!
  \************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var api = __webpack_require__(/*! ../node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js */ \"./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js\");\n            var content = __webpack_require__(/*! !../node_modules/css-loader/dist/cjs.js!./styles.css */ \"./node_modules/css-loader/dist/cjs.js!./css/styles.css\");\n\n            content = content.__esModule ? content.default : content;\n\n            if (typeof content === 'string') {\n              content = [[module.i, content, '']];\n            }\n\nvar options = {};\n\noptions.insert = \"head\";\noptions.singleton = false;\n\nvar update = api(content, options);\n\n\n\nmodule.exports = content.locals || {};\n\n//# sourceURL=webpack:///./css/styles.css?");

/***/ }),

/***/ "./events.js":
/*!*******************!*\
  !*** ./events.js ***!
  \*******************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"default\", function() { return Aladin; });\nclass Aladin {\n    constructor(webClient) {\n        this.aladin = webClient;\n        // Keep track of the touches for the events\n        this.ongoingTouches = new Array();\n        // Get the canvas\n        this.canvas = document.getElementById(\"canvas\");\n\n        // Create the events\n        this.createEvents();\n        // Render\n        this.then = Date.now();\n    }\n\n    /// Private methods\n    ongoingTouchIndexById(id) {\n        for(let idx = 0; idx < this.ongoingTouches.length; idx++) {\n            let touch = this.ongoingTouches[idx];\n            if (touch.identifier === id) {\n                return idx;\n            }\n        }\n\n        return -1;\n    }\n\n    removeTouchFromOnGoingTouches(touch) {\n        for(let i = 0; i < this.ongoingTouches.length; i++) {\n            if (this.ongoingTouches[i].identifier == touch.identifier) {\n                this.ongoingTouches.splice(i, 1);\n                break;\n            }\n        }\n    }\n\n    updateTouch(touch) {\n        let idx = this.ongoingTouchIndexById(touch.identifier);\n        this.ongoingTouches.splice(idx, 1, touch);\n    }\n\n    touchEventsOccuring() {\n        return this.ongoingTouches.length > 0;\n    }\n\n    update(elapsed) {\n        let posCenterFov = this.aladin.update(elapsed);\n        return posCenterFov;\n    }\n\n    render() {\n        this.aladin.render()\n    }\n\n    createEvents() {\n        // Touchpad events\n        let actionEnum = {\n            NOTHING: 0,\n            MOVING: 1,\n            ZOOMING: 2,\n            properties: {\n                2: {unZoom: false, startFov: undefined, startDist: undefined},\n            }\n        };\n        let action = actionEnum.NOTHING;\n\n        // Test via a getter in the options object to see if the passive property is accessed\n        this.canvas.addEventListener(\"touchstart\", (e) => {\n            e.preventDefault();\n\n            let touch = e.changedTouches[0];\n            this.ongoingTouches.push(touch);\n\n            let numTouches = this.ongoingTouches.length;\n            if (numTouches == 1) {\n                action = actionEnum.MOVING;\n                let [lon, lat] = this.aladin.screenToWorld(touch.pageX, touch.pageY);\n                this.aladin.setCenter(lon, lat);\n            } else if(numTouches == 2) {\n                // Release the mouse button at the first touch position\n                let firstTouch = this.ongoingTouches[0];\n                let secondTouch = this.ongoingTouches[1];\n                let [lon, lat] = this.aladin.screenToWorld(firstTouch.pageX, firstTouch.pageY);\n                this.aladin.setCenter(lon, lat);\n                // And enter in a zoom event\n                action = actionEnum.ZOOMING;\n\n                // Compute the initial distance between the first two fingers\n                actionEnum.properties[action].startDist = Math.hypot(\n                    firstTouch.pageX - secondTouch.pageX,\n                    firstTouch.pageY - secondTouch.pageY\n                );\n                actionEnum.properties[action].startFov = this.aladin.getFieldOfView();\n            } else {\n                action = actionEnum.NOTHING;\n            }\n        }, false);\n\n        let handleCancel = (e) => {\n            e.preventDefault();\n\n            let touch = e.changedTouches[0];\n            this.removeTouchFromOnGoingTouches(touch);\n\n            if(action === actionEnum.ZOOMING) {\n                // It remains only one touch\n                let [lon, lat] = this.aladin.screenToWorld(this.ongoingTouches[0].pageX, this.ongoingTouches[0].pageY);\n                this.aladin.setCenter(lon, lat);\n                action = actionEnum.MOVING;\n            } else if(action === actionEnum.MOVING) {\n                // The user was moving, so we release the finger\n                let [lon, lat] = this.aladin.screenToWorld(touch.pageX, touch.pageY);\n                this.aladin.setCenter(lon, lat);\n                this.aladin.startInertia();\n                action = actionEnum.NOTHING;\n            }\n        };\n        this.canvas.addEventListener(\"touchend\", handleCancel, false);\n        this.canvas.addEventListener(\"touchcancel\", handleCancel, false);\n        this.canvas.addEventListener(\"touchleave\", handleCancel, false);\n\n        this.canvas.addEventListener(\"touchmove\", (e) => {\n            e.preventDefault();\n\n            // Update the changed touches\n            let touch = e.changedTouches[0];\n            this.updateTouch(touch);\n\n            if (action === actionEnum.MOVING) {\n                // One touch\n                // Move event\n                let [lon, lat] = this.aladin.screenToWorld(touch.pageX, touch.pageY);\n                this.aladin.setCenter(lon, lat);\n            } else if (action === actionEnum.ZOOMING) {\n                // Two touches\n                // Zoom event\n                let curDist = Math.hypot(\n                    this.ongoingTouches[0].pageX - this.ongoingTouches[1].pageX,\n                    this.ongoingTouches[0].pageY - this.ongoingTouches[1].pageY\n                );\n\n                // Pinch zoom \n                let startFov = actionEnum.properties[action].startFov;\n                let startDist = actionEnum.properties[action].startDist;\n\n                this.aladin.setFieldOfView(startFov * startDist / curDist);\n            }\n        }, false);\n    \n        var mouseDown = 0;\n        window.addEventListener(\"mousedown\", (e) => {\n            ++mouseDown;\n        });\n        window.addEventListener(\"mouseup\", (e) => {\n            --mouseDown;\n        });\n        // Mouse events\n        this.canvas.addEventListener(\"mousedown\", (e) => {\n\n            if (this.touchEventsOccuring()) {\n                return;\n            }\n\n            let [lon, lat] = this.aladin.screenToWorld(e.clientX, e.clientY);\n\n            this.aladin.setCenter(lon, lat);\n        });\n        this.canvas.addEventListener(\"mouseup\", (e) => {\n            if (this.touchEventsOccuring()) {\n                return;\n            }\n\n            let [lon, lat] = this.aladin.screenToWorld(e.clientX, e.clientY);\n\n            this.aladin.setCenter(lon, lat);\n            this.aladin.startInertia();\n        });\n        this.canvas.addEventListener(\"mousemove\", (e) => {\n            if (mouseDown) {\n                if (this.touchEventsOccuring()) {\n                    return;\n                }\n\n                let [lon, lat] = this.aladin.screenToWorld(e.clientX, e.clientY);\n\n                this.aladin.setCenter(lon, lat);\n            }\n        });\n        this.canvas.addEventListener('dblclick', (e) => {\n            if (this.touchEventsOccuring()) {\n                return;\n            }\n\n            let [lon, lat] = this.aladin.screenToWorld(e.clientX, e.clientY);\n            this.aladin.moveToLocation(lon, lat);\n        });\n    \n        // Wheel events\n        this.canvas.addEventListener(\"wheel\", (e) => {\n            //if(this.lethargy.check(e) !== false) {\n                if (this.touchEventsOccuring()) {\n                    return;\n                }\n\n                let x = e.deltaY > 0 ? 1 : -1;\n                let fov = this.aladin.getFieldOfView();\n                if (e.deltaY > 0) {\n                    fov = fov * Math.pow(2, 3/2);\n                } else {\n                    fov = fov / Math.pow(2, 3/2);\n                }\n\n                try {\n                    let [lon, lat] = this.aladin.screenToWorld(e.clientX, e.clientY);\n                    this.aladin.zoomToLocation(lon, lat, fov);\n                } catch(error) {\n                    let [lon, lat] = this.aladin.getCenter();\n                    this.aladin.zoomToLocation(lon, lat, fov);\n                }\n            //}\n        }, false);\n    \n        // Resize event\n        window.addEventListener('resize', () => {\n            let width = window.innerWidth;\n            let height = window.innerHeight;\n\n            this.aladin.resize(width, height);\n        });\n    }\n};\n\n//# sourceURL=webpack:///./events.js?");

/***/ }),

/***/ "./img/myfont_0.png":
/*!**************************!*\
  !*** ./img/myfont_0.png ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (__webpack_require__.p + \"bf3093e7600cbe3125c0e04bb8b226c9.png\");\n\n//# sourceURL=webpack:///./img/myfont_0.png?");

/***/ }),

/***/ "./img/myfont_1.png":
/*!**************************!*\
  !*** ./img/myfont_1.png ***!
  \**************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (__webpack_require__.p + \"fd695aa6303f17dd15e65bcd348c4501.png\");\n\n//# sourceURL=webpack:///./img/myfont_1.png?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _shader_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./shader.js */ \"./shader.js\");\n/* harmony import */ var _css_styles_css__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./css/styles.css */ \"./css/styles.css\");\n/* harmony import */ var _css_styles_css__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_css_styles_css__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _css_auto_complete_css__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./css/auto-complete.css */ \"./css/auto-complete.css\");\n/* harmony import */ var _css_auto_complete_css__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_css_auto_complete_css__WEBPACK_IMPORTED_MODULE_2__);\n/* harmony import */ var _img_myfont_0_png__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./img/myfont_0.png */ \"./img/myfont_0.png\");\n/* harmony import */ var _img_myfont_1_png__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./img/myfont_1.png */ \"./img/myfont_1.png\");\n/* harmony import */ var _events_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./events.js */ \"./events.js\");\n/* harmony import */ var _widgets_hips_selector_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./widgets/hips_selector.js */ \"./widgets/hips_selector.js\");\n/* harmony import */ var _widgets_catalog_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./widgets/catalog.js */ \"./widgets/catalog.js\");\n\n\n\n\n\n\n\n\n\n\n\nwindow.addEventListener('load', async () => {\n    // Get our aladin webgl module\n    const webgl = await Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ./pkg/webgl */ \"./pkg/webgl.js\"));\n    let shaders = await Object(_shader_js__WEBPACK_IMPORTED_MODULE_0__[\"load_shaders\"])(webgl);\n    console.log(shaders);\n\n    //console.log(\"ksdlkdsklsd\", fs.readFileSync(\"./src/shaders/catalogs/aitoff.glsl\", { encoding: \"utf8\" }));\n\n    // Start our Rust application. You can find `WebClient` in `src/lib.rs`\n    let webClient = new webgl.WebClient(shaders);\n\n    // Instantiate the HiPS selector widget\n    Object(_widgets_hips_selector_js__WEBPACK_IMPORTED_MODULE_6__[\"HiPSSelectorWidget\"])(webClient);\n    Object(_widgets_catalog_js__WEBPACK_IMPORTED_MODULE_7__[\"CatalogSelectorWidget\"])(webClient);\n\n    // Add the UI event listeners\n    /*let select_projection = document.getElementById(\"proj-select\");\n    let equatorial_grid = document.getElementById(\"enable-grid\");\n    let inertia = document.getElementById(\"enable-inertia\");\n    let fps_counter = document.getElementById(\"fps-counter\");\n    let grid_color_picker = document.getElementById(\"grid-color\");\n    let grid_opacity = document.getElementById(\"grid-alpha\");\n    let catalog_opacity = document.getElementById(\"catalog-alpha\");\n    let kernel_strength = document.getElementById(\"kernel-strength\");\n    let colormap_selector = document.getElementById(\"colormap-select\");\n\n    let ra_input_text = document.getElementById(\"ra\");\n    let dec_input_text = document.getElementById(\"dec\");\n    let ra_value = document.getElementById(\"ra_value\");\n    let dec_value = document.getElementById(\"dec_value\");\n\n    let canvas = document.getElementById(\"canvas\");\n    canvas.focus();\n\n    let onchange_equatorial_grid = () => {\n        if (equatorial_grid.checked) {\n            webClient.enable_equatorial_grid();\n        } else {\n            webClient.disable_equatorial_grid();\n        }\n    };\n    let onchange_inertia = () => {\n        if (inertia.checked) {\n            webClient.enable_inertia();\n        } else {\n            webClient.disable_inertia();\n        }\n    };\n\n    // Projection selector\n    select_projection.addEventListener(\"change\", () => {\n        let projection = select_projection.value;\n\n        webClient.set_projection(projection);\n        console.log(\"change projection to: \", projection);\n\n        //onchange_equatorial_grid();\n    }, false);\n\n    // Enable equatorial grid checkbox\n    equatorial_grid.addEventListener(\"change\", () => {\n        onchange_equatorial_grid()\n    }, false);\n\n    // Enable equatorial grid checkbox\n    inertia.addEventListener(\"change\", () => {\n        onchange_inertia()\n    }, false);\n\n    // Enable equatorial grid checkbox\n    ra_input_text.addEventListener(\"change\", () => {\n        console.log(\"Change!!\");\n\n        let ra = ra_input_text.value;\n        let dec = dec_input_text.value;\n\n        webClient.set_position(ra, dec);\n    }, false);\n    dec_input_text.addEventListener(\"change\", () => {\n        let ra = ra_input_text.value;\n        let dec = dec_input_text.value;\n\n        webClient.set_position(ra, dec);\n    }, false);\n\n    // Change grid color\n    let parse_hex_color = function(color) {\n        m = color.match(/^#([0-9a-f]{6})$/i)[1];\n        if(m) {\n            return {\n                'red': parseInt(m.substr(0,2),16) / 255.0,\n                'green': parseInt(m.substr(2,2),16) / 255.0,\n                'blue': parseInt(m.substr(4,2),16) / 255.0,\n            }\n        }\n    }\n\n    grid_color_picker.addEventListener(\"input\", (event) => {\n        let color_hex = event.target.value;\n        let color = parse_hex_color(color_hex);\n\n        webClient.change_grid_color(color['red'], color['green'], color['blue']);\n    }, false);\n\n    // Alpha grid\n    grid_opacity.addEventListener(\"input\", (event) => {\n        let opacity = event.target.value;\n\n        webClient.change_grid_opacity(opacity);\n    }, false);\n\n    // Alpha catalog\n    catalog_opacity.addEventListener(\"input\", (event) => {\n        let opacity = event.target.value;\n\n        webClient.set_heatmap_opacity(\"Test\", opacity);\n    }, false);\n\n    // Alpha catalog\n    kernel_strength.addEventListener(\"input\", (event) => {\n        let strength = event.target.value;\n\n        webClient.set_kernel_strength(\"Test\", strength);\n    }, false);\n\n    // Alpha catalog\n    colormap_selector.addEventListener(\"change\", () => {\n        let colormap = colormap_selector.value;\n        webClient.set_colormap(\"Test\", colormap);\n    }, false);*/\n\n    let aladin = new _events_js__WEBPACK_IMPORTED_MODULE_5__[\"default\"](webClient);\n\n    (function run() {\n        // Request another frame\n        window.requestAnimationFrame(run)\n    \n        // Begin of a new frame\n        let now = Date.now();\n        // Compute the time between the latest and the new frame\n        let elapsed = now - aladin.then;\n    \n        aladin.then = now;\n        // Frame computation\n        {\n            // Update the client\n            let render = aladin.update(elapsed);\n            // Draw the client\n            if (render) {\n                aladin.render();\n                console.log(\"render \")\n\n            } else {\n                console.log(\"render not\")\n\n            }\n            //ra_value.innerText = pos_center[0].toFixed(4);\n            //dec_value.innerText = pos_center[1].toFixed(4);\n        }\n    })();\n})\n\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./js/auto-complete.js":
/*!*****************************!*\
  !*** ./js/auto-complete.js ***!
  \*****************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("var __WEBPACK_AMD_DEFINE_RESULT__;/*\n    JavaScript autoComplete v1.0.4\n    Copyright (c) 2014 Simon Steinberger / Pixabay\n    GitHub: https://github.com/Pixabay/JavaScript-autoComplete\n    License: http://www.opensource.org/licenses/mit-license.php\n*/\n\nvar autoComplete = (function(){\n    // \"use strict\";\n    function autoComplete(options){\n        if (!document.querySelector) return;\n\n        // helpers\n        function hasClass(el, className){ return el.classList ? el.classList.contains(className) : new RegExp('\\\\b'+ className+'\\\\b').test(el.className); }\n\n        function addEvent(el, type, handler){\n            if (el.attachEvent) el.attachEvent('on'+type, handler); else el.addEventListener(type, handler);\n        }\n        function removeEvent(el, type, handler){\n            // if (el.removeEventListener) not working in IE11\n            if (el.detachEvent) el.detachEvent('on'+type, handler); else el.removeEventListener(type, handler);\n        }\n        function live(elClass, event, cb, context){\n            addEvent(context || document, event, function(e){\n                var found, el = e.target || e.srcElement;\n                while (el && !(found = hasClass(el, elClass))) el = el.parentElement;\n                if (found) cb.call(el, e);\n            });\n        }\n\n        var o = {\n            selector: 0,\n            source: 0,\n            minChars: 3,\n            delay: 150,\n            offsetLeft: 0,\n            offsetTop: 1,\n            cache: 1,\n            menuClass: '',\n            renderItem: function (item, search){\n                // escape special characters\n                search = search.replace(/[-\\/\\\\^$*+?.()|[\\]{}]/g, '\\\\$&');\n                var re = new RegExp(\"(\" + search.split(' ').join('|') + \")\", \"gi\");\n                return '<div class=\"autocomplete-suggestion\" data-val=\"' + item + '\">' + item.replace(re, \"<b>$1</b>\") + '</div>';\n            },\n            onSelect: function(e, term, item){}\n        };\n        for (var k in options) { if (options.hasOwnProperty(k)) o[k] = options[k]; }\n\n        // init\n        var elems = typeof o.selector == 'object' ? [o.selector] : document.querySelectorAll(o.selector);\n        for (var i=0; i<elems.length; i++) {\n            var that = elems[i];\n\n            // create suggestions container \"sc\"\n            that.sc = document.createElement('div');\n            that.sc.className = 'autocomplete-suggestions '+o.menuClass;\n\n            that.autocompleteAttr = that.getAttribute('autocomplete');\n            that.setAttribute('autocomplete', 'off');\n            that.cache = {};\n            that.last_val = '';\n\n            that.updateSC = function(resize, next){\n                var rect = that.getBoundingClientRect();\n                that.sc.style.left = Math.round(rect.left + (window.pageXOffset || document.documentElement.scrollLeft) + o.offsetLeft) + 'px';\n                that.sc.style.top = Math.round(rect.bottom + (window.pageYOffset || document.documentElement.scrollTop) + o.offsetTop) + 'px';\n                that.sc.style.width = Math.round(rect.right - rect.left) + 'px'; // outerWidth\n                if (!resize) {\n                    that.sc.style.display = 'block';\n                    if (!that.sc.maxHeight) { that.sc.maxHeight = parseInt((window.getComputedStyle ? getComputedStyle(that.sc, null) : that.sc.currentStyle).maxHeight); }\n                    if (!that.sc.suggestionHeight) that.sc.suggestionHeight = that.sc.querySelector('.autocomplete-suggestion').offsetHeight;\n                    if (that.sc.suggestionHeight)\n                        if (!next) that.sc.scrollTop = 0;\n                        else {\n                            var scrTop = that.sc.scrollTop, selTop = next.getBoundingClientRect().top - that.sc.getBoundingClientRect().top;\n                            if (selTop + that.sc.suggestionHeight - that.sc.maxHeight > 0)\n                                that.sc.scrollTop = selTop + that.sc.suggestionHeight + scrTop - that.sc.maxHeight;\n                            else if (selTop < 0)\n                                that.sc.scrollTop = selTop + scrTop;\n                        }\n                }\n            }\n            addEvent(window, 'resize', that.updateSC);\n            document.body.appendChild(that.sc);\n\n            live('autocomplete-suggestion', 'mouseleave', function(e){\n                var sel = that.sc.querySelector('.autocomplete-suggestion.selected');\n                if (sel) setTimeout(function(){ sel.className = sel.className.replace('selected', ''); }, 20);\n            }, that.sc);\n\n            live('autocomplete-suggestion', 'mouseover', function(e){\n                var sel = that.sc.querySelector('.autocomplete-suggestion.selected');\n                if (sel) sel.className = sel.className.replace('selected', '');\n                this.className += ' selected';\n            }, that.sc);\n\n            live('autocomplete-suggestion', 'mousedown', function(e){\n                if (hasClass(this, 'autocomplete-suggestion')) { // else outside click\n                    var v = this.getAttribute('data-val');\n                    that.value = v;\n                    o.onSelect(e, v, this);\n                    that.sc.style.display = 'none';\n                }\n            }, that.sc);\n\n            that.blurHandler = function(){\n                try { var over_sb = document.querySelector('.autocomplete-suggestions:hover'); } catch(e){ var over_sb = 0; }\n                if (!over_sb) {\n                    that.last_val = that.value;\n                    that.sc.style.display = 'none';\n                    setTimeout(function(){ that.sc.style.display = 'none'; }, 350); // hide suggestions on fast input\n                } else if (that !== document.activeElement) setTimeout(function(){ that.focus(); }, 20);\n            };\n            addEvent(that, 'blur', that.blurHandler);\n\n            var suggest = function(data){\n                var val = that.value;\n                that.cache[val] = data;\n                if (data.length && val.length >= o.minChars) {\n                    var s = '';\n                    for (var i=0;i<data.length;i++) s += o.renderItem(data[i], val);\n                    that.sc.innerHTML = s;\n                    that.updateSC(0);\n                }\n                else\n                    that.sc.style.display = 'none';\n            }\n\n            that.keydownHandler = function(e){\n                var key = window.event ? e.keyCode : e.which;\n                // down (40), up (38)\n                if ((key == 40 || key == 38) && that.sc.innerHTML) {\n                    var next, sel = that.sc.querySelector('.autocomplete-suggestion.selected');\n                    if (!sel) {\n                        next = (key == 40) ? that.sc.querySelector('.autocomplete-suggestion') : that.sc.childNodes[that.sc.childNodes.length - 1]; // first : last\n                        next.className += ' selected';\n                        that.value = next.getAttribute('data-val');\n                    } else {\n                        next = (key == 40) ? sel.nextSibling : sel.previousSibling;\n                        if (next) {\n                            sel.className = sel.className.replace('selected', '');\n                            next.className += ' selected';\n                            that.value = next.getAttribute('data-val');\n                        }\n                        else { sel.className = sel.className.replace('selected', ''); that.value = that.last_val; next = 0; }\n                    }\n                    that.updateSC(0, next);\n                    return false;\n                }\n                // esc\n                else if (key == 27) { that.value = that.last_val; that.sc.style.display = 'none'; }\n                // enter\n                else if (key == 13 || key == 9) {\n                    var sel = that.sc.querySelector('.autocomplete-suggestion.selected');\n                    if (sel && that.sc.style.display != 'none') { o.onSelect(e, sel.getAttribute('data-val'), sel); setTimeout(function(){ that.sc.style.display = 'none'; }, 20); }\n                }\n            };\n            addEvent(that, 'keydown', that.keydownHandler);\n\n            that.keyupHandler = function(e){\n                var key = window.event ? e.keyCode : e.which;\n                if (!key || (key < 35 || key > 40) && key != 13 && key != 27) {\n                    var val = that.value;\n                    if (val.length >= o.minChars) {\n                        if (val != that.last_val) {\n                            that.last_val = val;\n                            clearTimeout(that.timer);\n                            if (o.cache) {\n                                if (val in that.cache) { suggest(that.cache[val]); return; }\n                                // no requests if previous suggestions were empty\n                                for (var i=1; i<val.length-o.minChars; i++) {\n                                    var part = val.slice(0, val.length-i);\n                                    if (part in that.cache && !that.cache[part].length) { suggest([]); return; }\n                                }\n                            }\n                            that.timer = setTimeout(function(){ o.source(val, suggest) }, o.delay);\n                        }\n                    } else {\n                        that.last_val = val;\n                        that.sc.style.display = 'none';\n                    }\n                }\n            };\n            addEvent(that, 'keyup', that.keyupHandler);\n\n            that.focusHandler = function(e){\n                that.last_val = '\\n';\n                that.keyupHandler(e)\n            };\n            if (!o.minChars) addEvent(that, 'focus', that.focusHandler);\n        }\n\n        // public destroy method\n        this.destroy = function(){\n            for (var i=0; i<elems.length; i++) {\n                var that = elems[i];\n                removeEvent(window, 'resize', that.updateSC);\n                removeEvent(that, 'blur', that.blurHandler);\n                removeEvent(that, 'focus', that.focusHandler);\n                removeEvent(that, 'keydown', that.keydownHandler);\n                removeEvent(that, 'keyup', that.keyupHandler);\n                if (that.autocompleteAttr)\n                    that.setAttribute('autocomplete', that.autocompleteAttr);\n                else\n                    that.removeAttribute('autocomplete');\n                document.body.removeChild(that.sc);\n                that = null;\n            }\n        };\n    }\n    return autoComplete;\n})();\n\n(function(){\n    if (true)\n        !(__WEBPACK_AMD_DEFINE_RESULT__ = (function () { return autoComplete; }).call(exports, __webpack_require__, exports, module),\n\t\t\t\t__WEBPACK_AMD_DEFINE_RESULT__ !== undefined && (module.exports = __WEBPACK_AMD_DEFINE_RESULT__));\n    else {}\n})();\n\n\n//# sourceURL=webpack:///./js/auto-complete.js?");

/***/ }),

/***/ "./node_modules/css-loader/dist/cjs.js!./css/auto-complete.css":
/*!*********************************************************************!*\
  !*** ./node_modules/css-loader/dist/cjs.js!./css/auto-complete.css ***!
  \*********************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// Imports\nvar ___CSS_LOADER_API_IMPORT___ = __webpack_require__(/*! ../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\");\nexports = ___CSS_LOADER_API_IMPORT___(false);\n// Module\nexports.push([module.i, \".autocomplete-suggestions {\\n    text-align: left; cursor: default; border: 1px solid #ccc; border-top: 0; background: #fff; box-shadow: -1px 1px 3px rgba(0,0,0,.1);\\n\\n    /* core styles should not be changed */\\n    position: absolute; display: none; z-index: 9999; max-height: 254px; overflow: hidden; overflow-y: auto; box-sizing: border-box;\\n}\\n.autocomplete-suggestion { position: relative; padding: 0 .6em; line-height: 23px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; font-size: 1.02em; color: #333; }\\n.autocomplete-suggestion b { font-weight: normal; color: #1f8dd6; }\\n.autocomplete-suggestion.selected { background: #f0f0f0; }\\n\", \"\"]);\n// Exports\nmodule.exports = exports;\n\n\n//# sourceURL=webpack:///./css/auto-complete.css?./node_modules/css-loader/dist/cjs.js");

/***/ }),

/***/ "./node_modules/css-loader/dist/cjs.js!./css/styles.css":
/*!**************************************************************!*\
  !*** ./node_modules/css-loader/dist/cjs.js!./css/styles.css ***!
  \**************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("// Imports\nvar ___CSS_LOADER_API_IMPORT___ = __webpack_require__(/*! ../node_modules/css-loader/dist/runtime/api.js */ \"./node_modules/css-loader/dist/runtime/api.js\");\nexports = ___CSS_LOADER_API_IMPORT___(false);\n// Module\nexports.push([module.i, \"* {\\n    -webkit-box-sizing: border-box;\\n    -moz-box-sizing: border-box;\\n    box-sizing: border-box;\\n}\\n\\nbody {\\n    /* Disable scrollbars */\\n    overflow: hidden;\\n}\\n\\n/* Top bar css */\\n.home-menu {\\n    padding: 0.5em;\\n    text-align: center;\\n    box-shadow: 0 1px 1px rgba(0,0,0, 0.10);\\n}\\n.home-menu {\\n    background: #2d3e50;\\n    opacity: 0.5;\\n}\\n.pure-menu.pure-menu-fixed {\\n    /* Fixed menus normally have a border at the bottom. */\\n    border-bottom: none;\\n    /* I need a higher z-index here because of the scroll-over effect. */\\n    z-index: 4;\\n}\\n.home-menu img.pure-menu-heading {\\n    position: fixed;\\n    width: 200px;\\n    bottom: 0;\\n    left: 0;\\n    /*font-weight: 400;*/\\n\\n    /*font-size: 120%;*/\\n}\\n\\n.home-menu .pure-menu-selected a {\\n    color: white;\\n}\\n\\n.home-menu a {\\n    color: #6FBEF3;\\n}\\n.home-menu li a:hover,\\n.home-menu li a:focus {\\n    background: none;\\n    border: none;\\n    color: #AECFE5;\\n}\\n\\n.home-menu input {\\n    left: 50%;\\n}\\n\\n@media (min-width: 48em) {\\n\\n    /* We increase the body font size */\\n    body {\\n        font-size: 16px;\\n    }\\n\\n    /* We can align the menu header to the left, but float the\\n    menu items to the right. */\\n    .home-menu {\\n        text-align: left;\\n    }\\n    .home-menu ul {\\n        float: right;\\n    }\\n}\\n\\n/*\\n * -- DESKTOP (AND UP) MEDIA QUERIES --\\n * On desktops and other large devices, we want to over-ride some\\n * of the mobile and tablet styles.\\n */\\n@media (min-width: 78em) {\\n\\n}\\n/*html, body {\\n    width: 100%;\\n    height: 100%;\\n    margin: 0px;\\n    border: 0;\\n    \\n    overflow: hidden;\\n    // No floating content on sides\\n\\n}\\n\\n#canvas {\\n    position: relative;\\n    left: 0%;\\n    top: 0px;\\n    z-index: 1;\\n    width: 100%;\\n    height: 100%;\\n}\\n*/\\n/*\\n#labels_grid {\\n    position: absolute;\\n    left: 0px;\\n    top: 0px;\\n    z-index: 2;\\n    pointer-events: none;\\n}\\n\\n.ui {\\n    position: absolute;\\n    left: 0px;\\n    top: 0px;\\n    z-index: 10;\\n    width: 15%;\\n    height: 100%;\\n\\n    background-color: rgba(0, 0, 0, .5);\\n    color: white;\\n\\n    overflow-y: scroll;\\n    scrollbar-width: thin;\\n}\\n\\np {\\n    margin: 0;\\n    font: \\\"Arial\\\";\\n    font-size: 0.875em;\\n    font-style: normal;\\n}\\n\\nh2 {\\n    text-align: center;\\n}\\n\\n.container {\\n    border-top: 1px solid white;\\n    border-bottom: 1px solid white;\\n    margin: 10px 0px 10px 0px;\\n    padding: 5px 0px 5px 0px;\\n}\\n\\n#slider-handles {\\n    width: 100%;\\n    margin: 20px 0px 20px 0px;\\n}\\n.noUi-tooltip {\\n    display: none;\\n}\\n.noUi-active .noUi-tooltip {\\n    display: block;\\n}\\n\\n.location {\\n    display: inline-block;\\n    width: 47%;\\n}*/\", \"\"]);\n// Exports\nmodule.exports = exports;\n\n\n//# sourceURL=webpack:///./css/styles.css?./node_modules/css-loader/dist/cjs.js");

/***/ }),

/***/ "./node_modules/css-loader/dist/runtime/api.js":
/*!*****************************************************!*\
  !*** ./node_modules/css-loader/dist/runtime/api.js ***!
  \*****************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\n/*\n  MIT License http://www.opensource.org/licenses/mit-license.php\n  Author Tobias Koppers @sokra\n*/\n// css base code, injected by the css-loader\n// eslint-disable-next-line func-names\nmodule.exports = function (useSourceMap) {\n  var list = []; // return the list of modules as css string\n\n  list.toString = function toString() {\n    return this.map(function (item) {\n      var content = cssWithMappingToString(item, useSourceMap);\n\n      if (item[2]) {\n        return \"@media \".concat(item[2], \" {\").concat(content, \"}\");\n      }\n\n      return content;\n    }).join('');\n  }; // import a list of modules into the list\n  // eslint-disable-next-line func-names\n\n\n  list.i = function (modules, mediaQuery, dedupe) {\n    if (typeof modules === 'string') {\n      // eslint-disable-next-line no-param-reassign\n      modules = [[null, modules, '']];\n    }\n\n    var alreadyImportedModules = {};\n\n    if (dedupe) {\n      for (var i = 0; i < this.length; i++) {\n        // eslint-disable-next-line prefer-destructuring\n        var id = this[i][0];\n\n        if (id != null) {\n          alreadyImportedModules[id] = true;\n        }\n      }\n    }\n\n    for (var _i = 0; _i < modules.length; _i++) {\n      var item = [].concat(modules[_i]);\n\n      if (dedupe && alreadyImportedModules[item[0]]) {\n        // eslint-disable-next-line no-continue\n        continue;\n      }\n\n      if (mediaQuery) {\n        if (!item[2]) {\n          item[2] = mediaQuery;\n        } else {\n          item[2] = \"\".concat(mediaQuery, \" and \").concat(item[2]);\n        }\n      }\n\n      list.push(item);\n    }\n  };\n\n  return list;\n};\n\nfunction cssWithMappingToString(item, useSourceMap) {\n  var content = item[1] || ''; // eslint-disable-next-line prefer-destructuring\n\n  var cssMapping = item[3];\n\n  if (!cssMapping) {\n    return content;\n  }\n\n  if (useSourceMap && typeof btoa === 'function') {\n    var sourceMapping = toComment(cssMapping);\n    var sourceURLs = cssMapping.sources.map(function (source) {\n      return \"/*# sourceURL=\".concat(cssMapping.sourceRoot || '').concat(source, \" */\");\n    });\n    return [content].concat(sourceURLs).concat([sourceMapping]).join('\\n');\n  }\n\n  return [content].join('\\n');\n} // Adapted from convert-source-map (MIT)\n\n\nfunction toComment(sourceMap) {\n  // eslint-disable-next-line no-undef\n  var base64 = btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap))));\n  var data = \"sourceMappingURL=data:application/json;charset=utf-8;base64,\".concat(base64);\n  return \"/*# \".concat(data, \" */\");\n}\n\n//# sourceURL=webpack:///./node_modules/css-loader/dist/runtime/api.js?");

/***/ }),

/***/ "./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js":
/*!****************************************************************************!*\
  !*** ./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js ***!
  \****************************************************************************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
eval("\n\nvar isOldIE = function isOldIE() {\n  var memo;\n  return function memorize() {\n    if (typeof memo === 'undefined') {\n      // Test for IE <= 9 as proposed by Browserhacks\n      // @see http://browserhacks.com/#hack-e71d8692f65334173fee715c222cb805\n      // Tests for existence of standard globals is to allow style-loader\n      // to operate correctly into non-standard environments\n      // @see https://github.com/webpack-contrib/style-loader/issues/177\n      memo = Boolean(window && document && document.all && !window.atob);\n    }\n\n    return memo;\n  };\n}();\n\nvar getTarget = function getTarget() {\n  var memo = {};\n  return function memorize(target) {\n    if (typeof memo[target] === 'undefined') {\n      var styleTarget = document.querySelector(target); // Special case to return head of iframe instead of iframe itself\n\n      if (window.HTMLIFrameElement && styleTarget instanceof window.HTMLIFrameElement) {\n        try {\n          // This will throw an exception if access to iframe is blocked\n          // due to cross-origin restrictions\n          styleTarget = styleTarget.contentDocument.head;\n        } catch (e) {\n          // istanbul ignore next\n          styleTarget = null;\n        }\n      }\n\n      memo[target] = styleTarget;\n    }\n\n    return memo[target];\n  };\n}();\n\nvar stylesInDom = [];\n\nfunction getIndexByIdentifier(identifier) {\n  var result = -1;\n\n  for (var i = 0; i < stylesInDom.length; i++) {\n    if (stylesInDom[i].identifier === identifier) {\n      result = i;\n      break;\n    }\n  }\n\n  return result;\n}\n\nfunction modulesToDom(list, options) {\n  var idCountMap = {};\n  var identifiers = [];\n\n  for (var i = 0; i < list.length; i++) {\n    var item = list[i];\n    var id = options.base ? item[0] + options.base : item[0];\n    var count = idCountMap[id] || 0;\n    var identifier = \"\".concat(id, \" \").concat(count);\n    idCountMap[id] = count + 1;\n    var index = getIndexByIdentifier(identifier);\n    var obj = {\n      css: item[1],\n      media: item[2],\n      sourceMap: item[3]\n    };\n\n    if (index !== -1) {\n      stylesInDom[index].references++;\n      stylesInDom[index].updater(obj);\n    } else {\n      stylesInDom.push({\n        identifier: identifier,\n        updater: addStyle(obj, options),\n        references: 1\n      });\n    }\n\n    identifiers.push(identifier);\n  }\n\n  return identifiers;\n}\n\nfunction insertStyleElement(options) {\n  var style = document.createElement('style');\n  var attributes = options.attributes || {};\n\n  if (typeof attributes.nonce === 'undefined') {\n    var nonce =  true ? __webpack_require__.nc : undefined;\n\n    if (nonce) {\n      attributes.nonce = nonce;\n    }\n  }\n\n  Object.keys(attributes).forEach(function (key) {\n    style.setAttribute(key, attributes[key]);\n  });\n\n  if (typeof options.insert === 'function') {\n    options.insert(style);\n  } else {\n    var target = getTarget(options.insert || 'head');\n\n    if (!target) {\n      throw new Error(\"Couldn't find a style target. This probably means that the value for the 'insert' parameter is invalid.\");\n    }\n\n    target.appendChild(style);\n  }\n\n  return style;\n}\n\nfunction removeStyleElement(style) {\n  // istanbul ignore if\n  if (style.parentNode === null) {\n    return false;\n  }\n\n  style.parentNode.removeChild(style);\n}\n/* istanbul ignore next  */\n\n\nvar replaceText = function replaceText() {\n  var textStore = [];\n  return function replace(index, replacement) {\n    textStore[index] = replacement;\n    return textStore.filter(Boolean).join('\\n');\n  };\n}();\n\nfunction applyToSingletonTag(style, index, remove, obj) {\n  var css = remove ? '' : obj.media ? \"@media \".concat(obj.media, \" {\").concat(obj.css, \"}\") : obj.css; // For old IE\n\n  /* istanbul ignore if  */\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = replaceText(index, css);\n  } else {\n    var cssNode = document.createTextNode(css);\n    var childNodes = style.childNodes;\n\n    if (childNodes[index]) {\n      style.removeChild(childNodes[index]);\n    }\n\n    if (childNodes.length) {\n      style.insertBefore(cssNode, childNodes[index]);\n    } else {\n      style.appendChild(cssNode);\n    }\n  }\n}\n\nfunction applyToTag(style, options, obj) {\n  var css = obj.css;\n  var media = obj.media;\n  var sourceMap = obj.sourceMap;\n\n  if (media) {\n    style.setAttribute('media', media);\n  } else {\n    style.removeAttribute('media');\n  }\n\n  if (sourceMap && btoa) {\n    css += \"\\n/*# sourceMappingURL=data:application/json;base64,\".concat(btoa(unescape(encodeURIComponent(JSON.stringify(sourceMap)))), \" */\");\n  } // For old IE\n\n  /* istanbul ignore if  */\n\n\n  if (style.styleSheet) {\n    style.styleSheet.cssText = css;\n  } else {\n    while (style.firstChild) {\n      style.removeChild(style.firstChild);\n    }\n\n    style.appendChild(document.createTextNode(css));\n  }\n}\n\nvar singleton = null;\nvar singletonCounter = 0;\n\nfunction addStyle(obj, options) {\n  var style;\n  var update;\n  var remove;\n\n  if (options.singleton) {\n    var styleIndex = singletonCounter++;\n    style = singleton || (singleton = insertStyleElement(options));\n    update = applyToSingletonTag.bind(null, style, styleIndex, false);\n    remove = applyToSingletonTag.bind(null, style, styleIndex, true);\n  } else {\n    style = insertStyleElement(options);\n    update = applyToTag.bind(null, style, options);\n\n    remove = function remove() {\n      removeStyleElement(style);\n    };\n  }\n\n  update(obj);\n  return function updateStyle(newObj) {\n    if (newObj) {\n      if (newObj.css === obj.css && newObj.media === obj.media && newObj.sourceMap === obj.sourceMap) {\n        return;\n      }\n\n      update(obj = newObj);\n    } else {\n      remove();\n    }\n  };\n}\n\nmodule.exports = function (list, options) {\n  options = options || {}; // Force single-tag solution on IE6-9, which has a hard limit on the # of <style>\n  // tags it will allow on a page\n\n  if (!options.singleton && typeof options.singleton !== 'boolean') {\n    options.singleton = isOldIE();\n  }\n\n  list = list || [];\n  var lastIdentifiers = modulesToDom(list, options);\n  return function update(newList) {\n    newList = newList || [];\n\n    if (Object.prototype.toString.call(newList) !== '[object Array]') {\n      return;\n    }\n\n    for (var i = 0; i < lastIdentifiers.length; i++) {\n      var identifier = lastIdentifiers[i];\n      var index = getIndexByIdentifier(identifier);\n      stylesInDom[index].references--;\n    }\n\n    var newLastIdentifiers = modulesToDom(newList, options);\n\n    for (var _i = 0; _i < lastIdentifiers.length; _i++) {\n      var _identifier = lastIdentifiers[_i];\n\n      var _index = getIndexByIdentifier(_identifier);\n\n      if (stylesInDom[_index].references === 0) {\n        stylesInDom[_index].updater();\n\n        stylesInDom.splice(_index, 1);\n      }\n    }\n\n    lastIdentifiers = newLastIdentifiers;\n  };\n};\n\n//# sourceURL=webpack:///./node_modules/style-loader/dist/runtime/injectStylesIntoStyleTag.js?");

/***/ }),

/***/ "./shader.js":
/*!*******************!*\
  !*** ./shader.js ***!
  \*******************/
/*! exports provided: load_shaders */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"load_shaders\", function() { return load_shaders; });\n/* harmony import */ var _src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./src/shaders/catalogs/aitoff.vert */ \"./src/shaders/catalogs/aitoff.vert\");\n/* harmony import */ var _src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0__);\n/* harmony import */ var _src_shaders_catalogs_aitoff_frag__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./src/shaders/catalogs/aitoff.frag */ \"./src/shaders/catalogs/aitoff.frag\");\n/* harmony import */ var _src_shaders_catalogs_aitoff_frag__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_aitoff_frag__WEBPACK_IMPORTED_MODULE_1__);\n/* harmony import */ var _src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./src/shaders/catalogs/mercator.vert */ \"./src/shaders/catalogs/mercator.vert\");\n/* harmony import */ var _src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_2__);\n/* harmony import */ var _src_shaders_catalogs_mercator_frag__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./src/shaders/catalogs/mercator.frag */ \"./src/shaders/catalogs/mercator.frag\");\n/* harmony import */ var _src_shaders_catalogs_mercator_frag__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_mercator_frag__WEBPACK_IMPORTED_MODULE_3__);\n/* harmony import */ var _src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./src/shaders/catalogs/mollweide.vert */ \"./src/shaders/catalogs/mollweide.vert\");\n/* harmony import */ var _src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4__);\n/* harmony import */ var _src_shaders_catalogs_mollweide_frag__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./src/shaders/catalogs/mollweide.frag */ \"./src/shaders/catalogs/mollweide.frag\");\n/* harmony import */ var _src_shaders_catalogs_mollweide_frag__WEBPACK_IMPORTED_MODULE_5___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_mollweide_frag__WEBPACK_IMPORTED_MODULE_5__);\n/* harmony import */ var _src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./src/shaders/catalogs/ortho.vert */ \"./src/shaders/catalogs/ortho.vert\");\n/* harmony import */ var _src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_6___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_6__);\n/* harmony import */ var _src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./src/shaders/catalogs/ortho.frag */ \"./src/shaders/catalogs/ortho.frag\");\n/* harmony import */ var _src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_7___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_7__);\n/* harmony import */ var _src_shaders_colormaps_blackwhite_vert__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ./src/shaders/colormaps/blackwhite.vert */ \"./src/shaders/colormaps/blackwhite.vert\");\n/* harmony import */ var _src_shaders_colormaps_blackwhite_vert__WEBPACK_IMPORTED_MODULE_8___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_blackwhite_vert__WEBPACK_IMPORTED_MODULE_8__);\n/* harmony import */ var _src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ./src/shaders/colormaps/blackwhite.frag */ \"./src/shaders/colormaps/blackwhite.frag\");\n/* harmony import */ var _src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9__);\n/* harmony import */ var _src_shaders_colormaps_BluePastelRed_vert__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! ./src/shaders/colormaps/BluePastelRed.vert */ \"./src/shaders/colormaps/BluePastelRed.vert\");\n/* harmony import */ var _src_shaders_colormaps_BluePastelRed_vert__WEBPACK_IMPORTED_MODULE_10___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_BluePastelRed_vert__WEBPACK_IMPORTED_MODULE_10__);\n/* harmony import */ var _src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_11__ = __webpack_require__(/*! ./src/shaders/colormaps/BluePastelRed.frag */ \"./src/shaders/colormaps/BluePastelRed.frag\");\n/* harmony import */ var _src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_11___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_11__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_BrBG_vert__WEBPACK_IMPORTED_MODULE_12__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_BrBG.vert */ \"./src/shaders/colormaps/IDL_CB_BrBG.vert\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_BrBG_vert__WEBPACK_IMPORTED_MODULE_12___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_BrBG_vert__WEBPACK_IMPORTED_MODULE_12__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_13__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_BrBG.frag */ \"./src/shaders/colormaps/IDL_CB_BrBG.frag\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_13___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_13__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_GnBu_vert__WEBPACK_IMPORTED_MODULE_14__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_GnBu.vert */ \"./src/shaders/colormaps/IDL_CB_GnBu.vert\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_GnBu_vert__WEBPACK_IMPORTED_MODULE_14___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_GnBu_vert__WEBPACK_IMPORTED_MODULE_14__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_15__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_GnBu.frag */ \"./src/shaders/colormaps/IDL_CB_GnBu.frag\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_15___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_15__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_YIGnBu_vert__WEBPACK_IMPORTED_MODULE_16__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_YIGnBu.vert */ \"./src/shaders/colormaps/IDL_CB_YIGnBu.vert\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_YIGnBu_vert__WEBPACK_IMPORTED_MODULE_16___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_YIGnBu_vert__WEBPACK_IMPORTED_MODULE_16__);\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_17__ = __webpack_require__(/*! ./src/shaders/colormaps/IDL_CB_YIGnBu.frag */ \"./src/shaders/colormaps/IDL_CB_YIGnBu.frag\");\n/* harmony import */ var _src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_17___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_17__);\n/* harmony import */ var _src_shaders_colormaps_red_vert__WEBPACK_IMPORTED_MODULE_18__ = __webpack_require__(/*! ./src/shaders/colormaps/red.vert */ \"./src/shaders/colormaps/red.vert\");\n/* harmony import */ var _src_shaders_colormaps_red_vert__WEBPACK_IMPORTED_MODULE_18___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_red_vert__WEBPACK_IMPORTED_MODULE_18__);\n/* harmony import */ var _src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_19__ = __webpack_require__(/*! ./src/shaders/colormaps/red.frag */ \"./src/shaders/colormaps/red.frag\");\n/* harmony import */ var _src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_19___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_19__);\n/* harmony import */ var _src_shaders_grid_aitoff_vert__WEBPACK_IMPORTED_MODULE_20__ = __webpack_require__(/*! ./src/shaders/grid/aitoff.vert */ \"./src/shaders/grid/aitoff.vert\");\n/* harmony import */ var _src_shaders_grid_aitoff_vert__WEBPACK_IMPORTED_MODULE_20___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_aitoff_vert__WEBPACK_IMPORTED_MODULE_20__);\n/* harmony import */ var _src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_21__ = __webpack_require__(/*! ./src/shaders/grid/aitoff.frag */ \"./src/shaders/grid/aitoff.frag\");\n/* harmony import */ var _src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_21___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_21__);\n/* harmony import */ var _src_shaders_grid_mollweide_vert__WEBPACK_IMPORTED_MODULE_22__ = __webpack_require__(/*! ./src/shaders/grid/mollweide.vert */ \"./src/shaders/grid/mollweide.vert\");\n/* harmony import */ var _src_shaders_grid_mollweide_vert__WEBPACK_IMPORTED_MODULE_22___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_mollweide_vert__WEBPACK_IMPORTED_MODULE_22__);\n/* harmony import */ var _src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_23__ = __webpack_require__(/*! ./src/shaders/grid/mollweide.frag */ \"./src/shaders/grid/mollweide.frag\");\n/* harmony import */ var _src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_23___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_23__);\n/* harmony import */ var _src_shaders_grid_ortho_vert__WEBPACK_IMPORTED_MODULE_24__ = __webpack_require__(/*! ./src/shaders/grid/ortho.vert */ \"./src/shaders/grid/ortho.vert\");\n/* harmony import */ var _src_shaders_grid_ortho_vert__WEBPACK_IMPORTED_MODULE_24___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_ortho_vert__WEBPACK_IMPORTED_MODULE_24__);\n/* harmony import */ var _src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_25__ = __webpack_require__(/*! ./src/shaders/grid/ortho.frag */ \"./src/shaders/grid/ortho.frag\");\n/* harmony import */ var _src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_25___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_25__);\n/* harmony import */ var _src_shaders_grid_mercator_vert__WEBPACK_IMPORTED_MODULE_26__ = __webpack_require__(/*! ./src/shaders/grid/mercator.vert */ \"./src/shaders/grid/mercator.vert\");\n/* harmony import */ var _src_shaders_grid_mercator_vert__WEBPACK_IMPORTED_MODULE_26___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_mercator_vert__WEBPACK_IMPORTED_MODULE_26__);\n/* harmony import */ var _src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_27__ = __webpack_require__(/*! ./src/shaders/grid/mercator.frag */ \"./src/shaders/grid/mercator.frag\");\n/* harmony import */ var _src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_27___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_27__);\n/* harmony import */ var _src_shaders_hips_raytracer_vert__WEBPACK_IMPORTED_MODULE_28__ = __webpack_require__(/*! ./src/shaders/hips/raytracer.vert */ \"./src/shaders/hips/raytracer.vert\");\n/* harmony import */ var _src_shaders_hips_raytracer_vert__WEBPACK_IMPORTED_MODULE_28___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_raytracer_vert__WEBPACK_IMPORTED_MODULE_28__);\n/* harmony import */ var _src_shaders_hips_raytracer_frag__WEBPACK_IMPORTED_MODULE_29__ = __webpack_require__(/*! ./src/shaders/hips/raytracer.frag */ \"./src/shaders/hips/raytracer.frag\");\n/* harmony import */ var _src_shaders_hips_raytracer_frag__WEBPACK_IMPORTED_MODULE_29___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_raytracer_frag__WEBPACK_IMPORTED_MODULE_29__);\n/* harmony import */ var _src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_30__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/aitoff.vert */ \"./src/shaders/hips/rasterizer/aitoff.vert\");\n/* harmony import */ var _src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_30___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_30__);\n/* harmony import */ var _src_shaders_hips_rasterizer_aitoff_frag__WEBPACK_IMPORTED_MODULE_31__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/aitoff.frag */ \"./src/shaders/hips/rasterizer/aitoff.frag\");\n/* harmony import */ var _src_shaders_hips_rasterizer_aitoff_frag__WEBPACK_IMPORTED_MODULE_31___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_aitoff_frag__WEBPACK_IMPORTED_MODULE_31__);\n/* harmony import */ var _src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_32__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/mollweide.vert */ \"./src/shaders/hips/rasterizer/mollweide.vert\");\n/* harmony import */ var _src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_32___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_32__);\n/* harmony import */ var _src_shaders_hips_rasterizer_mollweide_frag__WEBPACK_IMPORTED_MODULE_33__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/mollweide.frag */ \"./src/shaders/hips/rasterizer/mollweide.frag\");\n/* harmony import */ var _src_shaders_hips_rasterizer_mollweide_frag__WEBPACK_IMPORTED_MODULE_33___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_mollweide_frag__WEBPACK_IMPORTED_MODULE_33__);\n/* harmony import */ var _src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_34__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/ortho.vert */ \"./src/shaders/hips/rasterizer/ortho.vert\");\n/* harmony import */ var _src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_34___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_34__);\n/* harmony import */ var _src_shaders_hips_rasterizer_ortho_frag__WEBPACK_IMPORTED_MODULE_35__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/ortho.frag */ \"./src/shaders/hips/rasterizer/ortho.frag\");\n/* harmony import */ var _src_shaders_hips_rasterizer_ortho_frag__WEBPACK_IMPORTED_MODULE_35___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_ortho_frag__WEBPACK_IMPORTED_MODULE_35__);\n/* harmony import */ var _src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_36__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/mercator.vert */ \"./src/shaders/hips/rasterizer/mercator.vert\");\n/* harmony import */ var _src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_36___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_36__);\n/* harmony import */ var _src_shaders_hips_rasterizer_mercator_frag__WEBPACK_IMPORTED_MODULE_37__ = __webpack_require__(/*! ./src/shaders/hips/rasterizer/mercator.frag */ \"./src/shaders/hips/rasterizer/mercator.frag\");\n/* harmony import */ var _src_shaders_hips_rasterizer_mercator_frag__WEBPACK_IMPORTED_MODULE_37___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_hips_rasterizer_mercator_frag__WEBPACK_IMPORTED_MODULE_37__);\n/* harmony import */ var _src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_38__ = __webpack_require__(/*! ./src/shaders/misc/text.vert */ \"./src/shaders/misc/text.vert\");\n/* harmony import */ var _src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_38___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_38__);\n/* harmony import */ var _src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_39__ = __webpack_require__(/*! ./src/shaders/misc/text.frag */ \"./src/shaders/misc/text.frag\");\n/* harmony import */ var _src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_39___default = /*#__PURE__*/__webpack_require__.n(_src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_39__);\n/* Import all the shaders here*/ \n// Catalog shaders\n\n\n\n\n\n\n\n\n\n\n\n\n// Colormap shaders\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n// Grid shader\n\n\n\n\n\n\n\n\n\n\n\n// HiPS shaders\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n// Misc\n\n\n\nconst varToString = varObj => Object.keys(varObj)[0];\n\nlet shaders = [\n    // Catalog shaders\n    {\n        name: \"catalog_aitoff\",\n        vert: _src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0___default.a,\n        frag: _src_shaders_catalogs_aitoff_frag__WEBPACK_IMPORTED_MODULE_1___default.a,\n    },\n    {\n        name: \"catalog_mercator\",\n        vert: _src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_2___default.a,\n        frag: _src_shaders_catalogs_mercator_frag__WEBPACK_IMPORTED_MODULE_3___default.a,\n    },\n    {\n        name: \"catalog_mollweide\",\n        vert: _src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4___default.a,\n        frag: _src_shaders_catalogs_mollweide_frag__WEBPACK_IMPORTED_MODULE_5___default.a,\n    },\n    {\n        name: \"catalog_ortho\",\n        vert: _src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_6___default.a,\n        frag: _src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_7___default.a,\n    },\n\n    // Colormap shaders\n    {\n        name: \"black_white_linear\",\n        vert: _src_shaders_colormaps_blackwhite_vert__WEBPACK_IMPORTED_MODULE_8___default.a,\n        frag: _src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9___default.a,\n    },\n    {\n        name: \"BluePastelRed\",\n        vert: _src_shaders_colormaps_BluePastelRed_vert__WEBPACK_IMPORTED_MODULE_10___default.a,\n        frag: _src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_11___default.a,\n    },\n    {\n        name: \"IDL_CB_BrBG\",\n        vert: _src_shaders_colormaps_IDL_CB_BrBG_vert__WEBPACK_IMPORTED_MODULE_12___default.a,\n        frag: _src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_13___default.a,\n    },\n    {\n        name: \"IDL_CB_GnBu\",\n        vert: _src_shaders_colormaps_IDL_CB_GnBu_vert__WEBPACK_IMPORTED_MODULE_14___default.a,\n        frag: _src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_15___default.a,\n    },\n    {\n        name: \"IDL_CB_YIGnBu\",\n        vert: _src_shaders_colormaps_IDL_CB_YIGnBu_vert__WEBPACK_IMPORTED_MODULE_16___default.a,\n        frag: _src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_17___default.a,\n    },\n    {\n        name: \"red_temperature\",\n        vert: _src_shaders_colormaps_red_vert__WEBPACK_IMPORTED_MODULE_18___default.a,\n        frag: _src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_19___default.a,\n    },\n\n    // Grid shader\n    {\n        name: \"grid_aitoff\",\n        vert: _src_shaders_grid_aitoff_vert__WEBPACK_IMPORTED_MODULE_20___default.a,\n        frag: _src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_21___default.a,\n    },\n    {\n        name: \"grid_ortho\",\n        vert: _src_shaders_grid_ortho_vert__WEBPACK_IMPORTED_MODULE_24___default.a,\n        frag: _src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_25___default.a,\n    },\n    {\n        name: \"grid_mollweide\",\n        vert: _src_shaders_grid_mollweide_vert__WEBPACK_IMPORTED_MODULE_22___default.a,\n        frag: _src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_23___default.a,\n    },\n    {\n        name: \"grid_mercator\",\n        vert: _src_shaders_grid_mercator_vert__WEBPACK_IMPORTED_MODULE_26___default.a,\n        frag: _src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_27___default.a,\n    },\n    // HiPS shaders\n    {\n        name: \"raytracer\",\n        vert: _src_shaders_hips_raytracer_vert__WEBPACK_IMPORTED_MODULE_28___default.a,\n        frag: _src_shaders_hips_raytracer_frag__WEBPACK_IMPORTED_MODULE_29___default.a,\n    },\n    {\n        name: \"rasterizer_aitoff\",\n        vert: _src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_30___default.a,\n        frag: _src_shaders_hips_rasterizer_aitoff_frag__WEBPACK_IMPORTED_MODULE_31___default.a,\n    },\n    {\n        name: \"rasterizer_mercator\",\n        vert: _src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_36___default.a,\n        frag: _src_shaders_hips_rasterizer_mercator_frag__WEBPACK_IMPORTED_MODULE_37___default.a,\n    },\n    {\n        name: \"rasterizer_mollweide\",\n        vert: _src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_32___default.a,\n        frag: _src_shaders_hips_rasterizer_mollweide_frag__WEBPACK_IMPORTED_MODULE_33___default.a,\n    },\n    {\n        name: \"rasterizer_ortho\",\n        vert: _src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_34___default.a,\n        frag: _src_shaders_hips_rasterizer_ortho_frag__WEBPACK_IMPORTED_MODULE_35___default.a,\n    },\n\n    // Misc\n    {\n        name: \"text\",\n        vert: _src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_38___default.a,\n        frag: _src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_39___default.a,\n    },\n];\n\nfunction load_shaders(webgl) {\n    return shaders;\n}\n\n\n//# sourceURL=webpack:///./shader.js?");

/***/ }),

/***/ "./src/shaders/catalogs/aitoff.frag":
/*!******************************************!*\
  !*** ./src/shaders/catalogs/aitoff.frag ***!
  \******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\n\\nout vec4 color;\\n\\nuniform sampler2D kernel_texture;\\nuniform float max_density;\\n\\nvoid main() {\\n    color += texture(kernel_texture, out_uv) / log2(max_density + 1.0);\\n\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/aitoff.frag?");

/***/ }),

/***/ "./src/shaders/catalogs/aitoff.vert":
/*!******************************************!*\
  !*** ./src/shaders/catalogs/aitoff.vert ***!
  \******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nlayout (location = 0) in vec2 offset;\\nlayout (location = 1) in vec2 uv;\\n\\nlayout (location = 2) in vec3 center;\\nlayout (location = 3) in vec2 center_lonlat;\\n\\nuniform float current_time;\\nuniform mat4 model;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\nuniform vec2 kernel_size;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nout vec2 out_uv;\\nout vec3 out_p;\\n\\nvec2 world2clip_aitoff(vec3 p) {\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float theta_by_two = theta * 0.5f;\\n\\n    float alpha = acos(cos(delta)*cos(theta_by_two));\\n    float inv_sinc_alpha = 1.f;\\n    if (alpha > 1e-3f) {\\n        inv_sinc_alpha = alpha / sin(alpha);\\n    }\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    float x = -2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\\n    float y = inv_sinc_alpha * sin(delta);\\n\\n    return vec2(x / PI, y / PI);\\n}\\n\\nvoid main() {\\n    vec3 p = vec3(model * vec4(center, 1.0f));\\n    vec2 center_pos_clip_space = world2clip_aitoff(p);\\n\\n    vec2 pos_clip_space = center_pos_clip_space;\\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * clip_zoom_factor)) + offset * kernel_size , 0.f, 1.f);\\n\\n    out_uv = uv;\\n    out_p = p;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/aitoff.vert?");

/***/ }),

/***/ "./src/shaders/catalogs/mercator.frag":
/*!********************************************!*\
  !*** ./src/shaders/catalogs/mercator.frag ***!
  \********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\n\\nout vec4 color;\\n\\nuniform sampler2D kernel_texture;\\nuniform float max_density;\\n\\nvoid main() {\\n    color += texture(kernel_texture, out_uv) / log2(max_density + 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/mercator.frag?");

/***/ }),

/***/ "./src/shaders/catalogs/mercator.vert":
/*!********************************************!*\
  !*** ./src/shaders/catalogs/mercator.vert ***!
  \********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nlayout (location = 0) in vec2 offset;\\nlayout (location = 1) in vec2 uv;\\n\\nlayout (location = 2) in vec3 center;\\nlayout (location = 3) in vec2 center_lonlat;\\n\\nuniform float current_time;\\nuniform mat4 model;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\nuniform vec2 kernel_size;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nout vec2 out_uv;\\nout vec3 out_p;\\n\\nvec2 world2clip_mercator(vec3 p) {\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float x = -theta / PI;\\n    //float y = log(tan(PI * 0.25f + delta * 0.5f)) / PI;\\n    float y = asinh(tan(delta / PI));\\n\\n    return vec2(x, y);\\n}\\n\\nvoid main() {\\n    vec3 p = vec3(model * vec4(center, 1.0f));\\n    vec2 center_pos_clip_space = world2clip_mercator(p);\\n\\n    vec2 pos_clip_space = center_pos_clip_space;\\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * clip_zoom_factor)) + offset * kernel_size , 0.f, 1.f);\\n\\n    out_uv = uv;\\n    out_p = p;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/mercator.vert?");

/***/ }),

/***/ "./src/shaders/catalogs/mollweide.frag":
/*!*********************************************!*\
  !*** ./src/shaders/catalogs/mollweide.frag ***!
  \*********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\n\\nout vec4 color;\\n\\nuniform sampler2D kernel_texture;\\nuniform float max_density;\\n\\nvoid main() {\\n    color += texture(kernel_texture, out_uv) / log2(max_density + 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/mollweide.frag?");

/***/ }),

/***/ "./src/shaders/catalogs/mollweide.vert":
/*!*********************************************!*\
  !*** ./src/shaders/catalogs/mollweide.vert ***!
  \*********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nlayout (location = 0) in vec2 offset;\\nlayout (location = 1) in vec2 uv;\\n\\nlayout (location = 2) in vec3 center;\\nlayout (location = 3) in vec2 center_lonlat;\\n\\nuniform float current_time;\\nuniform mat4 model;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\nuniform vec2 kernel_size;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nout vec2 out_uv;\\nout vec3 out_p;\\n\\nvec2 world2clip_mollweide(vec3 p) {\\n    // X in [-1, 1]\\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\\n    int max_iter = 10;\\n\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float cst = PI * sin(delta);\\n\\n    float phi = delta;\\n    float f = phi + sin(phi) - cst;\\n\\n    int k = 0;\\n    while (abs(f) > 1e-4 && k < max_iter) {\\n        phi = phi - f / (1.f + cos(phi));\\n        f = phi + sin(phi) - cst;\\n\\n        k = k + 1;\\n    }\\n\\n    phi = phi * 0.5f;\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    float x = -(theta / PI) * cos(phi);\\n    float y = 0.5f * sin(phi);\\n\\n    return vec2(x, y);\\n}\\n\\nvoid main() {\\n    vec3 p = vec3(model * vec4(center, 1.0f));\\n    vec2 center_pos_clip_space = world2clip_mollweide(p);\\n\\n    vec2 pos_clip_space = center_pos_clip_space;\\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * clip_zoom_factor)) + offset * kernel_size , 0.f, 1.f);\\n\\n    out_uv = uv;\\n    out_p = p;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/mollweide.vert?");

/***/ }),

/***/ "./src/shaders/catalogs/ortho.frag":
/*!*****************************************!*\
  !*** ./src/shaders/catalogs/ortho.frag ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nin vec3 out_p;\\n\\nout vec4 color;\\n\\nuniform sampler2D kernel_texture;\\nuniform float max_density; // max number of sources in a kernel sized HEALPix cell at the current depth\\n\\nvoid main() {\\n    if (out_p.z < 0.f) {\\n        discard;\\n    }\\n\\n    color += texture(kernel_texture, out_uv) / log2(max_density + 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/ortho.frag?");

/***/ }),

/***/ "./src/shaders/catalogs/ortho.vert":
/*!*****************************************!*\
  !*** ./src/shaders/catalogs/ortho.vert ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nlayout (location = 0) in vec2 offset;\\nlayout (location = 1) in vec2 uv;\\n\\nlayout (location = 2) in vec3 center;\\nlayout (location = 3) in vec2 center_lonlat;\\n\\n\\nuniform float current_time;\\nuniform mat4 model;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\nuniform vec2 kernel_size;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nout vec2 out_uv;\\nout vec3 out_p;\\n\\nvec2 world2clip_orthographic(vec3 p) {\\n    return vec2(-p.x, p.y);\\n}\\n\\nvoid main() {\\n    vec3 p = vec3(model * vec4(center, 1.0f));\\n    vec2 center_pos_clip_space = world2clip_orthographic(p);\\n\\n    vec2 pos_clip_space = center_pos_clip_space;\\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * clip_zoom_factor)) + offset * kernel_size , 0.f, 1.f);\\n\\n    out_uv = uv;\\n    out_p = p;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/catalogs/ortho.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/BluePastelRed.frag":
/*!**************************************************!*\
  !*** ./src/shaders/colormaps/BluePastelRed.frag ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nfloat colormap_red(float x) {\\n    if (x < 0.1131206452846527) {\\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\\n    } else if (x < 0.5116005837917328) {\\n        return 0.0;\\n    } else if (x < 0.5705677568912506) {\\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\\n    } else if (x < 0.622047244) {\\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\\n    } else if (x < 0.7922459542751312) {\\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\\n    } else {\\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\\n    }\\n}\\n\\nfloat colormap_green(float x) {\\n    if (x < 0.114394336938858) {\\n        return 0.0;\\n    } else if (x < 0.4417250454425812) {\\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\\n    } else if (x < 0.4964917968308496) {\\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\\n    } else if (x < 0.6259051214039278) {\\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\\n    } else if (x < 0.8049814403057098) {\\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\\n    } else {\\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\\n    }\\n}\\n\\nfloat colormap_blue(float x) {\\n    if (x < 0.4424893036638088) {\\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\\n    } else if (x < 0.5) {\\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\\n    } else if (x < 0.5691165986930345) {\\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\\n    } else if (x < 0.6279306709766388) {\\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\\n    } else {\\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\\n    }\\n}\\n\\nvec4 colormap_f(float x) {\\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\\n    return vec4(r, g, b, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    /*if (opacity < 0.01f) {\\n        discard;\\n    }*/\\n\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/BluePastelRed.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/BluePastelRed.vert":
/*!**************************************************!*\
  !*** ./src/shaders/colormaps/BluePastelRed.vert ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/BluePastelRed.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_BrBG.frag":
/*!************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_BrBG.frag ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nfloat colormap_red(float x) {\\n    if (x < 0.4128910005092621) {\\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\\n    } else if (x < 0.5004365747118258) {\\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\\n    } else if (x < 0.6000321805477142) {\\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\\n    } else {\\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\\n    }\\n}\\n\\nfloat colormap_green(float x) {\\n    if (x < 0.3067105114459991) {\\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\\n    } else if (x < 0.4045854562297116) {\\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\\n    } else if (x < 0.5035906732082367) {\\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\\n    } else {\\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\\n    }\\n}\\n\\nfloat colormap_blue(float x) {\\n    if (x < 0.1012683545126085) {\\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\\n    } else if (x < 0.2050940692424774) {\\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\\n    } else if (x < 0.5022056996822357) {\\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\\n    } else if (x < 0.5970333516597748) {\\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\\n    } else {\\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\\n    }\\n}\\n\\nvec4 colormap_f(float x) {\\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\\n    return vec4(r, g, b, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    //color = texture(colormap, vec2(opacity, 0.5f));\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_BrBG.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_BrBG.vert":
/*!************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_BrBG.vert ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_BrBG.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_GnBu.frag":
/*!************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_GnBu.frag ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nfloat colormap_red(float x) {\\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\\n    if (v < 8.0) {\\n        return 8.0;\\n    } else {\\n        return v;\\n    }\\n}\\n\\nfloat colormap_green(float x) {\\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\\n}\\n\\nfloat colormap_blue(float x) {\\n    if (x < 0.3756393599187693) {\\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\\n    } else if (x < 0.6215448666633865) {\\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\\n    } else if (x < 0.8830064316178203) {\\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\\n    } else {\\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\\n    }\\n}\\n\\nvec4 colormap_f(float x) {\\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\\n    return vec4(r, g, b, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    //color = texture(colormap, vec2(opacity, 0.5f));\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_GnBu.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_GnBu.vert":
/*!************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_GnBu.vert ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_GnBu.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_YIGnBu.frag":
/*!**************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_YIGnBu.frag ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nfloat colormap_red(float x) {\\n    if (x < 0.2523055374622345) {\\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\\n    } else if (x < 0.6267540156841278) {\\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\\n    } else if (x < 0.8763731146612115) {\\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\\n    } else {\\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\\n    }\\n}\\n\\nfloat colormap_green(float x) {\\n    if (x < 0.4578040540218353) {\\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\\n    } else {\\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\\n    }\\n}\\n\\nfloat colormap_blue(float x) {\\n    if (x < 0.1239372193813324) {\\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\\n    } else if (x < 0.7535201013088226) {\\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\\n    } else {\\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\\n    }\\n}\\n\\nvec4 colormap_f(float x) {\\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\\n    return vec4(r, g, b, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    //color = texture(colormap, vec2(opacity, 0.5f));\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_YIGnBu.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/IDL_CB_YIGnBu.vert":
/*!**************************************************!*\
  !*** ./src/shaders/colormaps/IDL_CB_YIGnBu.vert ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/IDL_CB_YIGnBu.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/blackwhite.frag":
/*!***********************************************!*\
  !*** ./src/shaders/colormaps/blackwhite.frag ***!
  \***********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nvec4 colormap_f(float x) {\\n    float d = clamp(x, 0.0, 1.0);\\n    return vec4(d, d, d, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    //color = texture(colormap, vec2(opacity, 0.5f));\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/blackwhite.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/blackwhite.vert":
/*!***********************************************!*\
  !*** ./src/shaders/colormaps/blackwhite.vert ***!
  \***********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/blackwhite.vert?");

/***/ }),

/***/ "./src/shaders/colormaps/red.frag":
/*!****************************************!*\
  !*** ./src/shaders/colormaps/red.frag ***!
  \****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nin vec2 out_uv;\\nout vec4 color;\\n\\nuniform sampler2D texture_fbo;\\nuniform sampler2D colormap;\\nuniform float alpha;\\n\\nfloat colormap_red(float x) {\\n    return 1.448953446096850 * x - 5.02253539008443e-1;\\n}\\n\\nfloat colormap_green(float x) {\\n    return 1.889376646180860 * x - 2.272028094820020e2;\\n}\\n\\nfloat colormap_blue(float x) {\\n    return 3.92613636363636 * x - 7.46528409090909e+2;\\n}\\n\\nvec4 colormap_f(float x) {\\n    float t = x * 255.0;\\n    float r = clamp(colormap_red(t) / 255.0, 0.0, 1.0);\\n    float g = clamp(colormap_green(t) / 255.0, 0.0, 1.0);\\n    float b = clamp(colormap_blue(t) / 255.0, 0.0, 1.0);\\n    return vec4(r, g, b, 1.0);\\n}\\n\\nvoid main() {\\n    float opacity = texture(texture_fbo, out_uv).r;\\n\\n    float o = smoothstep(0.f, 0.1f, opacity);\\n\\n    //color = texture(colormap, vec2(opacity, 0.5f));\\n    color = colormap_f(opacity);\\n    color.a = alpha * o;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/red.frag?");

/***/ }),

/***/ "./src/shaders/colormaps/red.vert":
/*!****************************************!*\
  !*** ./src/shaders/colormaps/red.vert ***!
  \****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\nlayout (location = 1) in vec2 uv;\\n\\nout vec2 out_uv;\\n\\nvoid main() {\\n    gl_Position = vec4(position, 0.f, 1.f);\\n    out_uv = uv;\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/colormaps/red.vert?");

/***/ }),

/***/ "./src/shaders/grid/aitoff.frag":
/*!**************************************!*\
  !*** ./src/shaders/grid/aitoff.frag ***!
  \**************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nout vec4 color;\\nin vec2 pos_clip;\\n\\nuniform vec4 grid_color;\\nuniform mat4 world2model;\\nuniform mat4 model2world;\\nuniform float clip_zoom_factor;\\n\\nuniform float meridians[20];\\nuniform int num_meridians;\\nuniform float parallels[10];\\nuniform int num_parallels;\\n\\nuniform vec2 window_size;\\n\\nconst float PI = 3.141592653589793f;\\n\\nvec2 world2clip_aitoff(vec3 p) {\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float theta_by_two = theta * 0.5f;\\n\\n    float alpha = acos(cos(delta)*cos(theta_by_two));\\n    float inv_sinc_alpha = 1.f;\\n    if (alpha > 1e-3f) {\\n        inv_sinc_alpha = alpha / sin(alpha);\\n    }\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    float x = -2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\\n    float y = inv_sinc_alpha * sin(delta);\\n\\n    return vec2(x / PI, y / PI);\\n}\\n\\nbool is_included_inside_projection(vec2 pos_clip_space) {\\n    float px2 = pos_clip_space.x * pos_clip_space.x;\\n    float py2 = pos_clip_space.y * pos_clip_space.y;\\n\\n    return (px2 * 0.25 + py2) <= 0.25;\\n}\\n\\n/// View to world space transformation\\n/// \\n/// This returns a normalized vector along its first 3 dimensions.\\n/// Its fourth component is set to 1.\\n/// \\n/// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]\\n/// \\n/// # Arguments\\n/// \\n/// * `x` - in normalized device coordinates between [-1; 1]\\n/// * `y` - in normalized device coordinates between [-1; 1]\\nvec3 clip2world_aitoff(vec2 pos_clip_space) {\\n    if(!is_included_inside_projection(pos_clip_space)) {\\n        discard;\\n    }\\n\\n    vec2 uv = vec2(pos_clip_space.x * PI * 0.5, pos_clip_space.y * PI);\\n    //da uv a lat/lon\\n    float c = length(uv);\\n\\n    float phi = asin(uv.y * sin(c) / c);\\n    float theta = atan(uv.x * sin(c), c * cos(c)) * 2.0;\\n\\n    return vec3(\\n        -sin(theta) * cos(phi),\\n        sin(phi),\\n        cos(theta) * cos(phi)\\n    );\\n}\\n\\nfloat d_isolon(vec3 pos_model, float theta) {\\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\\n    // Discard the (theta + PI) meridian\\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\\n    if (dot(pos_model, e_xz) < 0.0) {\\n        return 1e3;\\n    }\\n\\n    float d = abs(dot(n, pos_model));\\n\\n    vec3 h_model = normalize(pos_model - n*d);\\n    vec3 h_world = vec3(model2world * vec4(h_model, 1.f));\\n\\n    // Project to screen x and h and compute the distance\\n    // between the two\\n    vec2 h_clip = world2clip_aitoff(h_world);\\n    \\n    return length(pos_clip - h_clip) * 2.0;\\n}\\nfloat d_isolat(vec3 pos_model, float delta) {\\n    float y = atan(pos_model.y, length(pos_model.xz));\\n    float d = abs(y - delta);\\n    return d;\\n}\\n\\nfloat grid_alpha(vec3 pos_model) {\\n    float v = 1e10;\\n    \\n    for (int i = 0; i < num_meridians; i++) {\\n        float a = d_isolon(pos_model, meridians[i]);\\n\\n        v = min(a, v);\\n    }\\n    \\n    for (int i = 0; i < num_parallels; i++) {\\n        float a = d_isolat(pos_model, parallels[i]);\\n\\n        v = min(a, v);\\n    }\\n\\n    float eps = 3.0 * clip_zoom_factor / window_size.x;\\n    return smoothstep(eps, 2.0*eps, v);\\n}\\n\\nvoid main() {\\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\\n\\n    vec3 pos_world = clip2world_aitoff(pos_clip);\\n    vec3 pos_model = vec3(world2model * vec4(pos_world, 1.f));\\n\\n    float alpha = grid_alpha(pos_model);\\n    color = mix(grid_color, transparency, alpha);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/aitoff.frag?");

/***/ }),

/***/ "./src/shaders/grid/aitoff.vert":
/*!**************************************!*\
  !*** ./src/shaders/grid/aitoff.vert ***!
  \**************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\n\\nout vec2 pos_clip;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n\\nvoid main() {\\n    pos_clip = position * (ndc_to_clip * clip_zoom_factor);\\n\\n    gl_Position = vec4(position, 0.0, 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/aitoff.vert?");

/***/ }),

/***/ "./src/shaders/grid/mercator.frag":
/*!****************************************!*\
  !*** ./src/shaders/grid/mercator.frag ***!
  \****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nout vec4 color;\\nin vec2 pos_clip;\\n\\nuniform vec4 grid_color;\\nuniform mat4 world2model;\\nuniform mat4 model2world;\\nuniform float clip_zoom_factor;\\n\\nuniform float meridians[20];\\nuniform int num_meridians;\\nuniform float parallels[10];\\nuniform int num_parallels;\\nuniform vec2 window_size;\\n\\nconst float PI = 3.141592653589793f;\\n\\nvec2 world2clip_mercator(vec3 p) {\\n    float theta = atan(p.x, p.z);\\n    float delta = asin(p.y);\\n\\n    float x = -theta / PI;\\n    float y = asinh(tan(delta / PI));\\n\\n    return vec2(x, y);\\n}\\n\\nvec3 clip2world_mercator(vec2 p) {\\n    float theta = p.x * PI;\\n    float delta = atan(sinh(p.y)) * PI;\\n\\n    return vec3(-sin(theta) * cos(delta), sin(delta), cos(theta) * cos(delta));\\n}\\n\\nfloat d_isolon(vec3 pos_model, float theta) {\\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\\n    // Discard the (theta + PI) meridian\\n    /*vec3 e_xz = vec3(-n.z, 0.0, n.x);\\n    if (dot(pos_model, e_xz) < 0.0) {\\n        return 1e3;\\n    }*/\\n\\n    float d = abs(dot(n, pos_model));\\n\\n    vec3 h_model = normalize(pos_model - n*d);\\n    vec3 h_world = vec3(model2world * vec4(h_model, 1.f));\\n\\n    // Project to screen x and h and compute the distance\\n    // between the two\\n    vec2 h_clip = world2clip_mercator(h_world);\\n    \\n    return length(pos_clip - h_clip) * 3.0;\\n}\\nfloat d_isolat(vec3 pos_model, float delta) {\\n    float y = atan(pos_model.y, length(pos_model.xz));\\n    float d = abs(y - delta);\\n    return d;\\n}\\n\\nfloat grid_alpha(vec3 pos_model) {\\n    float v = 1e10;\\n    \\n    for (int i = 0; i < num_meridians; i++) {\\n        float a = d_isolon(pos_model, meridians[i]);\\n\\n        v = min(a, v);\\n    }\\n    \\n    for (int i = 0; i < num_parallels; i++) {\\n        float a = d_isolat(pos_model, parallels[i]);\\n\\n        v = min(a, v);\\n    }\\n\\n    float eps = 3.0 * clip_zoom_factor / window_size.x;\\n    return smoothstep(eps, 2.0*eps, v);\\n}\\n\\nvoid main() {\\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\\n\\n    vec3 pos_world = clip2world_mercator(pos_clip);\\n    vec3 pos_model = vec3(world2model * vec4(pos_world, 1.f));\\n\\n    float alpha = grid_alpha(pos_model);\\n    color = mix(grid_color, transparency, alpha);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/mercator.frag?");

/***/ }),

/***/ "./src/shaders/grid/mercator.vert":
/*!****************************************!*\
  !*** ./src/shaders/grid/mercator.vert ***!
  \****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\n\\nout vec2 pos_clip;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n\\nvoid main() {\\n    pos_clip = position * (ndc_to_clip * clip_zoom_factor);\\n\\n    gl_Position = vec4(position, 0.0, 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/mercator.vert?");

/***/ }),

/***/ "./src/shaders/grid/mollweide.frag":
/*!*****************************************!*\
  !*** ./src/shaders/grid/mollweide.frag ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nout vec4 color;\\nin vec2 pos_clip;\\n\\nuniform vec4 grid_color;\\nuniform mat4 world2model;\\nuniform mat4 model2world;\\nuniform float clip_zoom_factor;\\n\\nuniform float meridians[20];\\nuniform int num_meridians;\\nuniform float parallels[10];\\nuniform int num_parallels;\\nuniform vec2 window_size;\\n\\nconst float PI = 3.141592653589793f;\\n\\n\\n/// World to screen space transformation\\n/// X is between [-1, 1]\\n/// Y is between [-0.5, 0.5]\\n/// \\n/// # Arguments\\n/// \\n/// * `pos_world_space` - Position in the world space. Must be a normalized vector\\nvec2 world2clip_mollweide(vec3 p) {\\n    float lat = asin(p.y);\\n    float lon = atan(p.x, p.z);\\n    // X in [-1, 1]\\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\\n    const float eps = 1e-3;\\n    const int max_iter = 10;\\n\\n    float cst = PI * sin(lat);\\n\\n    float theta = lat;\\n    float f = theta + sin(theta) - cst;\\n\\n    int k = 0;\\n    while (abs(f) > eps && k < max_iter) {\\n        theta = theta - f / (1.0 + cos(theta));\\n        f = theta + sin(theta) - cst;\\n\\n        k += 1;\\n    }\\n\\n    theta = theta * 0.5;\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    return vec2(\\n        -(lon / PI) * cos(theta),\\n        0.5 * sin(theta)\\n    );\\n}\\n\\nbool is_included_inside_projection(vec2 pos_clip_space) {\\n    float px2 = pos_clip_space.x * pos_clip_space.x;\\n    float py2 = pos_clip_space.y * pos_clip_space.y;\\n\\n    return (px2 * 0.25 + py2) <= 0.25;\\n}\\n\\n/// View to world space transformation\\n/// \\n/// This returns a normalized vector along its first 3 dimensions.\\n/// Its fourth component is set to 1.\\n/// \\n/// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]\\n/// \\n/// # Arguments\\n/// \\n/// * `x` - in normalized device coordinates between [-1; 1]\\n/// * `y` - in normalized device coordinates between [-1; 1]\\nvec3 clip2world_mollweide(vec2 pos_clip_space) {\\n    if (!is_included_inside_projection(pos_clip_space)) {\\n        discard;\\n    }\\n\\n    float y2 = pos_clip_space.y * pos_clip_space.y;\\n    float k = sqrt(1.0 - 4.0 * y2);\\n\\n    float theta = PI * pos_clip_space.x / k;\\n    float delta = asin((2.0 * asin(2.0 * pos_clip_space.y) + 4.0 * pos_clip_space.y * k) / PI);\\n    \\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    return vec3(-sin(theta) * cos(delta), sin(delta), cos(theta) * cos(delta));\\n}\\n\\nfloat d_isolon(vec3 pos_model, float theta) {\\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\\n    // Discard the (theta + PI) meridian\\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\\n    if (dot(pos_model, e_xz) < 0.0) {\\n        return 1e3;\\n    }\\n\\n    float d = abs(dot(n, pos_model));\\n\\n    vec3 h_model = normalize(pos_model - n*d);\\n    vec3 h_world = vec3(model2world * vec4(h_model, 1.f));\\n\\n    // Project to screen x and h and compute the distance\\n    // between the two\\n    vec2 h_clip = world2clip_mollweide(h_world);\\n    \\n    return length(pos_clip - h_clip) * 2.0;\\n}\\nfloat d_isolat(vec3 pos_model, float delta) {\\n    float y = atan(pos_model.y, length(pos_model.xz));\\n    float d = abs(y - delta);\\n    return d;\\n}\\n\\nfloat grid_alpha(vec3 pos_model) {\\n    float v = 1e10;\\n    \\n    for (int i = 0; i < num_meridians; i++) {\\n        float a = d_isolon(pos_model, meridians[i]);\\n\\n        v = min(a, v);\\n    }\\n    \\n    for (int i = 0; i < num_parallels; i++) {\\n        float a = d_isolat(pos_model, parallels[i]);\\n\\n        v = min(a, v);\\n    }\\n\\n    float eps = 3.0 * clip_zoom_factor / window_size.x;\\n    return smoothstep(eps, 2.0*eps, v);\\n}\\n\\nvoid main() {\\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\\n\\n    vec3 pos_world = clip2world_mollweide(pos_clip);\\n    vec3 pos_model = vec3(world2model * vec4(pos_world, 1.f));\\n\\n    float alpha = grid_alpha(pos_model);\\n    color = mix(grid_color, transparency, alpha);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/mollweide.frag?");

/***/ }),

/***/ "./src/shaders/grid/mollweide.vert":
/*!*****************************************!*\
  !*** ./src/shaders/grid/mollweide.vert ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\n\\nout vec2 pos_clip;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n\\nvoid main() {\\n    pos_clip = position * (ndc_to_clip * clip_zoom_factor);\\n\\n    gl_Position = vec4(position, 0.0, 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/mollweide.vert?");

/***/ }),

/***/ "./src/shaders/grid/ortho.frag":
/*!*************************************!*\
  !*** ./src/shaders/grid/ortho.frag ***!
  \*************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nout vec4 color;\\nin vec2 pos_clip;\\n\\nuniform vec4 grid_color;\\nuniform mat4 world2model;\\nuniform mat4 model2world;\\nuniform float clip_zoom_factor;\\n\\nuniform float meridians[20];\\nuniform int num_meridians;\\nuniform float parallels[10];\\nuniform int num_parallels;\\nuniform vec2 window_size;\\n\\nconst float PI = 3.141592653589793f;\\n\\nvec2 world2clip_orthographic(vec3 p) {\\n    return vec2(-p.x, p.y);\\n}\\n\\nvec3 clip2world_orthographic(vec2 pos_clip_space) {\\n    float z = 1.f - dot(pos_clip_space, pos_clip_space);\\n    if (z > 0.f) {\\n        return vec3(-pos_clip_space.x, pos_clip_space.y, sqrt(z));\\n    } else {\\n        discard;\\n    }\\n}\\n\\nfloat d_isolon(vec3 pos_model, float theta) {\\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\\n    // Discard the (theta + PI) meridian\\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\\n    if (dot(pos_model, e_xz) < 0.0) {\\n        return 1e3;\\n    }\\n\\n    float d = abs(dot(n, pos_model));\\n\\n    vec3 h_model = normalize(pos_model - n*d);\\n    vec3 h_world = vec3(model2world * vec4(h_model, 1.f));\\n\\n    // Project to screen x and h and compute the distance\\n    // between the two\\n    vec2 h_clip = world2clip_orthographic(h_world);\\n    \\n    return length(pos_clip - h_clip);\\n}\\nfloat d_isolat(vec3 pos_model, float delta) {\\n    float y = atan(pos_model.y, length(pos_model.xz));\\n    float d = abs(y - delta);\\n    return d;\\n}\\n\\nfloat grid_alpha(vec3 pos_model) {\\n    float v = 1e10;\\n    \\n    for (int i = 0; i < num_meridians; i++) {\\n        float a = d_isolon(pos_model, meridians[i]);\\n\\n        v = min(a, v);\\n    }\\n    \\n    for (int i = 0; i < num_parallels; i++) {\\n        float a = d_isolat(pos_model, parallels[i]);\\n\\n        v = min(a, v);\\n    }\\n\\n    float eps = clip_zoom_factor / window_size.x;\\n    return smoothstep(eps, 2.0*eps, v);\\n}\\n\\nvoid main() {\\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\\n\\n    vec3 pos_world = clip2world_orthographic(pos_clip);\\n    vec3 pos_model = vec3(world2model * vec4(pos_world, 1.f));\\n\\n    float alpha = grid_alpha(pos_model);\\n    color = mix(grid_color, transparency, alpha);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/ortho.frag?");

/***/ }),

/***/ "./src/shaders/grid/ortho.vert":
/*!*************************************!*\
  !*** ./src/shaders/grid/ortho.vert ***!
  \*************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\n\\nlayout (location = 0) in vec2 position;\\n\\nout vec2 pos_clip;\\n\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n\\nvoid main() {\\n    pos_clip = position * (ndc_to_clip * clip_zoom_factor);\\n\\n    gl_Position = vec4(position, 0.0, 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/grid/ortho.vert?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/aitoff.frag":
/*!*************************************************!*\
  !*** ./src/shaders/hips/rasterizer/aitoff.frag ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nin vec3 frag_uv_start;\\nin vec3 frag_uv_end;\\nin float frag_blending_factor;\\nin vec2 screen_pos;\\n\\nout vec4 out_frag_color;\\n\\nuniform sampler2DArray tex;\\n\\nvoid main() {\\n    vec4 color_start = vec4(0.f);\\n    color_start = texture(tex, frag_uv_start);\\n\\n    vec4 color_end = vec4(0.f);\\n    color_end = texture(tex, frag_uv_end);\\n\\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/aitoff.frag?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/aitoff.vert":
/*!*************************************************!*\
  !*** ./src/shaders/hips/rasterizer/aitoff.vert ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nlayout (location = 0) in vec2 lonlat;\\nlayout (location = 1) in vec3 position;\\nlayout (location = 2) in vec3 uv_start;\\nlayout (location = 3) in vec3 uv_end;\\nlayout (location = 4) in float time_tile_received;\\n\\nout vec3 frag_uv_start;\\nout vec3 frag_uv_end;\\nout float frag_blending_factor;\\nout vec2 screen_pos;\\n\\nuniform mat4 model;\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n// current time in ms\\nuniform float current_time;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nvec2 world2clip_aitoff(vec3 p) {\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float theta_by_two = theta * 0.5f;\\n\\n    float alpha = acos(cos(delta)*cos(theta_by_two));\\n    float inv_sinc_alpha = 1.f;\\n    if (alpha > 1e-3f) {\\n        inv_sinc_alpha = alpha / sin(alpha);\\n    }\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    float x = -2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\\n    float y = inv_sinc_alpha * sin(delta);\\n\\n    return vec2(x / PI, y / PI);\\n}\\n\\nvoid main() {\\n    vec3 world_pos = vec3(inverse(model) * vec4(position, 1.f));\\n    gl_Position = vec4(world2clip_aitoff(world_pos) / (ndc_to_clip * clip_zoom_factor), 0.0, 1.0);\\n\\n    screen_pos = gl_Position.xy;\\n    frag_uv_start = uv_start;\\n    frag_uv_end = uv_end;\\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/aitoff.vert?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/mercator.frag":
/*!***************************************************!*\
  !*** ./src/shaders/hips/rasterizer/mercator.frag ***!
  \***************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nin vec3 frag_uv_start;\\nin vec3 frag_uv_end;\\nin float frag_blending_factor;\\nin vec2 screen_pos;\\n\\nout vec4 out_frag_color;\\n\\nuniform sampler2DArray tex;\\n\\nvoid main() {\\n    vec4 color_start = vec4(0.f);\\n    color_start = texture(tex, frag_uv_start);\\n\\n    vec4 color_end = vec4(0.f);\\n    color_end = texture(tex, frag_uv_end);\\n\\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/mercator.frag?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/mercator.vert":
/*!***************************************************!*\
  !*** ./src/shaders/hips/rasterizer/mercator.vert ***!
  \***************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nlayout (location = 0) in vec2 lonlat;\\nlayout (location = 1) in vec3 position;\\nlayout (location = 2) in vec3 uv_start;\\nlayout (location = 3) in vec3 uv_end;\\nlayout (location = 4) in float time_tile_received;\\n\\nout vec3 frag_uv_start;\\nout vec3 frag_uv_end;\\nout float frag_blending_factor;\\nout vec2 screen_pos;\\n\\nuniform mat4 model;\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n// current time in ms\\nuniform float current_time;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nvec2 world2clip_mercator(vec3 p) {\\n    // X in [-1, 1]\\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\\n    int max_iter = 10;\\n\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float x = -theta / PI;\\n    float y = log(tan(PI * 0.25f + delta * 0.5f)) / PI;\\n\\n    return vec2(x, y);\\n}\\n\\nvoid main() {\\n    vec3 world_pos = vec3(inverse(model) * vec4(position, 1.f));\\n    gl_Position = vec4(world2clip_mercator(world_pos) / (ndc_to_clip * clip_zoom_factor), 0.0, 1.0);\\n\\n    screen_pos = gl_Position.xy;\\n    frag_uv_start = uv_start;\\n    frag_uv_end = uv_end;\\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/mercator.vert?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/mollweide.frag":
/*!****************************************************!*\
  !*** ./src/shaders/hips/rasterizer/mollweide.frag ***!
  \****************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nin vec3 frag_uv_start;\\nin vec3 frag_uv_end;\\nin float frag_blending_factor;\\nin vec2 screen_pos;\\n\\nout vec4 out_frag_color;\\n\\nuniform sampler2DArray tex;\\n\\nvoid main() {\\n    vec4 color_start = vec4(0.f);\\n    color_start = texture(tex, frag_uv_start);\\n\\n    vec4 color_end = vec4(0.f);\\n    color_end = texture(tex, frag_uv_end);\\n\\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/mollweide.frag?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/mollweide.vert":
/*!****************************************************!*\
  !*** ./src/shaders/hips/rasterizer/mollweide.vert ***!
  \****************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nlayout (location = 0) in vec2 lonlat;\\nlayout (location = 1) in vec3 position;\\nlayout (location = 2) in vec3 uv_start;\\nlayout (location = 3) in vec3 uv_end;\\nlayout (location = 4) in float time_tile_received;\\n\\nout vec3 frag_uv_start;\\nout vec3 frag_uv_end;\\nout float frag_blending_factor;\\nout vec2 screen_pos;\\n\\nuniform mat4 model;\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n// current time in ms\\nuniform float current_time;\\n\\nconst float PI = 3.1415926535897932384626433832795f;\\n\\nvec2 world2clip_mollweide(vec3 p) {\\n    // X in [-1, 1]\\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\\n    int max_iter = 10;\\n\\n    float delta = asin(p.y);\\n    float theta = atan(p.x, p.z);\\n\\n    float cst = PI * sin(delta);\\n\\n    float phi = delta;\\n    float f = phi + sin(phi) - cst;\\n\\n    int k = 0;\\n    while (abs(f) > 1e-4 && k < max_iter) {\\n        phi = phi - f / (1.f + cos(phi));\\n        f = phi + sin(phi) - cst;\\n\\n        k = k + 1;\\n    }\\n\\n    phi = phi * 0.5f;\\n\\n    // The minus is an astronomical convention.\\n    // longitudes are increasing from right to left\\n    float x = -(theta / PI) * cos(phi);\\n    float y = 0.5f * sin(phi);\\n\\n    return vec2(x, y);\\n}\\n\\nvoid main() {\\n    vec3 world_pos = vec3(inverse(model) * vec4(position, 1.f));\\n    gl_Position = vec4(world2clip_mollweide(world_pos) / (ndc_to_clip * clip_zoom_factor), 0.0, 1.0);\\n\\n    screen_pos = gl_Position.xy;\\n    frag_uv_start = uv_start;\\n    frag_uv_end = uv_end;\\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/mollweide.vert?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/ortho.frag":
/*!************************************************!*\
  !*** ./src/shaders/hips/rasterizer/ortho.frag ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nin vec3 frag_uv_start;\\nin vec3 frag_uv_end;\\nin float frag_blending_factor;\\n\\nout vec4 out_frag_color;\\n\\nuniform sampler2DArray tex;\\n\\nvoid main() {\\n    vec4 color_start = vec4(0.f);\\n    color_start = texture(tex, frag_uv_start);\\n\\n    vec4 color_end = vec4(0.f);\\n    color_end = texture(tex, frag_uv_end);\\n    //out_frag_color = mix(vec4(1.0, 0.0, 0.0, 1.0), color_end, frag_blending_factor);\\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/ortho.frag?");

/***/ }),

/***/ "./src/shaders/hips/rasterizer/ortho.vert":
/*!************************************************!*\
  !*** ./src/shaders/hips/rasterizer/ortho.vert ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nlayout (location = 0) in vec2 lonlat;\\nlayout (location = 1) in vec3 position;\\nlayout (location = 2) in vec3 uv_start;\\nlayout (location = 3) in vec3 uv_end;\\nlayout (location = 4) in float time_tile_received;\\n\\nout vec3 frag_uv_start;\\nout vec3 frag_uv_end;\\nout float frag_blending_factor;\\nout vec2 screen_pos;\\n\\nuniform mat4 model;\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n// current time in ms\\nuniform float current_time;\\n\\nvec2 world2screen_orthographic(vec3 p) {\\n    return vec2(-p.x, p.y);\\n}\\n\\nvoid main() {\\n    vec3 world_pos = vec3(inverse(model) * vec4(position, 1.f));\\n    gl_Position = vec4(world2screen_orthographic(world_pos) / (ndc_to_clip * clip_zoom_factor), 0.0, 1.0);\\n\\n    screen_pos = gl_Position.xy;\\n    frag_uv_start = uv_start;\\n    frag_uv_end = uv_end;\\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/rasterizer/ortho.vert?");

/***/ }),

/***/ "./src/shaders/hips/raytracer.frag":
/*!*****************************************!*\
  !*** ./src/shaders/hips/raytracer.frag ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision lowp sampler3D;\\nprecision lowp sampler2D;\\nprecision highp int;\\n\\nin vec3 out_vert_pos;\\nin vec2 pos_clip;\\n\\nout vec4 out_frag_color;\\n\\nconst float PI = 3.141592653589793f;\\nconst float FOUR_OVER_PI = 1.27323954474f;\\nconst float TRANSITION_Z = 0.66666666666f;\\nconst float TRANSITION_Z_INV = 1.5f;\\n\\nuint quarter(vec2 p) {\\n    uint x_neg = uint(p.x < 0.0f);\\n    uint y_neg = uint(p.y < 0.0f);\\n    uint q = (x_neg + y_neg) | (y_neg << 1U);\\n    return q;\\n}\\n\\nfloat xpm1(vec2 p) {\\n    bool x_neg = (p.x < 0.0f);\\n    //debug_assert!(x_neg <= 1);\\n    bool y_neg = (p.y < 0.0f);\\n    //debug_assert!(y_neg <= 1);\\n    // The purpose it to have the same numerical precision for each base cell\\n    // by avoiding subtraction by 1 or 3 or 5 or 7\\n    float lon = atan(abs(p.y), abs(p.x));\\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\\n    float x02 = lon * FOUR_OVER_PI;\\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\\n        return 1.0f - x02;\\n    } else {\\n        return x02 - 1.0f;\\n    }\\n}\\n\\nfloat one_minus_z_pos(vec3 p) {\\n    //debug_assert!(z > 0.0);\\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\\n\\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\\n    }\\n    return 1.0f - p.z;\\n}\\n\\nfloat one_minus_z_neg(vec3 p) {\\n    //debug_assert!(z < 0.0);\\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\\n        // 0.5 * d2 + 0.125 * d2 * d2\\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\\n    }\\n    return p.z + 1.0f;\\n}\\n\\n// Z-Order curve projection.\\nuint ij2z(uint i, uint j) {\\n    uint i1 = i | (j << 16U);\\n\\n    uint j1 = (i1 ^ (i1 >> 8U)) & 0x0000FF00U;\\n    uint i2 = i1 ^ j1 ^ (j1 << 8U);\\n\\n    uint j2 = (i2 ^ (i2 >> 4U)) & 0x00F000F0U;\\n    uint i3 = i2 ^ j2 ^ (j2 << 4U);\\n\\n    uint j3 = (i3 ^ (i3 >> 2U)) & 0x0C0C0C0CU;\\n    uint i4 = i3 ^ j3 ^ (j3 << 2U);\\n\\n    uint j4 = (i4 ^ (i4 >> 1U)) & 0x22222222U;\\n    uint i5 = i4 ^ j4 ^ (j4 << 1U);\\n\\n    return i5;\\n}\\n\\nstruct HashDxDy {\\n    uint idx;\\n    float dx;\\n    float dy;\\n};\\n\\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \\n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\\n// respect to the origin of the cell (South vertex).\\n// # Inputs:\\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\\n// - `x`: in `[-1.0, 1.0]`\\n// - `y`: in `[-1.0, 1.0]`\\n// - `z`: in `[-1.0, 1.0]`\\n// # Output\\n// - the cell number (hash value) associated with the given position on the unit sphere,\\n//   in `[0, 12*nside^2[`\\n// - `dx`: the positional offset $\\\\in [0, 1[$ along the south-to-east axis\\n// - `dy`: the positional offset $\\\\in [0, 1[$ along the south-to-west axis\\n// # WARNING\\n// - The function assumes, without checking, that the input vector is a unit vector \\n//   (hence `x^2 + y^2 + z^2 = 1`) !!\\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\\n    //assert!(depth <= 14);\\n    //assert!(-1.0 <= x && x <= 1.0);\\n    //assert!(-1.0 <= y && y <= 1.0);\\n    //assert!(-1.0 <= z && z <= 1.0);\\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\\n    // A f32 mantissa contains 23 bits.\\n    // - it basically means that when storing (x, y) coordinates,\\n    //   we can go as deep as depth 24 (or maybe 25)\\n    \\n    uint nside = 1U << uint(depth);\\n    float half_nside = float(nside) * 0.5f;\\n\\n    float x_pm1 = xpm1(p.xy);\\n    uint q = quarter(p.xy);\\n\\n    uint d0h = 0U;\\n    vec2 p_proj = vec2(0.f);\\n    if (p.z > TRANSITION_Z) {\\n        // North polar cap, Collignon projection.\\n        // - set the origin to (PI/4, 0)\\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\\n        d0h = q;\\n    } else if (p.z < -TRANSITION_Z) {\\n        // South polar cap, Collignon projection\\n        // - set the origin to (PI/4, -PI/2)\\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\\n        d0h = q + 8U;\\n    } else {\\n        // Equatorial region, Cylindrical equal area projection\\n        // - set the origin to (PI/4, 0)               if q = 2\\n        // - set the origin to (PI/4, -PI/2)           if q = 0\\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\\n        // let zero_or_one = (x_cea as u8) & 1;\\n        float y_pm1 = p.z * TRANSITION_Z_INV;\\n        // |\\\\2/|\\n        // .3X1.\\n        // |/0\\\\|\\n        uint q01 = uint(x_pm1 > y_pm1);  // 0/1\\n        //debug_assert!(q01 == 0 || q01 == 1);\\n        uint q12 = uint(x_pm1 >= -y_pm1); // 0\\\\1\\n        //debug_assert!(q12 == 0 || q12 == 1);\\n        uint q03 = 1U - q12; // 1\\\\0\\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\\n        uint q1 = q01 & q12; // = 1 if q1, 0 else\\n        //debug_assert!( q1 == 0 ||  q1 == 1);\\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \\n        //let y_proj = y_pm1 + (q01 + q03) as f32;\\n        p_proj = vec2(\\n            x_pm1 - float(int(q01 + q12) - 1),\\n            y_pm1 + float(q01 + q03)\\n        );\\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\\n        d0h = ((q01 + q03) << 2U) + ((q + q1) & 3U);\\n    }\\n\\n    // Coords inside the base cell\\n    float x = (half_nside * (p_proj.x + p_proj.y));\\n    float y = (half_nside * (p_proj.y - p_proj.x));\\n    uint i = uint(x);\\n    uint j = uint(y);\\n    // Deal with numerical inaccuracies, rare so branch miss-prediction negligible\\n    /*if (i == nside) {\\n        i = i - 1U;\\n    }\\n    // Deal with numerical inaccuracies, rare so branch miss-prediction negligible\\n    if (j == nside) {\\n        j = j - 1U;\\n    }*/\\n\\n    return HashDxDy(\\n        (d0h << (uint(depth) << 1U)) | ij2z(i, j),\\n        x - float(i),\\n        y - float(j)\\n    );\\n}\\n\\nuniform int user_action;\\n\\nstruct Tile {\\n    int uniq; // Healpix cell\\n    int texture_idx; // Index in the texture buffer\\n    float start_time; // Absolute time that the load has been done in ms\\n};\\n\\nuniform int current_depth;\\n\\nuniform sampler2DArray tex;\\nuniform Tile textures_tiles[64];\\n\\nuniform int num_textures;\\n\\nuniform float current_time; // current time in ms\\nstruct TileColor {\\n    Tile tile;\\n    vec3 color;\\n    bool found;\\n};\\n\\nTileColor get_tile_color(vec3 pos, int depth) {\\n    HashDxDy result = hash_with_dxdy(depth, pos.zxy);\\n    uint idx = result.idx;\\n    //int uniq = (1 << ((int(depth) + 1) << 1)) + int(idx);\\n    int uniq = (16 << (int(depth) << 1)) | int(idx);\\n\\n    vec2 uv = vec2(result.dy, result.dx);\\n\\n    int a = 0;\\n    int b = num_textures;\\n\\n    if (depth == 0) {\\n        b = 11;\\n    }\\n\\n    int i = (b + a) / 2;\\n\\n    int h = int(log2(float(b))) + 1;\\n    // Binary search among the tile idx\\n    for(int step = 0; step < h; step++) {\\n        if (uniq == textures_tiles[i].uniq) {\\n            Tile tile = textures_tiles[i];\\n\\n            int idx_texture = tile.texture_idx / 64;\\n            int off = tile.texture_idx % 64;\\n            float idx_row = float(off / 8); // in [0; 7]\\n            float idx_col = float(off % 8); // in [0; 7]\\n\\n            vec2 offset = (vec2(idx_col, idx_row) + uv)/8.f;\\n\\n            vec3 color = texture(tex, vec3(offset, float(idx_texture))).rgb;\\n\\n            return TileColor(tile, color, true);\\n        } else if (uniq < textures_tiles[i].uniq) {\\n            // go to left\\n            b = i - 1;\\n        } else {\\n            // go to right\\n            a = i + 1;\\n        }\\n        i = (a + b)/2;\\n    }\\n\\n    // code unreachable\\n    Tile empty = Tile(0, -1, current_time);\\n    return TileColor(empty, vec3(0.f), false);\\n}\\n\\nconst float duration = 500.f; // 500ms\\nuniform int max_depth; // max depth of the HiPS\\n\\nvoid main() {\\n    vec3 frag_pos = normalize(out_vert_pos);\\n    // Get the HEALPix cell idx and the uv in the texture\\n\\n    TileColor current_tile = get_tile_color(frag_pos, current_depth);\\n    out_frag_color = vec4(current_tile.color, 1.f);\\n\\n    if (!current_tile.found) {\\n        vec3 out_color = vec3(0.f);\\n        int depth = 0;\\n        if (user_action == 1) {\\n            // zoom\\n            depth = max(0, current_depth - 1);\\n        } else {\\n            // unzoom\\n            depth = min(max_depth, current_depth + 1);\\n        }\\n\\n        TileColor prev_tile = get_tile_color(frag_pos, depth);\\n        float alpha = clamp((current_time - prev_tile.tile.start_time) / duration, 0.f, 1.f);\\n        if (alpha == 1.f) {\\n            out_frag_color = vec4(prev_tile.color, 1.f);\\n            return;\\n        }\\n\\n        TileColor base_tile = get_tile_color(frag_pos, 0);\\n\\n        out_color = mix(base_tile.color, prev_tile.color, alpha);\\n        out_frag_color = vec4(out_color, 1.f);\\n        return;\\n    }\\n\\n    float alpha = clamp((current_time - current_tile.tile.start_time) / duration, 0.f, 1.f);\\n    \\n    // Little optimization: if the current tile is loaded since the time duration\\n    // then we do not need to evaluate the frag position for the previous/next depth\\n    if (alpha == 1.f) {\\n        out_frag_color = vec4(current_tile.color, 1.f);\\n        return;\\n    }\\n    vec3 out_color = vec3(0.f);\\n    int depth = 0;\\n    if (user_action == 1) {\\n        // zoom\\n        depth = max(0, current_depth - 1);\\n    } else if (user_action == 2) {\\n        // unzoom\\n        depth = min(max_depth, current_depth + 1);\\n    }\\n\\n    TileColor tile = get_tile_color(frag_pos, depth);\\n    if (!tile.found) {\\n        tile = get_tile_color(frag_pos, 0);\\n    }\\n\\n    out_color = mix(tile.color, current_tile.color, alpha);\\n    out_frag_color = vec4(out_color, 1.f);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/raytracer.frag?");

/***/ }),

/***/ "./src/shaders/hips/raytracer.vert":
/*!*****************************************!*\
  !*** ./src/shaders/hips/raytracer.vert ***!
  \*****************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision highp float;\\nprecision lowp sampler2DArray;\\nprecision highp int;\\n\\nlayout (location = 0) in vec2 pos_clip_space;\\nlayout (location = 1) in vec3 pos_world_space;\\n\\nout vec3 out_vert_pos;\\nout vec2 pos_clip;\\n\\nuniform mat4 model;\\nuniform vec2 ndc_to_clip;\\nuniform float clip_zoom_factor;\\n\\nvoid main() {\\n    gl_Position = vec4(pos_clip_space / (ndc_to_clip * clip_zoom_factor), 0.0, 1.0);\\n    pos_clip = pos_clip_space;\\n    out_vert_pos = vec3(model * vec4(pos_world_space, 1.f));\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/hips/raytracer.vert?");

/***/ }),

/***/ "./src/shaders/misc/text.frag":
/*!************************************!*\
  !*** ./src/shaders/misc/text.frag ***!
  \************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nprecision lowp sampler2DArray;\\n\\nuniform vec4 text_color;\\nuniform sampler2DArray font_textures;\\n\\nin vec3 out_uv;\\nout vec4 color;\\n\\nvoid main() {\\n    vec3 uv = vec3(out_uv.x, 1.f - out_uv.y, out_uv.z);\\n    vec4 mask = texture(font_textures, uv);\\n    color = text_color * mask;\\n    //color = vec4(1.0, 0.0, 0.0, 1.0);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/misc/text.frag?");

/***/ }),

/***/ "./src/shaders/misc/text.vert":
/*!************************************!*\
  !*** ./src/shaders/misc/text.vert ***!
  \************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = \"#version 300 es\\nprecision lowp float;\\nprecision lowp sampler2DArray;\\n\\nlayout (location = 0) in vec2 pos;\\nlayout (location = 1) in vec2 uv;\\n// Per instance attributes\\nlayout (location = 2) in vec2 center_letter;\\nlayout (location = 3) in vec2 size_letter;\\nlayout (location = 4) in vec2 pos_uv;\\nlayout (location = 5) in vec2 size_uv;\\nlayout (location = 6) in float idx_page;\\n\\nout vec3 out_uv;\\n\\nuniform vec2 window_size;\\nuniform float scaling;\\n\\nvec2 screen_to_ndc(vec2 p) {\\n    // Change of origin\\n    vec2 origin = p - window_size/2.0;\\n\\n    // Scale to fit in [-1, 1]\\n    return vec2(2.0 * (origin.x/window_size.x), -2.0 * (origin.y/window_size.y));\\n}\\n\\nvoid main() {\\n    vec2 ndc_pos = screen_to_ndc(center_letter + pos*32.0);\\n\\n    gl_Position = vec4(ndc_pos, 0.f, 1.f);\\n    out_uv = vec3(uv, idx_page);\\n}\"\n\n//# sourceURL=webpack:///./src/shaders/misc/text.vert?");

/***/ }),

/***/ "./widgets/catalog.js":
/*!****************************!*\
  !*** ./widgets/catalog.js ***!
  \****************************/
/*! exports provided: CatalogSelectorWidget */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"CatalogSelectorWidget\", function() { return CatalogSelectorWidget; });\n/* harmony import */ var _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../js/auto-complete.js */ \"./js/auto-complete.js\");\n/* harmony import */ var _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0__);\n\n\nasync function CatalogSelectorWidget(webClient) {\n    const url_catalogs = 'https://alasky.u-strasbg.fr/MocServer/query?expr=dataproduct_type%3Dcatalog%26%26nb_rows<%3D500000%26%26nb_rows>%3D50000%26%26data_ucd%3Dpos.parallax*%26%26data_ucd%3Dphot.mag*&get=record&fmt=json';\n    // Create our request constructor with all the parameters we need\n    var request = {\n        method: 'GET',\n        headers: new Headers(),\n        mode: 'cors',\n        cache: 'default'\n    };\n\n    let [catalogArray, catalogMap] = await fetch(url_catalogs, request)\n        .then(response => response.json())\n        .then((catalogs) => {\n            let catalogArray = [];\n            let catalogMap = new Map();\n            for (var k = 0; k < catalogs.length; k++) {\n                var cat_id = catalogs[k].ID;\n\n                catalogMap[cat_id] = {\n                    'obs_id': catalogs[k].obs_id,\n                };\n                catalogArray.push(cat_id);\n            }\n\n            return [catalogArray, catalogMap];\n        });\n\n    // Store the autoComplete widget\n    new _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0___default.a({\n        selector: '#catalog-selector',\n        minChars: 2,\n        source: function(term, suggest) {\n            term = term.toLowerCase();\n            var choices = catalogArray;\n            var matches = [];\n            for (let i=0; i<choices.length; i++) {\n                if (choices[i].toLowerCase().indexOf(term)>=0) {\n                    matches.push(choices[i]);\n                }\n            }\n            console.log(\"MATCHES\", matches)\n            suggest(matches);\n        },\n        renderItem: function (item, search) {\n            search = search.replace(/[-\\/\\\\^$*+?.()|[\\]{}]/g, '\\\\$&');\n            var re = new RegExp(\"(\" + search.split(' ').join('|') + \")\", \"gi\");\n            return '<div class=\"autocomplete-suggestion\" data-val=\"' + item + '\">' + item.replace(re, \"<b>$1</b>\") + '</div>';\n        },\n        onSelect: async function(e, term, item) {\n            let cat_id = term;\n\n            await loadCatalog(webClient, cat_id);\n        }\n    });\n}\n\nasync function loadCatalog(webClient, catalog_id) {\n    let parallax_column_name = await getTableColumnName(catalog_id, \"pos.parallax\");\n    console.log('parallax column name: ', parallax_column_name);\n    let phot_mag_column_name = await getTableColumnName(catalog_id, \"phot.mag\");\n    console.log('phot mag column name: ', phot_mag_column_name);\n\n    let pos_ra_column_name = await getTableColumnName(catalog_id, \"pos.eq.ra\");\n    let pos_dec_column_name = await getTableColumnName(catalog_id, \"pos.eq.dec\");\n\n    let table_obs_id = catalog_id.substring(4);\n    retrieveCatalog(table_obs_id, [pos_ra_column_name, pos_dec_column_name, phot_mag_column_name, parallax_column_name])\n        .then(sources => {\n            webClient.add_catalog(\"Test\", sources);\n        });\n}\n\nfunction getTableColumnName(table_name, ucd) {\n    let table_obs_id = table_name.substring(4);\n    let url = encodeURI('https://alasky.u-strasbg.fr/cgi/JSONProxy?url=') + encodeURIComponent('http://tapvizier.u-strasbg.fr/TAPVizieR/tap/sync?phase=RUN&lang=adql&format=json&request=doQuery&query=SELECT%20TOP%201%20table_name%2C%20column_name%2C%20ucd%20FROM%20TAP_SCHEMA.columns%20WHERE%20table_name%3D%27' + encodeURIComponent(table_obs_id) + '%27%20AND%20ucd%20LIKE%20%27' + encodeURIComponent(ucd) + '%25%27');\n    var request = {\n        method: 'GET',\n        headers: new Headers(),\n        mode: 'cors',\n        cache: 'default'\n    };\n    return fetch(url, request)\n        .then(response => response.json())\n        .then(table => {\n            // Return the column name of the first row corresponding to ucd\n            console.log(table_name, ucd, \": here are the data\", table.data);\n            return table.data[0][1];\n        });\n}\n\nfunction retrieveCatalog(table_obs_id, colnames, max_rows=\"*\") {\n    let cols = [];\n    colnames.forEach(col => {\n        console.log(col);\n        cols.push('\"' + table_obs_id + '\".\"' + encodeURIComponent(col) + '\"');\n    });\n\n    let cols_query = cols.join(\", \");\n\n    let sql_query = 'SELECT ' + cols_query + ' FROM \"' + table_obs_id + '\"';\n    console.log(sql_query);\n    \n    let url = encodeURI('https://alasky.u-strasbg.fr/cgi/JSONProxy?url=') + encodeURIComponent('http://tapvizier.u-strasbg.fr/TAPVizieR/tap/sync?phase=RUN&lang=adql&format=json&request=doQuery&query=' + encodeURIComponent(sql_query)) ;\n\n    var request = {\n        method: 'GET',\n        headers: new Headers(),\n        mode: 'cors',\n        cache: 'default'\n    };\n    return fetch(url, request)\n        .then(response => response.json())\n        .then((votable) => {\n            let sources = votable.data;\n            return sources;\n        });\n}\n\n\n//# sourceURL=webpack:///./widgets/catalog.js?");

/***/ }),

/***/ "./widgets/hips_selector.js":
/*!**********************************!*\
  !*** ./widgets/hips_selector.js ***!
  \**********************************/
/*! exports provided: HiPSSelectorWidget */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"HiPSSelectorWidget\", function() { return HiPSSelectorWidget; });\n/* harmony import */ var _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../js/auto-complete.js */ \"./js/auto-complete.js\");\n/* harmony import */ var _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0__);\n\n\nasync function HiPSSelectorWidget(webClient) {\n    const url = 'https://alasky.u-strasbg.fr/MocServer/query?hips_service_url*=*alasky*&&dataproduct_type=image&&hips_tile_format=*png*,*jpeg*&get=record&fmt=json';\n    // Create our request constructor with all the parameters we need\n    var request = {\n        method: 'GET',\n        headers: new Headers(),\n        mode: 'cors',\n        cache: 'default'\n    };\n    let [hipsesArray, hipsesMap] = await fetch(url, request)\n        .then((response) => response.json())\n        .then((hipsMetadata) =>  {\n            let hipsesArray = [];\n            let hipsesMap = new Map();\n            for(let k = 0; k < hipsMetadata.length; k++) {\n                let hips_id = hipsMetadata[k].ID;\n        \n                hipsesMap[hips_id] = {\n                    'hips_service_url': hipsMetadata[k].hips_service_url,\n                    'max_depth': hipsMetadata[k].hips_order,\n                    'tile_width': hipsMetadata[k].hips_tile_width,\n                    'hips_tile_format': hipsMetadata[k].hips_tile_format\n                };\n                hipsesArray.push(hips_id);\n            }\n\n            return [hipsesArray, hipsesMap];\n        });\n\n    // Store the autoComplete widget\n    new _js_auto_complete_js__WEBPACK_IMPORTED_MODULE_0___default.a({\n        selector: '#hips-selector',\n        minChars: 2,\n        source: function(term, suggest) {\n            term = term.toLowerCase();\n            var choices = hipsesArray;\n            var matches = [];\n            for (let i=0; i<choices.length; i++) {\n                if (choices[i].toLowerCase().indexOf(term)>=0) {\n                    matches.push(choices[i]);\n                }\n            }\n            suggest(matches);\n        },\n        renderItem: function (item, search) {\n            search = search.replace(/[-\\/\\\\^$*+?.()|[\\]{}]/g, '\\\\$&');\n            var re = new RegExp(\"(\" + search.split(' ').join('|') + \")\", \"gi\");\n            return '<div class=\"autocomplete-suggestion\" data-val=\"' + item + '\">' + item.replace(re, \"<b>$1</b>\") + '</div>';\n        },\n        onSelect: function(e, term, item) {\n            let hips_id = term;\n\n            let { hips_service_url, max_depth, hips_tile_format, tile_width } = hipsesMap[hips_id];\n            webClient.change_hips(hips_service_url,\n                tile_width,\n                max_depth,\n                hips_tile_format\n            );\n        }\n    });\n}\n\n//# sourceURL=webpack:///./widgets/hips_selector.js?");

/***/ })

/******/ });