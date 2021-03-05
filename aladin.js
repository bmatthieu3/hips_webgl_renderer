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
/******/ 		return __webpack_require__.p + "" + chunkId + ".aladin.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./src/core/pkg/index_bg.wasm": function() {
/******/ 			return {
/******/ 				"./index_bg.js": {
/******/ 					"__wbindgen_json_serialize": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_json_serialize"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_canvas_dd578e51a2bc736f": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_canvas_dd578e51a2bc736f"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430"](p0i32);
/******/ 					},
/******/ 					"__wbg_setwidth_41b2497107faaff7": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setwidth_41b2497107faaff7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setheight_e15cb9243262e701": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setheight_e15cb9243262e701"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_viewport_86b156d5858adab9": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_viewport_86b156d5858adab9"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_scissor_1f78ef0050a93516": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_scissor_1f78ef0050a93516"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbg_buffer_e35e010c3ba9f945": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_buffer_e35e010c3ba9f945"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_fe24eae01e10f223": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_fe24eae01e10f223"](p0i32);
/******/ 					},
/******/ 					"__wbg_subarray_3c6f7cfb4edcc351": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_subarray_3c6f7cfb4edcc351"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_measureText_2a4b2ca71061d96c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_measureText_2a4b2ca71061d96c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_width_979b596f39ba8319": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_width_979b596f39ba8319"](p0i32);
/******/ 					},
/******/ 					"__wbg_bindVertexArray_520c05423d3d6641": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bindVertexArray_520c05423d3d6641"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_bindBuffer_4a7874f09df12419": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bindBuffer_4a7874f09df12419"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_bufferSubData_51f29e78449b2095": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bufferSubData_51f29e78449b2095"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_bufferData_80963d2bd1ecb1bc": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bufferData_80963d2bd1ecb1bc"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_performance_800ff37c906b5f3b": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_performance_800ff37c906b5f3b"](p0i32);
/******/ 					},
/******/ 					"__wbg_now_9f22124bc74da886": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_now_9f22124bc74da886"](p0i32);
/******/ 					},
/******/ 					"__wbg_activeTexture_32edab6336bd38a9": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_activeTexture_32edab6336bd38a9"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_bindTexture_d659843380f373b5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bindTexture_d659843380f373b5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_texSubImage2D_e13399a16dfb0646": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_texSubImage2D_e13399a16dfb0646"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32);
/******/ 					},
/******/ 					"__wbg_blendFuncSeparate_13c318610edadb4a": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_blendFuncSeparate_13c318610edadb4a"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_enable_65590f4951fd0112": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_enable_65590f4951fd0112"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_cullFace_b0941c23a53ee9fc": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_cullFace_b0941c23a53ee9fc"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_innerWidth_aab6ec3242dff39e": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_innerWidth_aab6ec3242dff39e"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_innerHeight_7e514d9823f7864e": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_innerHeight_7e514d9823f7864e"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_17bf587bb9ce55f1": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_17bf587bb9ce55f1"]();
/******/ 					},
/******/ 					"__wbg_setcrossOrigin_054bb95c5a2b2640": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setcrossOrigin_054bb95c5a2b2640"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_4f8fb2c75215d83a": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_4f8fb2c75215d83a"]();
/******/ 					},
/******/ 					"__wbg_setresponseType_09ae5e5481a8947d": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setresponseType_09ae5e5481a8947d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createTexture_8ba2e566eb313fcf": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createTexture_8ba2e566eb313fcf"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_setonload_69f9426b613d7bd2": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setonload_69f9426b613d7bd2"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setonerror_e519d2d2cbd89b1d": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setonerror_e519d2d2cbd89b1d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setsrc_8742008d92b4e70e": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setsrc_8742008d92b4e70e"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_texParameteri_c0b2b665319f6a16": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_texParameteri_c0b2b665319f6a16"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_texImage2D_a5dad82b8f689bbd": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_texImage2D_a5dad82b8f689bbd"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32,p8i32,p9i32,p10i32);
/******/ 					},
/******/ 					"__wbg_createFramebuffer_edeb035499d73077": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createFramebuffer_edeb035499d73077"](p0i32);
/******/ 					},
/******/ 					"__wbg_bindFramebuffer_abbc9985c473f160": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_bindFramebuffer_abbc9985c473f160"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_framebufferTexture2D_bb45b3c3d234ddcd": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_framebufferTexture2D_bb45b3c3d234ddcd"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_createVertexArray_5cbff3d8bbe1c324": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createVertexArray_5cbff3d8bbe1c324"](p0i32);
/******/ 					},
/******/ 					"__wbg_useProgram_b1cc885b00b8f52c": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_useProgram_b1cc885b00b8f52c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createBuffer_4302ddbcbfc99048": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createBuffer_4302ddbcbfc99048"](p0i32);
/******/ 					},
/******/ 					"__wbg_lineWidth_a2c6059f833032d4": function(p0i32,p1f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_lineWidth_a2c6059f833032d4"](p0i32,p1f32);
/******/ 					},
/******/ 					"__wbg_vertexAttribPointer_3bb013e284cd07bf": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_vertexAttribPointer_3bb013e284cd07bf"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_enableVertexAttribArray_413ef49912a23f9e": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_enableVertexAttribArray_413ef49912a23f9e"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_document_d8cce4c1031c64eb": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_document_d8cce4c1031c64eb"](p0i32);
/******/ 					},
/******/ 					"__wbg_getElementsByClassName_a5ef560ae6918226": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getElementsByClassName_a5ef560ae6918226"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getwithindex_bcf1a04b716019a9": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getwithindex_bcf1a04b716019a9"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setAttribute_fb8737b4573a65f8": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setAttribute_fb8737b4573a65f8"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_getContext_d277f710e8035242": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getContext_d277f710e8035242"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3"](p0i32);
/******/ 					},
/******/ 					"__wbg_scale_e0fdce059098cd1b": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_scale_e0fdce059098cd1b"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_texImage2D_bc294af8c1a6a435": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_texImage2D_bc294af8c1a6a435"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32);
/******/ 					},
/******/ 					"__wbg_log_a39f164b49616cb0": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_log_a39f164b49616cb0"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_c77df81d6c892c35": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_c77df81d6c892c35"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_836859e5deb44d3f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_836859e5deb44d3f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_139e70222494b1ff": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_139e70222494b1ff"](p0i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_c274c3296a37fcb4": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_c274c3296a37fcb4"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_5b74a8dd0c5b71ac": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_5b74a8dd0c5b71ac"](p0i32);
/******/ 					},
/******/ 					"__wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_deleteBuffer_988823f4e76e697d": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_deleteBuffer_988823f4e76e697d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_createProgram_128698dd90ec070d": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createProgram_128698dd90ec070d"](p0i32);
/******/ 					},
/******/ 					"__wbg_attachShader_5d53b7b00823cafb": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_attachShader_5d53b7b00823cafb"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_linkProgram_370ed11b34456c89": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_linkProgram_370ed11b34456c89"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getProgramParameter_b949ba1d9662f6a2": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getProgramParameter_b949ba1d9662f6a2"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_boolean_get": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_boolean_get"](p0i32);
/******/ 					},
/******/ 					"__wbg_getProgramInfoLog_f8f65be65281f691": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getProgramInfoLog_f8f65be65281f691"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getActiveUniform_70b770a58f551f8f": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getActiveUniform_70b770a58f551f8f"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_name_4f3b7294acbeabad": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_name_4f3b7294acbeabad"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getUniformLocation_472b7459010900a5": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getUniformLocation_472b7459010900a5"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_createShader_26e4f959d5d64d80": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_createShader_26e4f959d5d64d80"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_shaderSource_96ace5133c032f2f": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_shaderSource_96ace5133c032f2f"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_compileShader_f7e245515fa1405d": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_compileShader_f7e245515fa1405d"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getShaderParameter_cced0ff8ba83f3e7": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getShaderParameter_cced0ff8ba83f3e7"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_getShaderInfoLog_5412e8bc642139e8": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getShaderInfoLog_5412e8bc642139e8"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_clearColor_fc22409197a5bd68": function(p0i32,p1f32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_clearColor_fc22409197a5bd68"](p0i32,p1f32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_clear_25e035ed3961f1c6": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_clear_25e035ed3961f1c6"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_width_d9e3643c351ff015": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_width_d9e3643c351ff015"](p0i32);
/******/ 					},
/******/ 					"__wbg_height_b92a879a29e66010": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_height_b92a879a29e66010"](p0i32);
/******/ 					},
/******/ 					"__wbg_uniform1f_fa50abe89ff891ea": function(p0i32,p1i32,p2f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform1f_fa50abe89ff891ea"](p0i32,p1i32,p2f32);
/******/ 					},
/******/ 					"__wbg_uniform2f_ab7c909be2949448": function(p0i32,p1i32,p2f32,p3f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform2f_ab7c909be2949448"](p0i32,p1i32,p2f32,p3f32);
/******/ 					},
/******/ 					"__wbg_drawElementsInstanced_6a606cd25bdbafb3": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_drawElementsInstanced_6a606cd25bdbafb3"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_drawElements_c109bfea7998fd99": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_drawElements_c109bfea7998fd99"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniform4f_9941fe9c32da60ea": function(p0i32,p1i32,p2f32,p3f32,p4f32,p5f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform4f_9941fe9c32da60ea"](p0i32,p1i32,p2f32,p3f32,p4f32,p5f32);
/******/ 					},
/******/ 					"__wbg_drawArrays_f6e7af9c06f4f4ae": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_drawArrays_f6e7af9c06f4f4ae"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform1i_a1e8f5ad954fa6b5": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform1i_a1e8f5ad954fa6b5"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_clearRect_620b55f817af6080": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_clearRect_620b55f817af6080"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_setfont_7d7b206c4c017729": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setfont_7d7b206c4c017729"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_settextAlign_0ab90671be8e1137": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_settextAlign_0ab90671be8e1137"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_setfillStyle_1b068f8d99084158": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setfillStyle_1b068f8d99084158"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_save_be2f4340f20bfe6f": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_save_be2f4340f20bfe6f"](p0i32);
/******/ 					},
/******/ 					"__wbg_translate_a603cdd310297ee8": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_translate_a603cdd310297ee8"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__wbg_rotate_4ae42333a58388ed": function(p0i32,p1f64) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_rotate_4ae42333a58388ed"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_fillText_aee0d6016521a3b2": function(p0i32,p1i32,p2i32,p3f64,p4f64) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_fillText_aee0d6016521a3b2"](p0i32,p1i32,p2i32,p3f64,p4f64);
/******/ 					},
/******/ 					"__wbg_restore_e6861230b7a8a25e": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_restore_e6861230b7a8a25e"](p0i32);
/******/ 					},
/******/ 					"__wbg_disable_827be6d0f77447e1": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_disable_827be6d0f77447e1"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_parse_b89e797098b3bc7b": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_parse_b89e797098b3bc7b"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_getContext_a5ae0a2c4fe6f42b": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_getContext_a5ae0a2c4fe6f42b"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_json_parse": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_json_parse"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_uniform1fv_7b33ccba8ca090e4": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform1fv_7b33ccba8ca090e4"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniformMatrix4fv_82825540b9315680": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniformMatrix4fv_82825540b9315680"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_uniform1iv_2c8af1d8286865f0": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform1iv_2c8af1d8286865f0"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__wbg_uniform3f_a7c04d3d1c2b18aa": function(p0i32,p1i32,p2f32,p3f32,p4f32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_uniform3f_a7c04d3d1c2b18aa"](p0i32,p1i32,p2f32,p3f32,p4f32);
/******/ 					},
/******/ 					"__wbg_isArray_3320300beb1837ab": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_isArray_3320300beb1837ab"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_8f15bbb4ecbf7e33": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_length_8f15bbb4ecbf7e33"](p0i32);
/******/ 					},
/******/ 					"__wbg_get_40375c2067f479fc": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_get_40375c2067f479fc"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_response_c70a68323728a385": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_response_c70a68323728a385"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_2cfa674c2a529bc1": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_length_2cfa674c2a529bc1"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_d771848e3c7935bb": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_set_d771848e3c7935bb"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteVertexArray_3f37aabaae61ca26": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_deleteVertexArray_3f37aabaae61ca26"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_disableVertexAttribArray_1401ee870505cf02": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_disableVertexAttribArray_1401ee870505cf02"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_deleteTexture_cf22af6782ebc54f": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_deleteTexture_cf22af6782ebc54f"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_eeabd9085c04fc17": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_self_eeabd9085c04fc17"]();
/******/ 					},
/******/ 					"__wbg_window_f110c13310da2c8f": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_window_f110c13310da2c8f"]();
/******/ 					},
/******/ 					"__wbg_globalThis_a2669bee93faee43": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_globalThis_a2669bee93faee43"]();
/******/ 					},
/******/ 					"__wbg_global_a5584d717f4d6761": function() {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_global_a5584d717f4d6761"]();
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_179d393e4626fcf7": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newnoargs_179d393e4626fcf7"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_8487a9f580e47219": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_call_8487a9f580e47219"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_a20c8edf0fedac40": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_a20c8edf0fedac40"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffsetandlength_7b9a415096aef9c1": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_newwithbyteoffsetandlength_7b9a415096aef9c1"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_blendFunc_8bd5998b54c12fd3": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_blendFunc_8bd5998b54c12fd3"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_open_c1608202d44b7d1c": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_open_c1608202d44b7d1c"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__wbg_setonload_c71ccab98777e104": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setonload_c71ccab98777e104"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setonerror_1bea8ceda68d0d63": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_setonerror_1bea8ceda68d0d63"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_send_3e459af287bba919": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_send_3e459af287bba919"](p0i32);
/******/ 					},
/******/ 					"__wbg_texSubImage2D_43d09711529aa698": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_texSubImage2D_43d09711529aa698"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32,p6i32,p7i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					},
/******/ 					"__wbg_instanceof_Window_fa4595281eb5ba83": function(p0i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbg_instanceof_Window_fa4595281eb5ba83"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper166": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_closure_wrapper166"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper170": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./src/core/pkg/index_bg.js"].exports["__wbindgen_closure_wrapper170"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 		"./node_modules/@fxpineau/healpix/healpix_bg.wasm": function() {
/******/ 			return {
/******/ 				"./healpix_bg.js": {
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./node_modules/@fxpineau/healpix/healpix_bg.js"].exports["__wbindgen_throw"](p0i32,p1i32);
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
/******/ 		var wasmModules = {"1":["./src/core/pkg/index_bg.wasm"],"2":["./node_modules/@fxpineau/healpix/healpix_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./src/core/pkg/index_bg.wasm":"491ab684e58b8267047e","./node_modules/@fxpineau/healpix/healpix_bg.wasm":"4b971868181910368ee1"}[wasmModuleId] + ".module.wasm");
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
/******/ 	return __webpack_require__(__webpack_require__.s = "./src/js/Aladin.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./src/core/img/kernel.png":
/*!*********************************!*\
  !*** ./src/core/img/kernel.png ***!
  \*********************************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony default export */ __webpack_exports__["default"] = (__webpack_require__.p + "ccdb93d24585bd08a4261722b95e9e3d.png");

/***/ }),

/***/ "./src/core/src/shaders/catalogs/aitoff.vert":
/*!***************************************************!*\
  !*** ./src/core/src/shaders/catalogs/aitoff.vert ***!
  \***************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\nuniform float current_time;\nuniform mat4 model;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_aitoff(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/arc.vert":
/*!************************************************!*\
  !*** ./src/core/src/shaders/catalogs/arc.vert ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\nuniform float current_time;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_arc(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/catalog.frag":
/*!****************************************************!*\
  !*** ./src/core/src/shaders/catalogs/catalog.frag ***!
  \****************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nin vec3 out_p;\n\nout vec4 color;\n\nuniform sampler2D kernel_texture;\nuniform float max_density; // max number of sources in a kernel sized HEALPix cell at the current depth\nuniform float fov;\nuniform float strength;\nvoid main() {\n    color = texture(kernel_texture, out_uv) / max(log2(fov*100.0), 1.0);\n    color.r *= strength;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/mercator.vert":
/*!*****************************************************!*\
  !*** ./src/core/src/shaders/catalogs/mercator.vert ***!
  \*****************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\nuniform float current_time;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_mercator(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/mollweide.vert":
/*!******************************************************!*\
  !*** ./src/core/src/shaders/catalogs/mollweide.vert ***!
  \******************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\nuniform float current_time;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_mollweide(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/ortho.frag":
/*!**************************************************!*\
  !*** ./src/core/src/shaders/catalogs/ortho.frag ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nin vec3 out_p;\n\nout vec4 color;\n\nuniform sampler2D kernel_texture;\nuniform float max_density; // max number of sources in a kernel sized HEALPix cell at the current depth\nuniform float fov;\nuniform float strength;\nvoid main() {\n    if (out_p.z < 0.f) {\n        discard;\n    }\n\n    color = texture(kernel_texture, out_uv) / max(log2(fov*100.0), 1.0);\n    color.r *= strength;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/ortho.vert":
/*!**************************************************!*\
  !*** ./src/core/src/shaders/catalogs/ortho.vert ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\n\nuniform float current_time;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_orthographic(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/catalogs/tan.vert":
/*!************************************************!*\
  !*** ./src/core/src/shaders/catalogs/tan.vert ***!
  \************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nlayout (location = 0) in vec2 offset;\nlayout (location = 1) in vec2 uv;\n\nlayout (location = 2) in vec3 center;\nlayout (location = 3) in vec2 center_lonlat;\n\n\nuniform float current_time;\nuniform mat4 inv_model;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\nuniform vec2 kernel_size;\n\nout vec2 out_uv;\nout vec3 out_p;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    vec3 p = vec3(inv_model * vec4(center, 1.0f));\n    p = check_inversed_longitude(p);\n\n    vec2 center_pos_clip_space = world2clip_gnomonic(p);\n\n    vec2 pos_clip_space = center_pos_clip_space;\n    gl_Position = vec4((pos_clip_space / (ndc_to_clip * czf)) + offset * kernel_size , 0.f, 1.f);\n\n    out_uv = uv;\n    out_p = p;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/BluePastelRed.frag":
/*!***********************************************************!*\
  !*** ./src/core/src/shaders/colormaps/BluePastelRed.frag ***!
  \***********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nprecision lowp sampler2D;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform float alpha;\n\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 colormap_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    color = colormap_f(opacity);\n    color.a = o * alpha;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/IDL_CB_BrBG.frag":
/*!*********************************************************!*\
  !*** ./src/core/src/shaders/colormaps/IDL_CB_BrBG.frag ***!
  \*********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform sampler2D colormap;\nuniform float alpha;\nuniform float strength;\n\nfloat colormap_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 colormap_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    //color = texture(colormap, vec2(opacity, 0.5f));\n    color = colormap_f(opacity);\n    color.a = alpha * o;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/IDL_CB_GnBu.frag":
/*!*********************************************************!*\
  !*** ./src/core/src/shaders/colormaps/IDL_CB_GnBu.frag ***!
  \*********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform sampler2D colormap;\nuniform float alpha;\n\nfloat colormap_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat colormap_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 colormap_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    //color = texture(colormap, vec2(opacity, 0.5f));\n    color = colormap_f(opacity);\n    color.a = alpha * o;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/IDL_CB_YIGnBu.frag":
/*!***********************************************************!*\
  !*** ./src/core/src/shaders/colormaps/IDL_CB_YIGnBu.frag ***!
  \***********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform sampler2D colormap;\nuniform float alpha;\n\nfloat colormap_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 colormap_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    //color = texture(colormap, vec2(opacity, 0.5f));\n    color = colormap_f(opacity);\n    color.a = alpha * o;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/blackwhite.frag":
/*!********************************************************!*\
  !*** ./src/core/src/shaders/colormaps/blackwhite.frag ***!
  \********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform sampler2D colormap;\nuniform float alpha;\n\nvec4 colormap_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    //color = texture(colormap, vec2(opacity, 0.5f));\n    color = colormap_f(opacity);\n    color.a = alpha * o;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/colormap.vert":
/*!******************************************************!*\
  !*** ./src/core/src/shaders/colormaps/colormap.vert ***!
  \******************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nlayout (location = 0) in vec2 position;\nlayout (location = 1) in vec2 uv;\n\nout vec2 out_uv;\n\nvoid main() {\n    gl_Position = vec4(position, 0.f, 1.f);\n    out_uv = uv;\n}"

/***/ }),

/***/ "./src/core/src/shaders/colormaps/red.frag":
/*!*************************************************!*\
  !*** ./src/core/src/shaders/colormaps/red.frag ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nin vec2 out_uv;\nout vec4 color;\n\nuniform sampler2D texture_fbo;\nuniform sampler2D colormap;\nuniform float alpha;\n\nfloat colormap_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat colormap_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat colormap_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 colormap_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(colormap_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(t) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\nvoid main() {\n    float opacity = texture(texture_fbo, out_uv).r;\n\n    float o = smoothstep(0.f, 0.1f, opacity);\n\n    //color = texture(colormap, vec2(opacity, 0.5f));\n    color = colormap_f(opacity);\n    color.a = alpha * o;\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/aitoff.frag":
/*!***********************************************!*\
  !*** ./src/core/src/shaders/grid/aitoff.frag ***!
  \***********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 to_icrs;\nuniform mat4 inv_model;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\n\nbool is_included_inside_projection(vec2 pos_clip_space) {\n    float px2 = pos_clip_space.x * pos_clip_space.x;\n    float py2 = pos_clip_space.y * pos_clip_space.y;\n\n    return (px2 * 0.25 + py2) <= 0.25;\n}\n\n/// View to world space transformation\n/// \n/// This returns a normalized vector along its first 3 dimensions.\n/// Its fourth component is set to 1.\n/// \n/// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]\n/// \n/// # Arguments\n/// \n/// * `x` - in normalized device coordinates between [-1; 1]\n/// * `y` - in normalized device coordinates between [-1; 1]\nvec3 clip2world_aitoff(vec2 pos_clip_space) {\n    if(!is_included_inside_projection(pos_clip_space)) {\n        discard;\n    }\n\n    vec2 uv = vec2(pos_clip_space.x * PI * 0.5, pos_clip_space.y * PI);\n    //da uv a lat/lon\n    float c = length(uv);\n\n    float phi = asin(uv.y * sin(c) / c);\n    float theta = atan(uv.x * sin(c), c * cos(c)) * 2.0;\n\n    vec3 world = vec3(\n        sin(theta) * cos(phi),\n        sin(phi),\n        cos(theta) * cos(phi)\n    );\n    return world;\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 posmodel = pos_model;\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(posmodel, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, posmodel));\n\n    vec3 h_model = normalize(posmodel - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n    h_world = check_inversed_longitude(h_world);\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    vec2 h_clip = world2clip_aitoff(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    vec3 posmodel = pos_model;\n    float y = atan(posmodel.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_aitoff(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f));\n\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/arc.frag":
/*!********************************************!*\
  !*** ./src/core/src/shaders/grid/arc.frag ***!
  \********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 inv_model;\nuniform mat4 to_icrs;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nfloat sinc_positive(float x) {\n    if (x > 1.0e-4) {\n        return sin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        x = x*x;\n        return 1.0 - x * (1.0 - x / 20.0) / 6.0;\n    }\n}\n\nvec3 clip2world_arc(vec2 pos_clip_space) {\n    // r <= pi\n    float x = pos_clip_space.x * PI;\n    float y = pos_clip_space.y * PI;\n    float r = length(vec2(x, y));\n    if (r <= PI) {\n        float z = cos(r);\n        r = sinc_positive(r);\n\n        return vec3(x * r, y * r, z);\n    }\n    discard;\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(pos_model, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, pos_model));\n\n    vec3 h_model = normalize(pos_model - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    h_world = check_inversed_longitude(h_world);\n    vec2 h_clip = world2clip_arc(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    float y = atan(pos_model.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_arc(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f));\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/grid.vert":
/*!*********************************************!*\
  !*** ./src/core/src/shaders/grid/grid.vert ***!
  \*********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nlayout (location = 0) in vec2 position;\n\nout vec2 pos_clip;\n\nuniform vec2 ndc_to_clip;\nuniform float czf;\n\nvoid main() {\n    pos_clip = position * (ndc_to_clip * czf);\n\n    gl_Position = vec4(position, 0.0, 1.0);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/grid_cpu.frag":
/*!*************************************************!*\
  !*** ./src/core/src/shaders/grid/grid_cpu.frag ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nout vec4 frag_color;\n\nuniform vec4 color;\nuniform float opacity;\n\nconst float PI = 3.141592653589793f;\n\nvoid main() {\n    frag_color = color;\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/grid_cpu.vert":
/*!*************************************************!*\
  !*** ./src/core/src/shaders/grid/grid_cpu.vert ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\n\nlayout (location = 0) in vec2 ndc_pos;\n\nvoid main() {\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/mercator.frag":
/*!*************************************************!*\
  !*** ./src/core/src/shaders/grid/mercator.frag ***!
  \*************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 inv_model;\nuniform mat4 to_icrs;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvec3 clip2world_mercator(vec2 p) {\n    float theta = p.x * PI;\n    float delta = atan(sinh(p.y)) * PI;\n\n    return vec3(sin(theta) * cos(delta), sin(delta), cos(theta) * cos(delta));\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(pos_model, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, pos_model));\n\n    vec3 h_model = normalize(pos_model - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n    h_world = check_inversed_longitude(h_world);\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    vec2 h_clip = world2clip_mercator(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    float y = atan(pos_model.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_mercator(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f));\n\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/mollweide.frag":
/*!**************************************************!*\
  !*** ./src/core/src/shaders/grid/mollweide.frag ***!
  \**************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 inv_model;\nuniform mat4 to_icrs;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nbool is_included_inside_projection(vec2 pos_clip_space) {\n    float px2 = pos_clip_space.x * pos_clip_space.x;\n    float py2 = pos_clip_space.y * pos_clip_space.y;\n\n    return (px2 * 0.25 + py2) <= 0.25;\n}\n\n/// View to world space transformation\n/// \n/// This returns a normalized vector along its first 3 dimensions.\n/// Its fourth component is set to 1.\n/// \n/// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]\n/// \n/// # Arguments\n/// \n/// * `x` - in normalized device coordinates between [-1; 1]\n/// * `y` - in normalized device coordinates between [-1; 1]\nvec3 clip2world_mollweide(vec2 pos_clip_space) {\n    if (!is_included_inside_projection(pos_clip_space)) {\n        discard;\n    }\n\n    float y2 = pos_clip_space.y * pos_clip_space.y;\n    float k = sqrt(1.0 - 4.0 * y2);\n\n    float theta = PI * pos_clip_space.x / k;\n    float delta = asin((2.0 * asin(2.0 * pos_clip_space.y) + 4.0 * pos_clip_space.y * k) / PI);\n    \n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    return vec3(sin(theta) * cos(delta), sin(delta), cos(theta) * cos(delta));\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(pos_model, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, pos_model));\n\n    vec3 h_model = normalize(pos_model - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n    h_world = check_inversed_longitude(h_world);\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    vec2 h_clip = world2clip_mollweide(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    float y = atan(pos_model.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_mollweide(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f));\n\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/ortho.frag":
/*!**********************************************!*\
  !*** ./src/core/src/shaders/grid/ortho.frag ***!
  \**********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 inv_model;\nuniform mat4 to_icrs;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvec3 clip2world_orthographic(vec2 pos_clip_space) {\n    float z = 1.f - dot(pos_clip_space, pos_clip_space);\n    if (z > 0.f) {\n        return vec3(pos_clip_space.x, pos_clip_space.y, sqrt(z));\n    } else {\n        discard;\n    }\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(pos_model, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, pos_model));\n\n    vec3 h_model = normalize(pos_model - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n    h_world = check_inversed_longitude(h_world);\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    vec2 h_clip = world2clip_orthographic(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    float y = atan(pos_model.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d*2.0;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_orthographic(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f));\n\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/grid/tan.frag":
/*!********************************************!*\
  !*** ./src/core/src/shaders/grid/tan.frag ***!
  \********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\n\nout vec4 c;\nin vec2 pos_clip;\n\nuniform vec4 color;\nuniform mat4 model;\nuniform mat4 inv_model;\nuniform mat4 to_icrs;\nuniform float czf;\n\nuniform float meridians[20];\nuniform int num_meridians;\nuniform float parallels[10];\nuniform int num_parallels;\n\nuniform vec2 window_size;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\n/// View to world space transformation\n/// \n/// This returns a normalized vector along its first 3 dimensions.\n/// Its fourth component is set to 1.\n/// \n/// The Aitoff projection maps screen coordinates from [-pi; pi] x [-pi/2; pi/2]\n/// \n/// # Arguments\n/// \n/// * `x` - in normalized device coordinates between [-1; 1]\n/// * `y` - in normalized device coordinates between [-1; 1]\nvec3 clip2world_gnomonic(vec2 pos_clip_space) {\n    float x_2d = pos_clip_space.x * PI;\n    float y_2d = pos_clip_space.y * PI;\n    float r = x_2d * x_2d + y_2d * y_2d;\n\n    float z = sqrt(1.0 + r);\n    return vec3(z * x_2d, z * y_2d, z);\n}\n\nfloat d_isolon(vec3 pos_model, float theta) {\n    vec3 n = vec3(cos(theta), 0.0, -sin(theta));\n    // Discard the (theta + PI) meridian\n    vec3 e_xz = vec3(-n.z, 0.0, n.x);\n    if (dot(pos_model, e_xz) < 0.0) {\n        return 1e3;\n    }\n\n    float d = abs(dot(n, pos_model));\n\n    vec3 h_model = normalize(pos_model - n*d);\n    vec3 h_world = vec3(inv_model * to_icrs * vec4(h_model, 1.f));\n    h_world = check_inversed_longitude(h_world);\n\n    // Project to screen x and h and compute the distance\n    // between the two\n    vec2 h_clip = world2clip_gnomonic(h_world);\n    \n    return length(pos_clip - h_clip) * 2.0;\n}\nfloat d_isolat(vec3 pos_model, float delta) {\n    float y = atan(pos_model.y, length(pos_model.xz));\n    float d = abs(y - delta);\n    return d;\n}\n\nfloat grid_alpha(vec3 p) {\n    float v = 1e10;\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float m = 0.0;\n    float mdist = 10.0;\n    for (int i = 0; i < num_meridians; i++) {\n        float tmp = meridians[i];\n        if (tmp > PI) {\n            tmp -= 2.0 * PI;\n        }\n        float d = abs(theta - tmp);\n        if (d < mdist) {\n            mdist = d;\n            m = tmp;\n        }\n    }\n\n    float par = 0.0;\n    float pdist = 10.0;\n    for (int i = 0; i < num_parallels; i++) {\n        float d = abs(delta - parallels[i]);\n        if (d < pdist) {\n            pdist = d;\n            par = parallels[i];\n        }\n    }\n\n    /*float a = 0.0;\n    if (mdist < pdist) {\n        a = d_isolon(p, m);\n    } else {\n        a = d_isolat(p, par);\n    }\n    v = min(a, v);*/\n    v = min(d_isolon(p, m), v);\n    v = min(d_isolat(p, par), v);\n\n    float eps = 3.0 * czf / window_size.x;\n    return smoothstep(eps, 2.0*eps, v);\n}\n\nvoid main() {\n    vec4 transparency = vec4(0.f, 0.f, 0.f, 0.f);\n\n    vec3 pos_world = clip2world_gnomonic(pos_clip);\n    pos_world = check_inversed_longitude(pos_world);\n\n    vec3 pos_model = normalize(vec3(transpose(to_icrs) * model * vec4(pos_world, 1.f)));\n    float alpha = grid_alpha(pos_model);\n    c = mix(color, transparency, alpha);\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/aitoff.vert":
/*!**********************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/aitoff.vert ***!
  \**********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    gl_Position = vec4(world2clip_aitoff(world_pos) / (ndc_to_clip * czf), 0.0, 1.0);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/arc.vert":
/*!*******************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/arc.vert ***!
  \*******************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    gl_Position = vec4(world2clip_arc(world_pos) / (ndc_to_clip * czf), 0.0, 1.0);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/color.frag":
/*!*********************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/color.frag ***!
  \*********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision mediump int;\n\nin vec3 frag_uv_start;\nin vec3 frag_uv_end;\nin float frag_blending_factor;\nin float m_start;\nin float m_end;\n\nout vec4 out_frag_color;\nuniform float opacity;\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\n\nvoid main() {\n    vec4 color_start = get_color_from_texture(frag_uv_start);\n    vec4 color_end = get_color_from_texture(frag_uv_end);\n\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\n    out_frag_color.a = opacity * out_frag_color.a;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/gnomonic.vert":
/*!************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/gnomonic.vert ***!
  \************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    gl_Position = vec4(world2clip_gnomonic(world_pos) / (ndc_to_clip * czf), 0.0, 1.0);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/grayscale_to_color.frag":
/*!**********************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/grayscale_to_color.frag ***!
  \**********************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision mediump int;\n\nin vec3 frag_uv_start;\nin vec3 frag_uv_end;\nin float frag_blending_factor;\nin float m_start;\nin float m_end;\n\nout vec4 out_frag_color;\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\n\nuniform float opacity;\n\nvec4 get_color(vec3 uv, float empty) {\n    vec4 color = get_color_from_grayscale_texture(uv);\n    return color;\n}\n\nvoid main() {\n    vec4 color_start = get_color(frag_uv_start, m_start);\n    vec4 color_end = get_color(frag_uv_end, m_end);\n\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\n    out_frag_color.a = out_frag_color.a * opacity;\n}\n\n"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/grayscale_to_color_i.frag":
/*!************************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/grayscale_to_color_i.frag ***!
  \************************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision mediump int;\n\nin vec3 frag_uv_start;\nin vec3 frag_uv_end;\nin float frag_blending_factor;\nin float m_start;\nin float m_end;\n\nout vec4 out_frag_color;\n\n//const int MAX_NUM_TEX = 3;\nuniform isampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nfloat get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return float(texture(tex[0], uv.xy).r);\n    } else if (idx_texture == 1) {\n        return float(texture(tex[1], uv.xy).r);\n    } else if (idx_texture == 2) {\n        return float(texture(tex[2], uv.xy).r);\n    } else {\n        return 0.0;\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\n\nuniform float opacity;\n\nvec4 get_color(vec3 uv, float empty) {\n    vec4 color = get_color_from_grayscale_texture(uv);\n    return color;\n}\n\nvoid main() {\n    vec4 color_start = get_color(frag_uv_start, m_start);\n    vec4 color_end = get_color(frag_uv_end, m_end);\n\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap.frag":
/*!*************************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap.frag ***!
  \*************************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision mediump int;\n\nin vec3 frag_uv_start;\nin vec3 frag_uv_end;\nin float frag_blending_factor;\nin float m_start;\nin float m_end;\n\nout vec4 out_frag_color;\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\n\nuniform float opacity;\n\nvec4 get_color(vec3 uv, float empty) {\n    vec4 color = get_colormap_from_grayscale_texture(uv);\n    return color;\n}\n\nvoid main() {\n    vec4 color_start = get_color(frag_uv_start, m_start);\n    vec4 color_end = get_color(frag_uv_end, m_end);\n\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap_i.frag":
/*!***************************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap_i.frag ***!
  \***************************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision mediump int;\n\nin vec3 frag_uv_start;\nin vec3 frag_uv_end;\nin float frag_blending_factor;\nin float m_start;\nin float m_end;\n\nout vec4 out_frag_color;\n\n//const int MAX_NUM_TEX = 3;\nuniform isampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nfloat get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return float(texture(tex[0], uv.xy).r);\n    } else if (idx_texture == 1) {\n        return float(texture(tex[1], uv.xy).r);\n    } else if (idx_texture == 2) {\n        return float(texture(tex[2], uv.xy).r);\n    } else {\n        return 0.0;\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\n\nuniform float opacity;\n\nvec4 get_color(vec3 uv, float empty) {\n    vec4 color = get_colormap_from_grayscale_texture(uv);\n    return color;\n}\n\nvoid main() {\n    vec4 color_start = get_color(frag_uv_start, m_start);\n    vec4 color_end = get_color(frag_uv_end, m_end);\n\n    out_frag_color = mix(color_start, color_end, frag_blending_factor);\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/mercator.vert":
/*!************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/mercator.vert ***!
  \************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    gl_Position = vec4(world2clip_mercator(world_pos) / (ndc_to_clip * czf), 0.0, 1.0);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/mollweide.vert":
/*!*************************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/mollweide.vert ***!
  \*************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    gl_Position = vec4(world2clip_mollweide(world_pos) / (ndc_to_clip * czf), 0.0, 1.0);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/rasterizer/ortho.vert":
/*!*********************************************************!*\
  !*** ./src/core/src/shaders/hips/rasterizer/ortho.vert ***!
  \*********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision mediump int;\n\nlayout (location = 0) in vec2 lonlat;\n//layout (location = 1) in vec3 position;\nlayout (location = 1) in vec2 ndc_pos;\nlayout (location = 2) in vec3 uv_start;\nlayout (location = 3) in vec3 uv_end;\nlayout (location = 4) in float time_tile_received;\nlayout (location = 5) in float m0;\nlayout (location = 6) in float m1;\n\nout vec3 frag_uv_start;\nout vec3 frag_uv_end;\nout float frag_blending_factor;\nout float m_start;\nout float m_end;\n\nuniform mat4 inv_model;\nuniform vec2 ndc_to_clip;\n//uniform float czf;\n\n// current time in ms\nuniform float current_time;\n\nconst float PI = 3.1415926535897932384626433832795f;\n\nuniform int inversed_longitude;\n\nconst mat3 inverseLongitude = mat3(\n    -1.0, 0.0, 0.0,\n    0.0, 1.0, 0.0,\n    0.0, 0.0, 1.0\n);\n\nconst mat4 GAL2J2000 = mat4(\n    -0.4448296299195045,\n    0.7469822444763707,\n    0.4941094279435681,\n    0.0,\n\n    -0.1980763734646737,\n    0.4559837762325372,\n    -0.8676661489811610,\n    0.0,\n\n    -0.873437090247923,\n    -0.4838350155267381,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nconst mat4 J20002GAL = mat4(\n    -0.4448296299195045,\n    -0.1980763734646737,\n    -0.873437090247923,\n    0.0,\n\n    0.7469822444763707,\n    0.4559837762325372,\n    -0.4838350155267381,\n    0.0,\n\n    0.4941094279435681,\n    -0.8676661489811610,\n    -0.0548755604024359,\n    0.0,\n\n    0.0,\n    0.0,\n    0.0,\n    1.0\n);\n\nvec3 check_inversed_longitude(vec3 p) {\n    if (inversed_longitude == 1) {\n        return inverseLongitude * p;\n    } else {\n        return p;\n    }\n}\n\nvec2 world2clip_orthographic(vec3 p) {\n    return vec2(p.x, p.y);\n}\n\nvec2 world2clip_aitoff(vec3 p) {\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float theta_by_two = theta * 0.5f;\n\n    float alpha = acos(cos(delta)*cos(theta_by_two));\n    float inv_sinc_alpha = 1.f;\n    if (alpha > 1e-3f) {\n        inv_sinc_alpha = alpha / sin(alpha);\n    }\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = 2.f * inv_sinc_alpha * cos(delta) * sin(theta_by_two);\n    float y = inv_sinc_alpha * sin(delta);\n\n    return vec2(x / PI, y / PI);\n}\n\nvec2 world2clip_mollweide(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n    int max_iter = 10;\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float cst = PI * sin(delta);\n\n    float phi = delta;\n    float f = phi + sin(phi) - cst;\n\n    int k = 0;\n    while (abs(f) > 1e-6 && k < max_iter) {\n        phi = phi - f / (1.f + cos(phi));\n        f = phi + sin(phi) - cst;\n\n        k = k + 1;\n    }\n\n    phi = phi * 0.5f;\n\n    // The minus is an astronomical convention.\n    // longitudes are increasing from right to left\n    float x = (theta / PI) * cos(phi);\n    float y = 0.5f * sin(phi);\n\n    return vec2(x, y);\n}\n\nvec2 world2clip_mercator(vec3 p) {\n    // X in [-1, 1]\n    // Y in [-1/2; 1/2] and scaled by the screen width/height ratio\n\n    float delta = asin(p.y);\n    float theta = atan(p.x, p.z);\n\n    float x = theta / PI;\n    float y = asinh(tan(delta / PI));\n\n    return vec2(x, y);\n}\n\nfloat arc_sinc(float x) {\n    if (x > 1e-4) {\n        return asin(x) / x;\n    } else {\n        // If a is mall, use Taylor expension of asin(a) / a\n        // a = 1e-4 => a^4 = 1.e-16\n        float x2 = x*x;\n        return 1.0 + x2 * (1.0 + x2 * 9.0 / 20.0) / 6.0;\n    }\n}\n\nvec2 world2clip_arc(vec3 p) {\n    if (p.z > -1.0) {\n        // Distance in the Euclidean plane (xy)\n        // Angular distance is acos(x), but for small separation, asin(r)\n        // is more accurate.\n        float r = length(p.xy);\n        if (p.z > 0.0) { // Angular distance < PI/2, angular distance = asin(r)\n            r = arc_sinc(r);\n        } else { // Angular distance > PI/2, angular distance = acos(x)\n            r = acos(p.z) / r;\n        }\n        float x = p.x * r;\n        float y = p.y * r;\n\n        return vec2(x / PI, y / PI);\n    } else {\n        return vec2(1.0, 0.0);\n    }\n}\n\nvec2 world2clip_gnomonic(vec3 p) {\n    if (p.z <= 1e-2) { // Back hemisphere (x < 0) + diverges near x=0\n        return vec2(1.0, 0.0);\n    } else {\n        return vec2((p.x/p.z) / PI , (p.y/p.z) / PI);\n    }\n}\n\nvoid main() {\n    /*\n    vec3 world_pos = vec3(inv_model * vec4(position, 1.f));\n    world_pos = check_inversed_longitude(world_pos);\n\n    vec2 ndc_pos = world2clip_orthographic(world_pos) / (ndc_to_clip * czf);\n    */\n    gl_Position = vec4(ndc_pos, 0.0, 1.0);\n\n    frag_uv_start = uv_start;\n    frag_uv_end = uv_end;\n\n    frag_blending_factor = min((current_time - time_tile_received) / 500.f, 1.f);\n    m_start = m0;\n    m_end = m1;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/color.frag":
/*!********************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/color.frag ***!
  \********************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision highp int;\n\nin vec3 out_vert_pos;\n\nout vec4 out_frag_color;\n\nuniform int user_action;\n\nstruct Tile {\n    int uniq; // Healpix cell\n    int texture_idx; // Index in the texture buffer\n    float start_time; // Absolute time that the load has been done in ms\n    int empty;\n};\n\nuniform int current_depth;\nuniform Tile textures_tiles[12];\n\nuniform float current_time; // current time in ms\nstruct TileColor {\n    Tile tile;\n    vec4 color;\n    bool found;\n};\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\nconst float TWICE_PI = 6.28318530718f;\nconst float PI = 3.141592653589793f;\nconst float FOUR_OVER_PI = 1.27323954474f;\nconst float TRANSITION_Z = 0.66666666666f;\nconst float TRANSITION_Z_INV = 1.5f;\n\nint quarter(vec2 p) {\n    int x_neg = int(p.x < 0.0f);\n    int y_neg = int(p.y < 0.0f);\n    int q = (x_neg + y_neg) | (y_neg << 1);\n    return q;\n}\n\nfloat xpm1(vec2 p) {\n    bool x_neg = (p.x < 0.0f);\n    //debug_assert!(x_neg <= 1);\n    bool y_neg = (p.y < 0.0f);\n    //debug_assert!(y_neg <= 1);\n    // The purpose it to have the same numerical precision for each base cell\n    // by avoiding subtraction by 1 or 3 or 5 or 7\n    float lon = atan(abs(p.y), abs(p.x));\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\n    float x02 = lon * FOUR_OVER_PI;\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\n        return 1.0f - x02;\n    } else {\n        return x02 - 1.0f;\n    }\n}\n\nfloat one_minus_z_pos(vec3 p) {\n    //debug_assert!(z > 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return 1.0f - p.z;\n}\n\nfloat one_minus_z_neg(vec3 p) {\n    //debug_assert!(z < 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\n        // 0.5 * d2 + 0.125 * d2 * d2\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return p.z + 1.0f;\n}\n\n// Z-Order curve projection.\nint ij2z(int i, int j) {\n    int i1 = i | (j << 16);\n\n    int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;\n    int i2 = i1 ^ j1 ^ (j1 << 8);\n\n    int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;\n    int i3 = i2 ^ j2 ^ (j2 << 4);\n\n    int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;\n    int i4 = i3 ^ j3 ^ (j3 << 2);\n\n    int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;\n    int i5 = i4 ^ j4 ^ (j4 << 1);\n\n    return i5;\n}\n\nstruct HashDxDy {\n    int idx;\n    float dx;\n    float dy;\n};\n\nuniform sampler2D ang2pixd;\nHashDxDy hash_with_dxdy2(vec2 radec) {\n    vec2 aa = vec2(radec.x/TWICE_PI + 1.0, (radec.y/PI) + 0.5);\n    vec3 v = texture(ang2pixd, aa).rgb;\n    return HashDxDy(\n        int(v.x * 255.0),\n        v.y,\n        v.z\n    );\n}\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\n// respect to the origin of the cell (South vertex).\n// # Inputs:\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\n// - `x`: in `[-1.0, 1.0]`\n// - `y`: in `[-1.0, 1.0]`\n// - `z`: in `[-1.0, 1.0]`\n// # Output\n// - the cell number (hash value) associated with the given position on the unit sphere,\n//   in `[0, 12*nside^2[`\n// - `dx`: the positional offset $\\in [0, 1[$ along the south-to-east axis\n// - `dy`: the positional offset $\\in [0, 1[$ along the south-to-west axis\n// # WARNING\n// - The function assumes, without checking, that the input vector is a unit vector \n//   (hence `x^2 + y^2 + z^2 = 1`) !!\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\n    //assert!(depth <= 14);\n    //assert!(-1.0 <= x && x <= 1.0);\n    //assert!(-1.0 <= y && y <= 1.0);\n    //assert!(-1.0 <= z && z <= 1.0);\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\n    // A f32 mantissa contains 23 bits.\n    // - it basically means that when storing (x, y) coordinates,\n    //   we can go as deep as depth 24 (or maybe 25)\n    \n    int nside = 1 << depth;\n    float half_nside = float(nside) * 0.5f;\n\n    float x_pm1 = xpm1(p.xy);\n    int q = quarter(p.xy);\n\n    int d0h = 0;\n    vec2 p_proj = vec2(0.f);\n    if (p.z > TRANSITION_Z) {\n        // North polar cap, Collignon projection.\n        // - set the origin to (PI/4, 0)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\n        d0h = q;\n    } else if (p.z < -TRANSITION_Z) {\n        // South polar cap, Collignon projection\n        // - set the origin to (PI/4, -PI/2)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\n        d0h = q + 8;\n    } else {\n        // Equatorial region, Cylindrical equal area projection\n        // - set the origin to (PI/4, 0)               if q = 2\n        // - set the origin to (PI/4, -PI/2)           if q = 0\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\n        // let zero_or_one = (x_cea as u8) & 1;\n        float y_pm1 = p.z * TRANSITION_Z_INV;\n        // |\\2/|\n        // .3X1.\n        // |/0\\|\n        int q01 = int(x_pm1 > y_pm1);  // 0/1\n        //debug_assert!(q01 == 0 || q01 == 1);\n        int q12 = int(x_pm1 >= -y_pm1); // 0\\1\n        //debug_assert!(q12 == 0 || q12 == 1);\n        int q03 = 1 - q12; // 1\\0\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\n        int q1 = q01 & q12; // = 1 if q1, 0 else\n        //debug_assert!( q1 == 0 ||  q1 == 1);\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \n        //let y_proj = y_pm1 + (q01 + q03) as f32;\n        p_proj = vec2(\n            x_pm1 - float(q01 + q12 - 1),\n            y_pm1 + float(q01 + q03)\n        );\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\n        d0h = ((q01 + q03) << 2) + ((q + q1) & 3);\n    }\n\n    // Coords inside the base cell\n    float x = (half_nside * (p_proj.x + p_proj.y));\n    float y = (half_nside * (p_proj.y - p_proj.x));\n    int i = int(x);\n    int j = int(y);\n\n    return HashDxDy(\n        (d0h << (depth << 1)) | ij2z(i, j),\n        x - float(i),\n        y - float(j)\n    );\n}\n\nuniform float opacity;\n\nTileColor get_tile_color(vec3 pos) {\n    HashDxDy result = hash_with_dxdy(0, pos.zxy);\n\n    int idx = result.idx;\n    vec2 uv = vec2(result.dy, result.dx);\n\n    Tile tile = textures_tiles[idx];\n\n    int idx_texture = tile.texture_idx >> 6;\n    int off = tile.texture_idx & 0x3F;\n    float idx_row = float(off >> 3); // in [0; 7]\n    float idx_col = float(off & 0x7); // in [0; 7]\n\n    vec2 offset = (vec2(idx_col, idx_row) + uv)*0.125;\n    vec3 UV = vec3(offset, float(idx_texture));\n\n    vec4 color = get_color_from_texture(UV);\n    color.a = mix(color.a, blank_color.a, float(tile.empty));\n    \n    return TileColor(tile, color, true);\n}\n\nconst float duration = 500.f; // 500ms\nuniform int max_depth; // max depth of the HiPS\n\nvoid main() {\n    vec3 frag_pos = normalize(out_vert_pos);\n\n    // Get the HEALPix cell idx and the uv in the texture\n    TileColor current_tile = get_tile_color(frag_pos);\n    float pixel_transparency = current_tile.color.a;\n    out_frag_color = vec4(current_tile.color.rgb, opacity * pixel_transparency);\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/grayscale_to_color.frag":
/*!*********************************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/grayscale_to_color.frag ***!
  \*********************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision highp int;\n\nin vec3 out_vert_pos;\nin vec2 pos_clip;\nin vec2 out_lonlat;\n\nout vec4 out_frag_color;\n\nuniform int user_action;\n\nstruct Tile {\n    int uniq; // Healpix cell\n    int texture_idx; // Index in the texture buffer\n    float start_time; // Absolute time that the load has been done in ms\n    int empty;\n};\n\nuniform int current_depth;\n\nuniform Tile textures_tiles[12];\n\nuniform float current_time; // current time in ms\nstruct TileColor {\n    Tile tile;\n    vec4 color;\n    bool found;\n};\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\nconst float TWICE_PI = 6.28318530718f;\nconst float PI = 3.141592653589793f;\nconst float FOUR_OVER_PI = 1.27323954474f;\nconst float TRANSITION_Z = 0.66666666666f;\nconst float TRANSITION_Z_INV = 1.5f;\n\nint quarter(vec2 p) {\n    int x_neg = int(p.x < 0.0f);\n    int y_neg = int(p.y < 0.0f);\n    int q = (x_neg + y_neg) | (y_neg << 1);\n    return q;\n}\n\nfloat xpm1(vec2 p) {\n    bool x_neg = (p.x < 0.0f);\n    //debug_assert!(x_neg <= 1);\n    bool y_neg = (p.y < 0.0f);\n    //debug_assert!(y_neg <= 1);\n    // The purpose it to have the same numerical precision for each base cell\n    // by avoiding subtraction by 1 or 3 or 5 or 7\n    float lon = atan(abs(p.y), abs(p.x));\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\n    float x02 = lon * FOUR_OVER_PI;\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\n        return 1.0f - x02;\n    } else {\n        return x02 - 1.0f;\n    }\n}\n\nfloat one_minus_z_pos(vec3 p) {\n    //debug_assert!(z > 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return 1.0f - p.z;\n}\n\nfloat one_minus_z_neg(vec3 p) {\n    //debug_assert!(z < 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\n        // 0.5 * d2 + 0.125 * d2 * d2\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return p.z + 1.0f;\n}\n\n// Z-Order curve projection.\nint ij2z(int i, int j) {\n    int i1 = i | (j << 16);\n\n    int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;\n    int i2 = i1 ^ j1 ^ (j1 << 8);\n\n    int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;\n    int i3 = i2 ^ j2 ^ (j2 << 4);\n\n    int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;\n    int i4 = i3 ^ j3 ^ (j3 << 2);\n\n    int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;\n    int i5 = i4 ^ j4 ^ (j4 << 1);\n\n    return i5;\n}\n\nstruct HashDxDy {\n    int idx;\n    float dx;\n    float dy;\n};\n\nuniform sampler2D ang2pixd;\nHashDxDy hash_with_dxdy2(vec2 radec) {\n    vec2 aa = vec2(radec.x/TWICE_PI + 1.0, (radec.y/PI) + 0.5);\n    vec3 v = texture(ang2pixd, aa).rgb;\n    return HashDxDy(\n        int(v.x * 255.0),\n        v.y,\n        v.z\n    );\n}\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\n// respect to the origin of the cell (South vertex).\n// # Inputs:\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\n// - `x`: in `[-1.0, 1.0]`\n// - `y`: in `[-1.0, 1.0]`\n// - `z`: in `[-1.0, 1.0]`\n// # Output\n// - the cell number (hash value) associated with the given position on the unit sphere,\n//   in `[0, 12*nside^2[`\n// - `dx`: the positional offset $\\in [0, 1[$ along the south-to-east axis\n// - `dy`: the positional offset $\\in [0, 1[$ along the south-to-west axis\n// # WARNING\n// - The function assumes, without checking, that the input vector is a unit vector \n//   (hence `x^2 + y^2 + z^2 = 1`) !!\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\n    //assert!(depth <= 14);\n    //assert!(-1.0 <= x && x <= 1.0);\n    //assert!(-1.0 <= y && y <= 1.0);\n    //assert!(-1.0 <= z && z <= 1.0);\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\n    // A f32 mantissa contains 23 bits.\n    // - it basically means that when storing (x, y) coordinates,\n    //   we can go as deep as depth 24 (or maybe 25)\n    \n    int nside = 1 << depth;\n    float half_nside = float(nside) * 0.5f;\n\n    float x_pm1 = xpm1(p.xy);\n    int q = quarter(p.xy);\n\n    int d0h = 0;\n    vec2 p_proj = vec2(0.f);\n    if (p.z > TRANSITION_Z) {\n        // North polar cap, Collignon projection.\n        // - set the origin to (PI/4, 0)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\n        d0h = q;\n    } else if (p.z < -TRANSITION_Z) {\n        // South polar cap, Collignon projection\n        // - set the origin to (PI/4, -PI/2)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\n        d0h = q + 8;\n    } else {\n        // Equatorial region, Cylindrical equal area projection\n        // - set the origin to (PI/4, 0)               if q = 2\n        // - set the origin to (PI/4, -PI/2)           if q = 0\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\n        // let zero_or_one = (x_cea as u8) & 1;\n        float y_pm1 = p.z * TRANSITION_Z_INV;\n        // |\\2/|\n        // .3X1.\n        // |/0\\|\n        int q01 = int(x_pm1 > y_pm1);  // 0/1\n        //debug_assert!(q01 == 0 || q01 == 1);\n        int q12 = int(x_pm1 >= -y_pm1); // 0\\1\n        //debug_assert!(q12 == 0 || q12 == 1);\n        int q03 = 1 - q12; // 1\\0\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\n        int q1 = q01 & q12; // = 1 if q1, 0 else\n        //debug_assert!( q1 == 0 ||  q1 == 1);\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \n        //let y_proj = y_pm1 + (q01 + q03) as f32;\n        p_proj = vec2(\n            x_pm1 - float(q01 + q12 - 1),\n            y_pm1 + float(q01 + q03)\n        );\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\n        d0h = ((q01 + q03) << 2) + ((q + q1) & 3);\n    }\n\n    // Coords inside the base cell\n    float x = (half_nside * (p_proj.x + p_proj.y));\n    float y = (half_nside * (p_proj.y - p_proj.x));\n    int i = int(x);\n    int j = int(y);\n\n    return HashDxDy(\n        (d0h << (depth << 1)) | ij2z(i, j),\n        x - float(i),\n        y - float(j)\n    );\n}\n\nuniform float opacity;\n\nTileColor get_tile_color(vec3 pos) {\n    HashDxDy result = hash_with_dxdy(0, pos.zxy);\n    int idx = result.idx;\n    vec2 uv = vec2(result.dy, result.dx);\n\n    Tile tile = textures_tiles[idx];\n\n    int idx_texture = tile.texture_idx >> 6;\n    int off = tile.texture_idx & 0x3F;\n    float idx_row = float(off >> 3); // in [0; 7]\n    float idx_col = float(off & 0x7); // in [0; 7]\n\n    vec2 offset = (vec2(idx_col, idx_row) + uv)*0.125;\n    vec3 UV = vec3(offset, float(idx_texture));\n\n    vec4 color = mix(get_color_from_grayscale_texture(UV), blank_color, float(tile.empty));\n    return TileColor(tile, color, true);\n}\n\nconst float duration = 500.f; // 500ms\nuniform int max_depth; // max depth of the HiPS\n\nvoid main() {\n    vec3 frag_pos = normalize(out_vert_pos);\n    // Get the HEALPix cell idx and the uv in the texture\n\n    TileColor current_tile = get_tile_color(frag_pos);\n    out_frag_color = current_tile.color;\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/grayscale_to_color_i.frag":
/*!***********************************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/grayscale_to_color_i.frag ***!
  \***********************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision highp int;\n\nin vec3 out_vert_pos;\nin vec2 pos_clip;\nin vec2 out_lonlat;\n\nout vec4 out_frag_color;\n\nuniform int user_action;\n\nstruct Tile {\n    int uniq; // Healpix cell\n    int texture_idx; // Index in the texture buffer\n    float start_time; // Absolute time that the load has been done in ms\n    int empty;\n};\n\nuniform int current_depth;\n\nuniform Tile textures_tiles[12];\n\nuniform float current_time; // current time in ms\nstruct TileColor {\n    Tile tile;\n    vec4 color;\n    bool found;\n};\n\n//const int MAX_NUM_TEX = 3;\nuniform isampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nfloat get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return float(texture(tex[0], uv.xy).r);\n    } else if (idx_texture == 1) {\n        return float(texture(tex[1], uv.xy).r);\n    } else if (idx_texture == 2) {\n        return float(texture(tex[2], uv.xy).r);\n    } else {\n        return 0.0;\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\nconst float TWICE_PI = 6.28318530718f;\nconst float PI = 3.141592653589793f;\nconst float FOUR_OVER_PI = 1.27323954474f;\nconst float TRANSITION_Z = 0.66666666666f;\nconst float TRANSITION_Z_INV = 1.5f;\n\nint quarter(vec2 p) {\n    int x_neg = int(p.x < 0.0f);\n    int y_neg = int(p.y < 0.0f);\n    int q = (x_neg + y_neg) | (y_neg << 1);\n    return q;\n}\n\nfloat xpm1(vec2 p) {\n    bool x_neg = (p.x < 0.0f);\n    //debug_assert!(x_neg <= 1);\n    bool y_neg = (p.y < 0.0f);\n    //debug_assert!(y_neg <= 1);\n    // The purpose it to have the same numerical precision for each base cell\n    // by avoiding subtraction by 1 or 3 or 5 or 7\n    float lon = atan(abs(p.y), abs(p.x));\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\n    float x02 = lon * FOUR_OVER_PI;\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\n        return 1.0f - x02;\n    } else {\n        return x02 - 1.0f;\n    }\n}\n\nfloat one_minus_z_pos(vec3 p) {\n    //debug_assert!(z > 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return 1.0f - p.z;\n}\n\nfloat one_minus_z_neg(vec3 p) {\n    //debug_assert!(z < 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\n        // 0.5 * d2 + 0.125 * d2 * d2\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return p.z + 1.0f;\n}\n\n// Z-Order curve projection.\nint ij2z(int i, int j) {\n    int i1 = i | (j << 16);\n\n    int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;\n    int i2 = i1 ^ j1 ^ (j1 << 8);\n\n    int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;\n    int i3 = i2 ^ j2 ^ (j2 << 4);\n\n    int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;\n    int i4 = i3 ^ j3 ^ (j3 << 2);\n\n    int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;\n    int i5 = i4 ^ j4 ^ (j4 << 1);\n\n    return i5;\n}\n\nstruct HashDxDy {\n    int idx;\n    float dx;\n    float dy;\n};\n\nuniform sampler2D ang2pixd;\nHashDxDy hash_with_dxdy2(vec2 radec) {\n    vec2 aa = vec2(radec.x/TWICE_PI + 1.0, (radec.y/PI) + 0.5);\n    vec3 v = texture(ang2pixd, aa).rgb;\n    return HashDxDy(\n        int(v.x * 255.0),\n        v.y,\n        v.z\n    );\n}\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\n// respect to the origin of the cell (South vertex).\n// # Inputs:\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\n// - `x`: in `[-1.0, 1.0]`\n// - `y`: in `[-1.0, 1.0]`\n// - `z`: in `[-1.0, 1.0]`\n// # Output\n// - the cell number (hash value) associated with the given position on the unit sphere,\n//   in `[0, 12*nside^2[`\n// - `dx`: the positional offset $\\in [0, 1[$ along the south-to-east axis\n// - `dy`: the positional offset $\\in [0, 1[$ along the south-to-west axis\n// # WARNING\n// - The function assumes, without checking, that the input vector is a unit vector \n//   (hence `x^2 + y^2 + z^2 = 1`) !!\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\n    //assert!(depth <= 14);\n    //assert!(-1.0 <= x && x <= 1.0);\n    //assert!(-1.0 <= y && y <= 1.0);\n    //assert!(-1.0 <= z && z <= 1.0);\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\n    // A f32 mantissa contains 23 bits.\n    // - it basically means that when storing (x, y) coordinates,\n    //   we can go as deep as depth 24 (or maybe 25)\n    \n    int nside = 1 << depth;\n    float half_nside = float(nside) * 0.5f;\n\n    float x_pm1 = xpm1(p.xy);\n    int q = quarter(p.xy);\n\n    int d0h = 0;\n    vec2 p_proj = vec2(0.f);\n    if (p.z > TRANSITION_Z) {\n        // North polar cap, Collignon projection.\n        // - set the origin to (PI/4, 0)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\n        d0h = q;\n    } else if (p.z < -TRANSITION_Z) {\n        // South polar cap, Collignon projection\n        // - set the origin to (PI/4, -PI/2)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\n        d0h = q + 8;\n    } else {\n        // Equatorial region, Cylindrical equal area projection\n        // - set the origin to (PI/4, 0)               if q = 2\n        // - set the origin to (PI/4, -PI/2)           if q = 0\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\n        // let zero_or_one = (x_cea as u8) & 1;\n        float y_pm1 = p.z * TRANSITION_Z_INV;\n        // |\\2/|\n        // .3X1.\n        // |/0\\|\n        int q01 = int(x_pm1 > y_pm1);  // 0/1\n        //debug_assert!(q01 == 0 || q01 == 1);\n        int q12 = int(x_pm1 >= -y_pm1); // 0\\1\n        //debug_assert!(q12 == 0 || q12 == 1);\n        int q03 = 1 - q12; // 1\\0\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\n        int q1 = q01 & q12; // = 1 if q1, 0 else\n        //debug_assert!( q1 == 0 ||  q1 == 1);\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \n        //let y_proj = y_pm1 + (q01 + q03) as f32;\n        p_proj = vec2(\n            x_pm1 - float(q01 + q12 - 1),\n            y_pm1 + float(q01 + q03)\n        );\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\n        d0h = ((q01 + q03) << 2) + ((q + q1) & 3);\n    }\n\n    // Coords inside the base cell\n    float x = (half_nside * (p_proj.x + p_proj.y));\n    float y = (half_nside * (p_proj.y - p_proj.x));\n    int i = int(x);\n    int j = int(y);\n\n    return HashDxDy(\n        (d0h << (depth << 1)) | ij2z(i, j),\n        x - float(i),\n        y - float(j)\n    );\n}\n\nuniform float opacity;\n\nTileColor get_tile_color(vec3 pos) {\n    HashDxDy result = hash_with_dxdy(0, pos.zxy);\n    int idx = result.idx;\n    vec2 uv = vec2(result.dy, result.dx);\n\n    Tile tile = textures_tiles[idx];\n\n    int idx_texture = tile.texture_idx >> 6;\n    int off = tile.texture_idx & 0x3F;\n    float idx_row = float(off >> 3); // in [0; 7]\n    float idx_col = float(off & 0x7); // in [0; 7]\n\n    vec2 offset = (vec2(idx_col, idx_row) + uv)*0.125;\n    vec3 UV = vec3(offset, float(idx_texture));\n\n    vec4 color = mix(get_color_from_grayscale_texture(UV), blank_color, float(tile.empty));\n    return TileColor(tile, color, true);\n}\n\nconst float duration = 500.f; // 500ms\nuniform int max_depth; // max depth of the HiPS\n\nvoid main() {\n    vec3 frag_pos = normalize(out_vert_pos);\n    // Get the HEALPix cell idx and the uv in the texture\n\n    TileColor current_tile = get_tile_color(frag_pos);\n    out_frag_color = current_tile.color;\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/grayscale_to_colormap.frag":
/*!************************************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/grayscale_to_colormap.frag ***!
  \************************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision highp int;\n\nin vec3 out_vert_pos;\nin vec2 pos_clip;\n\nout vec4 out_frag_color;\n\nuniform int user_action;\n\nstruct Tile {\n    int uniq; // Healpix cell\n    int texture_idx; // Index in the texture buffer\n    float start_time; // Absolute time that the load has been done in ms\n    int empty;\n};\n\nuniform int current_depth;\n\nuniform Tile textures_tiles[12];\n\nuniform float opacity;\nuniform float current_time; // current time in ms\nstruct TileColor {\n    Tile tile;\n    vec4 color;\n    bool found;\n};\n\n//const int MAX_NUM_TEX = 3;\nuniform sampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nvec4 get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return texture(tex[0], uv.xy);\n    } else if (idx_texture == 1) {\n        return texture(tex[1], uv.xy);\n    } else if (idx_texture == 2) {\n        return texture(tex[2], uv.xy);\n    } else {\n        return vec4(0.0, 1.0, 1.0, 1.0);\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nvec4 get_color_from_texture(vec3 UV) {\n    return get_pixels(UV);\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv).r;\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\nconst float TWICE_PI = 6.28318530718f;\nconst float PI = 3.141592653589793f;\nconst float FOUR_OVER_PI = 1.27323954474f;\nconst float TRANSITION_Z = 0.66666666666f;\nconst float TRANSITION_Z_INV = 1.5f;\n\nint quarter(vec2 p) {\n    int x_neg = int(p.x < 0.0f);\n    int y_neg = int(p.y < 0.0f);\n    int q = (x_neg + y_neg) | (y_neg << 1);\n    return q;\n}\n\nfloat xpm1(vec2 p) {\n    bool x_neg = (p.x < 0.0f);\n    //debug_assert!(x_neg <= 1);\n    bool y_neg = (p.y < 0.0f);\n    //debug_assert!(y_neg <= 1);\n    // The purpose it to have the same numerical precision for each base cell\n    // by avoiding subtraction by 1 or 3 or 5 or 7\n    float lon = atan(abs(p.y), abs(p.x));\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\n    float x02 = lon * FOUR_OVER_PI;\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\n        return 1.0f - x02;\n    } else {\n        return x02 - 1.0f;\n    }\n}\n\nfloat one_minus_z_pos(vec3 p) {\n    //debug_assert!(z > 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return 1.0f - p.z;\n}\n\nfloat one_minus_z_neg(vec3 p) {\n    //debug_assert!(z < 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\n        // 0.5 * d2 + 0.125 * d2 * d2\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return p.z + 1.0f;\n}\n\n// Z-Order curve projection.\nint ij2z(int i, int j) {\n    int i1 = i | (j << 16);\n\n    int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;\n    int i2 = i1 ^ j1 ^ (j1 << 8);\n\n    int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;\n    int i3 = i2 ^ j2 ^ (j2 << 4);\n\n    int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;\n    int i4 = i3 ^ j3 ^ (j3 << 2);\n\n    int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;\n    int i5 = i4 ^ j4 ^ (j4 << 1);\n\n    return i5;\n}\n\nstruct HashDxDy {\n    int idx;\n    float dx;\n    float dy;\n};\n\nuniform sampler2D ang2pixd;\nHashDxDy hash_with_dxdy2(vec2 radec) {\n    vec2 aa = vec2(radec.x/TWICE_PI + 1.0, (radec.y/PI) + 0.5);\n    vec3 v = texture(ang2pixd, aa).rgb;\n    return HashDxDy(\n        int(v.x * 255.0),\n        v.y,\n        v.z\n    );\n}\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\n// respect to the origin of the cell (South vertex).\n// # Inputs:\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\n// - `x`: in `[-1.0, 1.0]`\n// - `y`: in `[-1.0, 1.0]`\n// - `z`: in `[-1.0, 1.0]`\n// # Output\n// - the cell number (hash value) associated with the given position on the unit sphere,\n//   in `[0, 12*nside^2[`\n// - `dx`: the positional offset $\\in [0, 1[$ along the south-to-east axis\n// - `dy`: the positional offset $\\in [0, 1[$ along the south-to-west axis\n// # WARNING\n// - The function assumes, without checking, that the input vector is a unit vector \n//   (hence `x^2 + y^2 + z^2 = 1`) !!\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\n    //assert!(depth <= 14);\n    //assert!(-1.0 <= x && x <= 1.0);\n    //assert!(-1.0 <= y && y <= 1.0);\n    //assert!(-1.0 <= z && z <= 1.0);\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\n    // A f32 mantissa contains 23 bits.\n    // - it basically means that when storing (x, y) coordinates,\n    //   we can go as deep as depth 24 (or maybe 25)\n    \n    int nside = 1 << depth;\n    float half_nside = float(nside) * 0.5f;\n\n    float x_pm1 = xpm1(p.xy);\n    int q = quarter(p.xy);\n\n    int d0h = 0;\n    vec2 p_proj = vec2(0.f);\n    if (p.z > TRANSITION_Z) {\n        // North polar cap, Collignon projection.\n        // - set the origin to (PI/4, 0)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\n        d0h = q;\n    } else if (p.z < -TRANSITION_Z) {\n        // South polar cap, Collignon projection\n        // - set the origin to (PI/4, -PI/2)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\n        d0h = q + 8;\n    } else {\n        // Equatorial region, Cylindrical equal area projection\n        // - set the origin to (PI/4, 0)               if q = 2\n        // - set the origin to (PI/4, -PI/2)           if q = 0\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\n        // let zero_or_one = (x_cea as u8) & 1;\n        float y_pm1 = p.z * TRANSITION_Z_INV;\n        // |\\2/|\n        // .3X1.\n        // |/0\\|\n        int q01 = int(x_pm1 > y_pm1);  // 0/1\n        //debug_assert!(q01 == 0 || q01 == 1);\n        int q12 = int(x_pm1 >= -y_pm1); // 0\\1\n        //debug_assert!(q12 == 0 || q12 == 1);\n        int q03 = 1 - q12; // 1\\0\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\n        int q1 = q01 & q12; // = 1 if q1, 0 else\n        //debug_assert!( q1 == 0 ||  q1 == 1);\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \n        //let y_proj = y_pm1 + (q01 + q03) as f32;\n        p_proj = vec2(\n            x_pm1 - float(q01 + q12 - 1),\n            y_pm1 + float(q01 + q03)\n        );\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\n        d0h = ((q01 + q03) << 2) + ((q + q1) & 3);\n    }\n\n    // Coords inside the base cell\n    float x = (half_nside * (p_proj.x + p_proj.y));\n    float y = (half_nside * (p_proj.y - p_proj.x));\n    int i = int(x);\n    int j = int(y);\n\n    return HashDxDy(\n        (d0h << (depth << 1)) | ij2z(i, j),\n        x - float(i),\n        y - float(j)\n    );\n}\n\nTileColor get_tile_color(vec3 pos) {\n    HashDxDy result = hash_with_dxdy(0, pos.zxy);\n\n    int idx = result.idx;\n    vec2 uv = vec2(result.dy, result.dx);\n\n    Tile tile = textures_tiles[idx];\n\n    int idx_texture = tile.texture_idx >> 6;\n    int off = tile.texture_idx & 0x3F;\n    float idx_row = float(off >> 3); // in [0; 7]\n    float idx_col = float(off & 0x7); // in [0; 7]\n\n    vec2 offset = (vec2(idx_col, idx_row) + uv)*0.125;\n    vec3 UV = vec3(offset, float(idx_texture));\n\n    vec4 color = mix(get_colormap_from_grayscale_texture(UV), blank_color, float(tile.empty));\n    return TileColor(tile, color, true);\n}\n\nconst float duration = 500.f; // 500ms\nuniform int max_depth; // max depth of the HiPS\n\nvoid main() {\n    vec3 frag_pos = normalize(out_vert_pos);\n    // Get the HEALPix cell idx and the uv in the texture\n\n    TileColor current_tile = get_tile_color(frag_pos);\n    out_frag_color = current_tile.color;\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/grayscale_to_colormap_i.frag":
/*!**************************************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/grayscale_to_colormap_i.frag ***!
  \**************************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp sampler2D;\nprecision highp isampler2D;\nprecision highp int;\n\nin vec3 out_vert_pos;\nin vec2 pos_clip;\nin vec2 out_lonlat;\n\nout vec4 out_frag_color;\n\nuniform int user_action;\n\nstruct Tile {\n    int uniq; // Healpix cell\n    int texture_idx; // Index in the texture buffer\n    float start_time; // Absolute time that the load has been done in ms\n    int empty;\n};\n\nuniform int current_depth;\n\nuniform Tile textures_tiles[12];\n\nuniform float opacity;\nuniform float current_time; // current time in ms\nstruct TileColor {\n    Tile tile;\n    vec4 color;\n    bool found;\n};\n\n//const int MAX_NUM_TEX = 3;\nuniform isampler2D tex[3];\nuniform int num_tex;\n\nuniform float scale;\nuniform float offset;\nuniform float blank;\n\nuniform float min_value;\nuniform float max_value;\nuniform int H;\n\nuniform float size_tile_uv;\n\nuniform float tex_storing_integers;\nuniform int tex_storing_fits;\n\n// Blue & Pastel & Red\nfloat colormap_red(float x) {\n    if (x < 0.1131206452846527) {\n        return (-9.40943766883858E+02 * x - 1.84146720562529E+02) * x + 3.28713709677420E+01;\n    } else if (x < 0.5116005837917328) {\n        return 0.0;\n    } else if (x < 0.5705677568912506) {\n        return (-2.22507913165263E+03 * x + 2.76053354341733E+03) * x - 8.29909138655453E+02;\n    } else if (x < 0.622047244) {\n        return (-1.84774532967032E+04 * x + 2.30647002747253E+04) * x - 7.12389120879120E+03;\n    } else if (x < 0.7922459542751312) {\n        return ((((1.29456468589020E+06 * x - 4.64095889653844E+06) * x + 6.62951004830418E+06) * x - 4.71587036142377E+06) * x + 1.67048886368434E+06) * x - 2.35682532934682E+05;\n    } else {\n        return 3.34889230769210E+02 * x - 1.41006123680226E+02;\n    }\n}\n\nfloat colormap_green(float x) {\n    if (x < 0.114394336938858) {\n        return 0.0;\n    } else if (x < 0.4417250454425812) {\n        return (9.43393359191585E+02 * x + 1.86774918014536E+02) * x - 3.37113020096108E+01;\n    } else if (x < 0.4964917968308496) {\n        return 3.11150000000070E+02 * x + 9.54249999999731E+01;\n    } else if (x < 0.6259051214039278) {\n        return -1.03272635599706E+03 * x + 7.62648586707481E+02;\n    } else if (x < 0.8049814403057098) {\n        return -2.92799028677160E+02 * x + 2.99524283071235E+02;\n    } else {\n        return (1.34145201311283E+03 * x - 2.75066701126586E+03) * x + 1.40880802982723E+03;\n    }\n}\n\nfloat colormap_blue(float x) {\n    if (x < 0.4424893036638088) {\n        return 3.09636968527514E+02 * x + 9.62203074056821E+01;\n    } else if (x < 0.5) {\n        return -4.59921428571535E+02 * x + 4.36741666666678E+02;\n    } else if (x < 0.5691165986930345) {\n        return -1.81364912280674E+03 * x + 1.05392982456125E+03;\n    } else if (x < 0.6279306709766388) {\n        return 1.83776470588197E+02 * x - 8.28382352940910E+01;\n    } else {\n        return ((-1.14087926835422E+04 * x + 2.47091243363548E+04) * x - 1.80428756181930E+04) * x + 4.44421976986281E+03;\n    }\n}\n\nvec4 bluepastelred_f(float x) {\n    float r = clamp(colormap_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(colormap_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(colormap_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n\n// Red\nfloat c_red(float x) {\n    return 1.448953446096850 * x - 5.02253539008443e-1;\n}\n\nfloat c_green(float x) {\n    return 1.889376646180860 * x - 2.272028094820020e2;\n}\n\nfloat c_blue(float x) {\n    return 3.92613636363636 * x - 7.46528409090909e+2;\n}\n\nvec4 red_f(float x) {\n    float t = x * 255.0;\n    float r = clamp(c_red(t) / 255.0, 0.0, 1.0);\n    float g = clamp(c_green(t) / 255.0, 0.0, 1.0);\n    float b = clamp(c_blue(t) / 255.0, 0.0, 1.0);\n\n    return vec4(r, g, b, 1.0);\n}\n// Gray\nvec4 blackw_f(float x) {\n    float d = clamp(x, 0.0, 1.0);\n    return vec4(d, d, d, 1.0);\n}\n// IDLCBGnBu\nfloat cbgnbu_red(float x) {\n    float v = ((((-2.83671754639782E+03 * x + 6.51753994553536E+03) * x - 5.00110948171466E+03) * x + 1.30359712298773E+03) * x - 2.89958300810074E+02) * x + 2.48458039402758E+02;\n    if (v < 8.0) {\n        return 8.0;\n    } else {\n        return v;\n    }\n}\n\nfloat cbgnbu_green(float x) {\n    return (((((-1.36304822155833E+03 * x + 4.37691418182849E+03) * x - 5.01802019417285E+03) * x + 2.39971481269598E+03) * x - 5.65401491984724E+02) * x - 1.48189675724133E+01) * x + 2.50507618187374E+02;\n}\n\nfloat cbgnbu_blue(float x) {\n    if (x < 0.3756393599187693) {\n        return (9.62948273917718E+01 * x - 1.96136874142438E+02) * x + 2.41033490809633E+02;\n    } else if (x < 0.6215448666633865) {\n        return 1.21184043778803E+02 * x + 1.35422939068100E+02;\n    } else if (x < 0.8830064316178203) {\n        return -1.53052165744713E+02 * x + 3.05873047350666E+02;\n    } else {\n        return -3.49468965517114E+02 * x + 4.79310344827486E+02;\n    }\n}\n\nvec4 cbgnbu_f(float x) {\n    float r = clamp(cbgnbu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbgnbu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbgnbu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBYIGnBu\nfloat CBYIGnBu_red(float x) {\n    if (x < 0.2523055374622345) {\n        return (-5.80630393656902E+02 * x - 8.20261301968494E+01) * x + 2.53829637096771E+02;\n    } else if (x < 0.6267540156841278) {\n        return (((-4.07958939010649E+03 * x + 8.13296992114899E+03) * x - 5.30725139102868E+03) * x + 8.58474724851723E+02) * x + 2.03329669375107E+02;\n    } else if (x < 0.8763731146612115) {\n        return 3.28717357910916E+01 * x + 8.82117255504255E+00;\n    } else {\n        return -2.29186583577707E+02 * x + 2.38482038123159E+02;\n    }\n}\n\nfloat CBYIGnBu_green(float x) {\n    if (x < 0.4578040540218353) {\n        return ((4.49001704856054E+02 * x - 5.56217473429394E+02) * x + 2.09812296466262E+01) * x + 2.52987561849833E+02;\n    } else {\n        return ((1.28031059709139E+03 * x - 2.71007279113343E+03) * x + 1.52699334501816E+03) * x - 6.48190622715140E+01;\n    }\n}\n\nfloat CBYIGnBu_blue(float x) {\n    if (x < 0.1239372193813324) {\n        return (1.10092779856059E+02 * x - 3.41564374557536E+02) * x + 2.17553885630496E+02;\n    } else if (x < 0.7535201013088226) {\n        return ((((3.86204601547122E+03 * x - 8.79126469446648E+03) * x + 6.80922226393264E+03) * x - 2.24007302003438E+03) * x + 3.51344388740066E+02) * x + 1.56774650431396E+02;\n    } else {\n        return (((((-7.46693234167480E+06 * x + 3.93327773566702E+07) * x - 8.61050867447971E+07) * x + 1.00269040461745E+08) * x - 6.55080846112976E+07) * x + 2.27664953009389E+07) * x - 3.28811994253461E+06;\n    }\n}\n\nvec4 CBYIGnBu_f(float x) {\n    float r = clamp(CBYIGnBu_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(CBYIGnBu_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(CBYIGnBu_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\n// IDLCBBrBG\nfloat cbbrbg_red(float x) {\n    if (x < 0.4128910005092621) {\n        return (-6.30796693758704E+02 * x + 6.59139629181867E+02) * x + 8.16592339699109E+01;\n    } else if (x < 0.5004365747118258) {\n        return -1.99292307692284E+01 * x + 2.54503076923075E+02;\n    } else if (x < 0.6000321805477142) {\n        return -4.46903540903651E+02 * x + 4.68176638176691E+02;\n    } else {\n        return ((2.43537534073204E+03 * x - 5.03831150657605E+03) * x + 2.73595321475367E+03) * x - 1.53778856560153E+02;\n    }\n}\n\nfloat cbbrbg_green(float x) {\n    if (x < 0.3067105114459991) {\n        return (((((-1.43558931121826E+06 * x + 1.21789289489746E+06) * x - 3.88754308517456E+05) * x + 5.87745165729522E+04) * x - 3.61237992835044E+03) * x + 4.00139210969209E+02) * x + 4.80612502318691E+01;\n    } else if (x < 0.4045854562297116) {\n        return 3.64978461538455E+02 * x + 8.50984615384636E+01;\n    } else if (x < 0.5035906732082367) {\n        return 1.25827692307720E+02 * x + 1.81855384615367E+02;\n    } else {\n        return ((((-2.83948052403926E+04 * x + 1.08768529946603E+05) * x - 1.62569302478295E+05) * x + 1.17919256227845E+05) * x - 4.16776268978779E+04) * x + 6.01529271177582E+03;\n    }\n}\n\nfloat cbbrbg_blue(float x) {\n    if (x < 0.1012683545126085) {\n        return 5.85993431855501E+01 * x + 4.56403940886700E+00;\n    } else if (x < 0.2050940692424774) {\n        return 3.51072173913048E+02 * x - 2.50542028985514E+01;\n    } else if (x < 0.5022056996822357) {\n        return (-7.65121475963620E+02 * x + 1.20827362856208E+03) * x - 1.68677387505814E+02;\n    } else if (x < 0.5970333516597748) {\n        return -1.62299487179500E+02 * x + 3.26660512820525E+02;\n    } else {\n        return ((1.27993125066091E+03 * x - 3.19799978871341E+03) * x + 2.16242391471484E+03) * x - 1.93738146367890E+02;\n    }\n}\n\nvec4 cbbrbg_f(float x) {\n    float r = clamp(cbbrbg_red(x) / 255.0, 0.0, 1.0);\n    float g = clamp(cbbrbg_green(x) / 255.0, 0.0, 1.0);\n    float b = clamp(cbbrbg_blue(x) / 255.0, 0.0, 1.0);\n    return vec4(r, g, b, 1.0);\n}\nuniform int colormap;\n/*\nBlackWhiteLinear = 0,\nRedTemperature = 1,\nIDLCBGnBu = 2,\nIDLCBYIGnBu = 3,\nBluePastelRed = 4,\nIDLCBBrBG = 5,\n*/\nvec4 colormap_f(float x) {\n    // BlackWhiteLinear = 0,\n    if (colormap == 0) {\n        return blackw_f(x);\n    // RedTemperature = 1,\n    } else if (colormap == 1) {\n        return red_f(x);\n    // IDLCBGnBu = 2,\n    } else if (colormap == 2) {\n        return cbgnbu_f(x);\n    // IDLCBYIGnBu = 3,\n    } else if (colormap == 3) {\n        return CBYIGnBu_f(x);\n    // BluePastelRed = 4,\n    } else if (colormap == 4) {\n        return bluepastelred_f(x);\n    // IDLCBBrBG = 5,\n    } else {\n        return cbbrbg_f(x);\n    }\n}\nfloat linear_f(float x, float min_value, float max_value) {\n    return clamp((x - min_value)/(max_value - min_value), 0.0, 1.0);\n}\n\nfloat sqrt_f(float x, float min_value, float max_value) {\n    float a = linear_f(x, min_value, max_value);\n    return sqrt(a);\n}\n\nfloat log_f(float x, float min_value, float max_value) {\n    float y = linear_f(x, min_value, max_value);\n    float a = 1000.0;\n    return log(a*y + 1.0)/log(a);\n}\n\nfloat asinh_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return asinh(10.0*d)/3.0;\n}\n\nfloat pow2_f(float x, float min_value, float max_value) {\n    float d = linear_f(x, min_value, max_value);\n    return d*d;\n}\n\nfloat transfer_func(int H, float x, float min_value, float max_value) {\n    if (H == 0) {\n        return linear_f(x, min_value, max_value);\n    } else if (H == 1) {\n        return sqrt_f(x, min_value, max_value);\n    } else if (H == 2) {\n        return log_f(x, min_value, max_value);\n    } else if (H == 3) {\n        return asinh_f(x, min_value, max_value);\n    } else {\n        return pow2_f(x, min_value, max_value);\n    }\n}\n\nfloat get_pixels(vec3 uv) {\n    int idx_texture = int(uv.z);\n    if (idx_texture == 0) {\n        return float(texture(tex[0], uv.xy).r);\n    } else if (idx_texture == 1) {\n        return float(texture(tex[1], uv.xy).r);\n    } else if (idx_texture == 2) {\n        return float(texture(tex[2], uv.xy).r);\n    } else {\n        return 0.0;\n    }\n}\n\nvec3 reverse_uv(vec3 uv) {\n    uv.y = size_tile_uv + 2.0*size_tile_uv*floor(uv.y / size_tile_uv) - uv.y;\n\n    return uv;\n}\n\nuniform vec4 blank_color;\n\nvec4 get_colormap_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return colormap_f(h);\n    }\n}\n\nuniform vec3 C;\nuniform float K;\nvec4 get_color_from_grayscale_texture(vec3 UV) {\n    vec3 uv = UV;\n    // FITS data pixels are reversed along the y axis\n    if (tex_storing_fits == 1) {\n        uv = reverse_uv(uv);\n    }\n\n    float x = get_pixels(uv);\n    if (x == blank) {\n        return blank_color;\n    } else {\n        float alpha = x * scale + offset;\n        float h = transfer_func(H, alpha, min_value, max_value);\n\n        return vec4(C * K * h, 1.0);\n    }\n}\nconst float TWICE_PI = 6.28318530718f;\nconst float PI = 3.141592653589793f;\nconst float FOUR_OVER_PI = 1.27323954474f;\nconst float TRANSITION_Z = 0.66666666666f;\nconst float TRANSITION_Z_INV = 1.5f;\n\nint quarter(vec2 p) {\n    int x_neg = int(p.x < 0.0f);\n    int y_neg = int(p.y < 0.0f);\n    int q = (x_neg + y_neg) | (y_neg << 1);\n    return q;\n}\n\nfloat xpm1(vec2 p) {\n    bool x_neg = (p.x < 0.0f);\n    //debug_assert!(x_neg <= 1);\n    bool y_neg = (p.y < 0.0f);\n    //debug_assert!(y_neg <= 1);\n    // The purpose it to have the same numerical precision for each base cell\n    // by avoiding subtraction by 1 or 3 or 5 or 7\n    float lon = atan(abs(p.y), abs(p.x));\n    //debug_assert!(0.0 <= lon && lon <= PI / 2.0);\n    float x02 = lon * FOUR_OVER_PI;\n    //debug_assert!(0.0 <= x02 && x02 <= 2.0);\n    if (x_neg != y_neg) { // Could be replaced by a sign copy from (x_neg ^ y_neg) << 32\n        return 1.0f - x02;\n    } else {\n        return x02 - 1.0f;\n    }\n}\n\nfloat one_minus_z_pos(vec3 p) {\n    //debug_assert!(z > 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n\n    if (d2 < 1e-1f) { // <=> dec > 84.27 deg\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return 1.0f - p.z;\n}\n\nfloat one_minus_z_neg(vec3 p) {\n    //debug_assert!(z < 0.0);\n    float d2 = dot(p.xy, p.xy); // z = sqrt(1 - d2) AND sqrt(1 - x) = 1 - x / 2 - x^2 / 8 - x^3 / 16 - 5 x^4/128 - 7 * x^5/256\n    if (d2 < 1e-1f) { // <=> dec < -84.27 deg\n        // 0.5 * d2 + 0.125 * d2 * d2\n        return d2 * (0.5f + d2 * (0.125f + d2 * (0.0625f + d2 * (0.0390625f + d2 * 0.02734375f))));\n    }\n    return p.z + 1.0f;\n}\n\n// Z-Order curve projection.\nint ij2z(int i, int j) {\n    int i1 = i | (j << 16);\n\n    int j1 = (i1 ^ (i1 >> 8)) & 0x0000FF00;\n    int i2 = i1 ^ j1 ^ (j1 << 8);\n\n    int j2 = (i2 ^ (i2 >> 4)) & 0x00F000F0;\n    int i3 = i2 ^ j2 ^ (j2 << 4);\n\n    int j3 = (i3 ^ (i3 >> 2)) & 0x0C0C0C0C;\n    int i4 = i3 ^ j3 ^ (j3 << 2);\n\n    int j4 = (i4 ^ (i4 >> 1)) & 0x22222222;\n    int i5 = i4 ^ j4 ^ (j4 << 1);\n\n    return i5;\n}\n\nstruct HashDxDy {\n    int idx;\n    float dx;\n    float dy;\n};\n\nuniform sampler2D ang2pixd;\nHashDxDy hash_with_dxdy2(vec2 radec) {\n    vec2 aa = vec2(radec.x/TWICE_PI + 1.0, (radec.y/PI) + 0.5);\n    vec3 v = texture(ang2pixd, aa).rgb;\n    return HashDxDy(\n        int(v.x * 255.0),\n        v.y,\n        v.z\n    );\n}\n// Returns the cell number (hash value) associated with the given position on the unit sphere, \n// together with the offset `(dx, dy)` on the Euclidean plane of the projected position with\n// respect to the origin of the cell (South vertex).\n// # Inputs:\n// - `depth` in `[0, 14]` (so that and HEALPix cell number can be stored on an unsigned integer)\n// - `x`: in `[-1.0, 1.0]`\n// - `y`: in `[-1.0, 1.0]`\n// - `z`: in `[-1.0, 1.0]`\n// # Output\n// - the cell number (hash value) associated with the given position on the unit sphere,\n//   in `[0, 12*nside^2[`\n// - `dx`: the positional offset $\\in [0, 1[$ along the south-to-east axis\n// - `dy`: the positional offset $\\in [0, 1[$ along the south-to-west axis\n// # WARNING\n// - The function assumes, without checking, that the input vector is a unit vector \n//   (hence `x^2 + y^2 + z^2 = 1`) !!\n// - Operations being made on simple precision float, the precision is lower than `~0.2 arcsec` only!!\n// - At depth 13, the precision on `(dx, dy)` is better than `(1/512, 1/512)`, i.e. 2e-3.\nHashDxDy hash_with_dxdy(int depth, vec3 p) {\n    //assert!(depth <= 14);\n    //assert!(-1.0 <= x && x <= 1.0);\n    //assert!(-1.0 <= y && y <= 1.0);\n    //assert!(-1.0 <= z && z <= 1.0);\n    //debug_assert!(1.0 - (x * x + y * y + z * z) < 1e-5);\n    // A f32 mantissa contains 23 bits.\n    // - it basically means that when storing (x, y) coordinates,\n    //   we can go as deep as depth 24 (or maybe 25)\n    \n    int nside = 1 << depth;\n    float half_nside = float(nside) * 0.5f;\n\n    float x_pm1 = xpm1(p.xy);\n    int q = quarter(p.xy);\n\n    int d0h = 0;\n    vec2 p_proj = vec2(0.f);\n    if (p.z > TRANSITION_Z) {\n        // North polar cap, Collignon projection.\n        // - set the origin to (PI/4, 0)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_pos(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, 2.0f - sqrt_3_one_min_z);\n        d0h = q;\n    } else if (p.z < -TRANSITION_Z) {\n        // South polar cap, Collignon projection\n        // - set the origin to (PI/4, -PI/2)\n        float sqrt_3_one_min_z = sqrt(3.0f * one_minus_z_neg(p));\n        p_proj = vec2(x_pm1 * sqrt_3_one_min_z, sqrt_3_one_min_z);\n        d0h = q + 8;\n    } else {\n        // Equatorial region, Cylindrical equal area projection\n        // - set the origin to (PI/4, 0)               if q = 2\n        // - set the origin to (PI/4, -PI/2)           if q = 0\n        // - set the origin to (0, -TRANSITION_LAT)    if q = 3\n        // - set the origin to (PI/2, -TRANSITION_LAT) if q = 1\n        // let zero_or_one = (x_cea as u8) & 1;\n        float y_pm1 = p.z * TRANSITION_Z_INV;\n        // |\\2/|\n        // .3X1.\n        // |/0\\|\n        int q01 = int(x_pm1 > y_pm1);  // 0/1\n        //debug_assert!(q01 == 0 || q01 == 1);\n        int q12 = int(x_pm1 >= -y_pm1); // 0\\1\n        //debug_assert!(q12 == 0 || q12 == 1);\n        int q03 = 1 - q12; // 1\\0\n        //let q13 = q01 ^ q12; debug_assert!(q13 == 0 || q13 == 1);\n        int q1 = q01 & q12; // = 1 if q1, 0 else\n        //debug_assert!( q1 == 0 ||  q1 == 1);\n        // x: xcea - 0 if q3 | xcea - 2 if q1 | xcea - 1 if q0 or q2\n        //let x_proj = x_pm1 - ((q01 + q12) as i8 - 1) as f32;\n        // y: y - 0 if q2 | y - 1 if q1 or q3 | y - 2 if q0 \n        //let y_proj = y_pm1 + (q01 + q03) as f32;\n        p_proj = vec2(\n            x_pm1 - float(q01 + q12 - 1),\n            y_pm1 + float(q01 + q03)\n        );\n        // d0h: +8 if q0 | +4 if q3 | +5 if q1\n        d0h = ((q01 + q03) << 2) + ((q + q1) & 3);\n    }\n\n    // Coords inside the base cell\n    float x = (half_nside * (p_proj.x + p_proj.y));\n    float y = (half_nside * (p_proj.y - p_proj.x));\n    int i = int(x);\n    int j = int(y);\n\n    return HashDxDy(\n        (d0h << (depth << 1)) | ij2z(i, j),\n        x - float(i),\n        y - float(j)\n    );\n}\n\nTileColor get_tile_color(vec3 pos) {\n    HashDxDy result = hash_with_dxdy(0, pos.zxy);\n\n    int idx = result.idx;\n\n    vec2 uv = vec2(result.dy, result.dx);\n\n    Tile tile = textures_tiles[idx];\n\n    int idx_texture = tile.texture_idx >> 6;\n    int off = tile.texture_idx & 0x3F;\n    float idx_row = float(off >> 3); // in [0; 7]\n    float idx_col = float(off & 0x7); // in [0; 7]\n\n    vec2 offset = (vec2(idx_col, idx_row) + uv)*0.125;\n    vec3 UV = vec3(offset, float(idx_texture));\n\n    vec4 color = mix(get_colormap_from_grayscale_texture(UV), blank_color, float(tile.empty));\n    return TileColor(tile, color, true);\n}\n\nconst float duration = 500.f; // 500ms\nuniform int max_depth; // max depth of the HiPS\n\nvoid main() {\n    vec3 frag_pos = normalize(out_vert_pos);\n    // Get the HEALPix cell idx and the uv in the texture\n\n    TileColor current_tile = get_tile_color(frag_pos);\n    out_frag_color = current_tile.color;\n    out_frag_color.a = out_frag_color.a * opacity;\n}"

/***/ }),

/***/ "./src/core/src/shaders/hips/raytracer/raytracer.vert":
/*!************************************************************!*\
  !*** ./src/core/src/shaders/hips/raytracer/raytracer.vert ***!
  \************************************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision highp float;\nprecision highp int;\n\nlayout (location = 0) in vec2 pos_clip_space;\nlayout (location = 1) in vec2 lonlat;\nlayout (location = 2) in vec3 pos_world_space;\n\nout vec3 out_vert_pos;\nout vec2 out_lonlat;\n\nuniform mat4 model;\nuniform vec2 ndc_to_clip;\nuniform float czf;\n\nvoid main() {\n    gl_Position = vec4(pos_clip_space / (ndc_to_clip * czf), 0.0, 1.0);\n    //out_vert_pos = vec3(inverse(gal_to_icrs) * model * vec4(pos_world_space, 1.f));\n    out_vert_pos = vec3(model * vec4(pos_world_space, 1.f));\n    out_lonlat = vec2(atan(out_vert_pos.x, out_vert_pos.z), asin(out_vert_pos.y));\n}"

/***/ }),

/***/ "./src/core/src/shaders/misc/text.frag":
/*!*********************************************!*\
  !*** ./src/core/src/shaders/misc/text.frag ***!
  \*********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nprecision lowp sampler2DArray;\n\nuniform vec4 text_color;\nuniform sampler2DArray font_textures;\n\nin vec3 out_uv;\nout vec4 color;\n\nvoid main() {\n    vec3 uv = vec3(out_uv.x, 1.f - out_uv.y, out_uv.z);\n    vec4 mask = texture(font_textures, uv);\n    color = text_color * mask;\n    //color = vec4(1.0, 0.0, 0.0, 1.0);\n}"

/***/ }),

/***/ "./src/core/src/shaders/misc/text.vert":
/*!*********************************************!*\
  !*** ./src/core/src/shaders/misc/text.vert ***!
  \*********************************************/
/*! no static exports found */
/***/ (function(module, exports) {

module.exports = "#version 300 es\nprecision lowp float;\nprecision lowp sampler2DArray;\n\nlayout (location = 0) in vec2 pos;\nlayout (location = 1) in vec2 uv;\n// Per instance attributes\nlayout (location = 2) in vec2 center_letter;\nlayout (location = 3) in vec2 size_letter;\nlayout (location = 4) in vec2 pos_uv;\nlayout (location = 5) in vec2 size_uv;\nlayout (location = 6) in float idx_page;\n\nout vec3 out_uv;\n\nuniform vec2 window_size;\nuniform float scaling;\n\nvec2 screen_to_ndc(vec2 p) {\n    // Change of origin\n    vec2 origin = p - window_size/2.0;\n\n    // Scale to fit in [-1, 1]\n    return vec2(2.0 * (origin.x/window_size.x), -2.0 * (origin.y/window_size.y));\n}\n\nvoid main() {\n    vec2 ndc_pos = screen_to_ndc(center_letter + pos*32.0);\n\n    gl_Position = vec4(ndc_pos, 0.f, 1.f);\n    out_uv = vec3(uv, idx_page);\n}"

/***/ }),

/***/ "./src/js/Aladin.js":
/*!**************************!*\
  !*** ./src/js/Aladin.js ***!
  \**************************/
/*! exports provided: Aladin */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Aladin", function() { return Aladin; });
/* harmony import */ var _View_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./View.js */ "./src/js/View.js");
/* harmony import */ var _MOC_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./MOC.js */ "./src/js/MOC.js");
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _Overlay_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./Overlay.js */ "./src/js/Overlay.js");
/* harmony import */ var _Footprint_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./Footprint.js */ "./src/js/Footprint.js");
/* harmony import */ var _Circle_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./Circle.js */ "./src/js/Circle.js");
/* harmony import */ var _Ellipse_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./Ellipse.js */ "./src/js/Ellipse.js");
/* harmony import */ var _Polyline_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./Polyline.js */ "./src/js/Polyline.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _Logger_js__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ./Logger.js */ "./src/js/Logger.js");
/* harmony import */ var _Catalog_js__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! ./Catalog.js */ "./src/js/Catalog.js");
/* harmony import */ var _ProgressiveCat_js__WEBPACK_IMPORTED_MODULE_11__ = __webpack_require__(/*! ./ProgressiveCat.js */ "./src/js/ProgressiveCat.js");
/* harmony import */ var _Sesame_js__WEBPACK_IMPORTED_MODULE_12__ = __webpack_require__(/*! ./Sesame.js */ "./src/js/Sesame.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _MeasurementTable_js__WEBPACK_IMPORTED_MODULE_14__ = __webpack_require__(/*! ./MeasurementTable.js */ "./src/js/MeasurementTable.js");
/* harmony import */ var _Location_js__WEBPACK_IMPORTED_MODULE_15__ = __webpack_require__(/*! ./Location.js */ "./src/js/Location.js");
/* harmony import */ var _Source_js__WEBPACK_IMPORTED_MODULE_16__ = __webpack_require__(/*! ./Source.js */ "./src/js/Source.js");
/* harmony import */ var _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_17__ = __webpack_require__(/*! ./HpxImageSurvey.js */ "./src/js/HpxImageSurvey.js");
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_18__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
/* harmony import */ var _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__ = __webpack_require__(/*! ./CooConversion.js */ "./src/js/CooConversion.js");
/* harmony import */ var _Color_js__WEBPACK_IMPORTED_MODULE_20__ = __webpack_require__(/*! ./Color.js */ "./src/js/Color.js");
/* harmony import */ var _ColorMap_js__WEBPACK_IMPORTED_MODULE_21__ = __webpack_require__(/*! ./ColorMap.js */ "./src/js/ColorMap.js");
/* harmony import */ var _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__ = __webpack_require__(/*! ./URLBuilder.js */ "./src/js/URLBuilder.js");
/* harmony import */ var _HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_23__ = __webpack_require__(/*! ./HiPSDefinition.js */ "./src/js/HiPSDefinition.js");
/* harmony import */ var _DiscoveryTree_js__WEBPACK_IMPORTED_MODULE_24__ = __webpack_require__(/*! ./DiscoveryTree.js */ "./src/js/DiscoveryTree.js");
/* harmony import */ var _ImageSurveyLayer_js__WEBPACK_IMPORTED_MODULE_25__ = __webpack_require__(/*! ./ImageSurveyLayer.js */ "./src/js/ImageSurveyLayer.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//


/******************************************************************************
 * Aladin Lite project
 * 
 * File Aladin.js (main class)
 * Facade to expose Aladin Lite methods
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/




























let Aladin = (function () {

    // Constructor
    var Aladin = function (aladinDiv, requestedOptions) {
        // check that aladinDiv exists, stop immediately otherwise
        if ($(aladinDiv).length == 0) {
            return;
        }
        this.webglAPI = null;

        var self = this;

        // if not options was set, try to retrieve them from the query string
        if (requestedOptions === undefined) {
            requestedOptions = this.getOptionsFromQueryString();
        }
        requestedOptions = requestedOptions || {};


        // 'fov' option was previsouly called 'zoom'
        if ('zoom' in requestedOptions) {
            var fovValue = requestedOptions.zoom;
            delete requestedOptions.zoom;
            requestedOptions.fov = fovValue;
        }
        // merge with default options
        var options = {};
        for (var key in Aladin.DEFAULT_OPTIONS) {
            if (requestedOptions[key] !== undefined) {
                options[key] = requestedOptions[key];
            }
            else {
                options[key] = Aladin.DEFAULT_OPTIONS[key];
            }
        }
        for (var key in requestedOptions) {
            if (Aladin.DEFAULT_OPTIONS[key] === undefined) {
                options[key] = requestedOptions[key];
            }
        }

        this.options = options;

        $("<style type='text/css'> .aladin-reticleColor { color: " + this.options.reticleColor + "; font-weight:bold;} </style>").appendTo(aladinDiv);



        this.aladinDiv = aladinDiv;

        this.reduceDeformations = true;

        // parent div
        $(aladinDiv).addClass("aladin-container");


        var cooFrame = _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].fromString(options.cooFrame, _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000);
        // locationDiv is the div where we write the position
        var locationDiv = $('<div class="aladin-location">'
            + (options.showFrame ? '<select class="aladin-frameChoice"><option value="' + _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000.label + '" '
                + (cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000 ? 'selected="selected"' : '') + '>J2000</option><option value="' + _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000d.label + '" '
                + (cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000d ? 'selected="selected"' : '') + '>J2000d</option><option value="' + _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].GAL.label + '" '
                + (cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].GAL ? 'selected="selected"' : '') + '>GAL</option></select>' : '')
            + '<span class="aladin-location-text"></span></div>')
            .appendTo(aladinDiv);
        // div where FoV value is written
        var fovDiv = $('<div class="aladin-fov"></div>').appendTo(aladinDiv);


        // zoom control
        if (options.showZoomControl) {
            $('<div class="aladin-zoomControl"><a href="#" class="zoomPlus" title="Zoom in">+</a><a href="#" class="zoomMinus" title="Zoom out">&ndash;</a></div>').appendTo(aladinDiv);
        }

        // maximize control
        if (options.showFullscreenControl) {
            $('<div class="aladin-fullscreenControl aladin-maximize" title="Full screen"></div>')
                .appendTo(aladinDiv);
        }
        this.fullScreenBtn = $(aladinDiv).find('.aladin-fullscreenControl')
        this.fullScreenBtn.click(function () {
            self.toggleFullscreen(self.options.realFullscreen);
        });
        // react to fullscreenchange event to restore initial width/height (if user pressed ESC to go back from full screen)
        $(document).on('fullscreenchange webkitfullscreenchange mozfullscreenchange MSFullscreenChange', function (e) {
            var fullscreenElt = document.fullscreenElement || document.webkitFullscreenElement || document.mozFullScreenElement || document.msFullscreenElement;
            if (fullscreenElt === null || fullscreenElt === undefined) {
                self.fullScreenBtn.removeClass('aladin-restore');
                self.fullScreenBtn.addClass('aladin-maximize');
                self.fullScreenBtn.attr('title', 'Full screen');
                $(self.aladinDiv).removeClass('aladin-fullscreen');

                var fullScreenToggledFn = self.callbacksByEventName['fullScreenToggled'];
                var isInFullscreen = self.fullScreenBtn.hasClass('aladin-restore');
                (typeof fullScreenToggledFn === 'function') && fullScreenToggledFn(isInFullscreen);
            }
        });





        // Aladin logo
        $("<div class='aladin-logo-container'><a href='https://aladin.unistra.fr/' title='Powered by Aladin Lite' target='_blank'><div class='aladin-logo'></div></a></div>").appendTo(aladinDiv);


        // we store the boxes
        this.boxes = [];

        // measurement table
        this.measurementTable = new _MeasurementTable_js__WEBPACK_IMPORTED_MODULE_14__["MeasurementTable"](aladinDiv);



        var location = new _Location_js__WEBPACK_IMPORTED_MODULE_15__["Location"](locationDiv.find('.aladin-location-text'));

        // set different options
        this.view = new _View_js__WEBPACK_IMPORTED_MODULE_0__["View"](this, location, fovDiv, cooFrame, options.fov);
        this.view.setShowGrid(options.showCooGrid);

        // retrieve available surveys
        // TODO: replace call with MocServer
        /*$.ajax({
            url: "//aladin.unistra.fr/java/nph-aladin.pl",
            data: { "frame": "aladinLiteDic" },
            method: 'GET',
            dataType: 'jsonp', // could this be repaced by json ??
            success: function (data) {
                var map = {};
                for (var k = 0; k < data.length; k++) {
                    map[data[k].id] = true;
                }
                // retrieve existing surveys
                for (var k = 0; k < HpxImageSurvey.SURVEYS.length; k++) {
                    if (!map[HpxImageSurvey.SURVEYS[k].id]) {
                        data.push(HpxImageSurvey.SURVEYS[k]);
                    }
                }
                HpxImageSurvey.SURVEYS = data;
                self.view.setUnknownSurveyIfNeeded();
            },
            error: function () {
            }
        });*/

        // layers control panel
        // TODO : valeur des checkbox en fonction des options
        // TODO : classe LayerBox
        if (options.showLayersControl) {
            var d = $('<div class="aladin-layersControl-container" title="Manage layers"><div class="aladin-layersControl"></div></div>');
            d.appendTo(aladinDiv);

            var layerBox = $('<div class="aladin-box aladin-layerBox aladin-cb-list"></div>');
            layerBox.appendTo(aladinDiv);

            this.boxes.push(layerBox);

            // we return false so that the default event is not submitted, and to prevent event bubbling
            d.click(function () { self.hideBoxes(); self.showLayerBox(); return false; });

        }


        // goto control panel
        if (options.showGotoControl) {
            var d = $('<div class="aladin-gotoControl-container" title="Go to position"><div class="aladin-gotoControl"></div></div>');
            d.appendTo(aladinDiv);

            var gotoBox =
                $('<div class="aladin-box aladin-gotoBox">' +
                    '<a class="aladin-closeBtn">&times;</a>' +
                    '<div style="clear: both;"></div>' +
                    '<form class="aladin-target-form">Go to: <input type="text" placeholder="Object name/position" /></form></div>');
            gotoBox.appendTo(aladinDiv);
            this.boxes.push(gotoBox);

            var input = gotoBox.find('.aladin-target-form input');
            input.on("paste keydown", function () {
                $(this).removeClass('aladin-unknownObject'); // remove red border
            });

            // TODO : classe GotoBox
            d.click(function () {
                self.hideBoxes();
                input.val('');
                input.removeClass('aladin-unknownObject');
                gotoBox.show();
                input.focus();


                return false;
            });
            gotoBox.find('.aladin-closeBtn').click(function () { self.hideBoxes(); return false; });
        }

        // simbad pointer tool
        if (options.showSimbadPointerControl) {
            var d = $('<div class="aladin-simbadPointerControl-container" title="SIMBAD pointer"><div class="aladin-simbadPointerControl"></div></div>');
            d.appendTo(aladinDiv);

            d.click(function () {
                self.view.setMode(_View_js__WEBPACK_IMPORTED_MODULE_0__["View"].TOOL_SIMBAD_POINTER);
            });
        }

        // share control panel
        if (options.showShareControl) {
            var d = $('<div class="aladin-shareControl-container" title="Get link for current view"><div class="aladin-shareControl"></div></div>');
            d.appendTo(aladinDiv);

            var shareBox =
                $('<div class="aladin-box aladin-shareBox">' +
                    '<a class="aladin-closeBtn">&times;</a>' +
                    '<div style="clear: both;"></div>' +
                    'Link to previewer: <span class="info"></span>' +
                    '<input type="text" class="aladin-shareInput" />' +
                    '</div>');
            shareBox.appendTo(aladinDiv);
            this.boxes.push(shareBox);


            // TODO : classe GotoBox, GenericBox
            d.click(function () {
                self.hideBoxes();
                shareBox.show();
                var url = self.getShareURL();
                shareBox.find('.aladin-shareInput').val(url).select();
                document.execCommand('copy');

                return false;
            });
            shareBox.find('.aladin-closeBtn').click(function () { self.hideBoxes(); return false; });
        }


        this.gotoObject(options.target, undefined, {forceAnimation: false});

        if (options.log) {
            var params = requestedOptions;
            params['version'] = Aladin.VERSION;
            _Logger_js__WEBPACK_IMPORTED_MODULE_9__["Logger"].log("startup", params);
        }

        this.showReticle(options.showReticle);

        if (options.catalogUrls) {
            for (var k = 0, len = options.catalogUrls.length; k < len; k++) {
                this.createCatalogFromVOTable(options.catalogUrls[k]);
            }
        }
        
        /*let webglAPI = await import('../render/pkg/');
        console.log('webgl imported');
        let shaders = await loadShaders(webglAPI);
        console.log(shaders);
    
        // Start our Rust application. You can find `WebClient` in `src/lib.rs`
        let resources = {
            'kernel': kernel,
        };
        Aladin.wasmLibs.webglAPI = new webglAPI.WebClient(shaders, resources);
        let webgl = Aladin.wasmLibs.webglAPI;
        webgl.resize(500, 400);*/

        /*let imageSurveyInfo = HpxImageSurvey.getSurveyInfoFromId(options.survey);
        console.log('image survey, ', imageSurveyInfo)
        webgl.setImageSurvey(imageSurveyInfo);*/
        // Add the image layers
        // For that we check the survey key of options
        // It can be given as a single string or an array of strings
        // for multiple blending surveys
        if (options.survey) {
            (async () => {
                if (typeof options.survey === Array) {
                    let i = 0;
                    options.survey.forEach(async (rootUrlOrId) => {
                        const survey = await Aladin.createImageSurvey(rootUrlOrId);
                        if (i == 0) {
                            this.setImageSurvey(survey, "base");
                        } else {
                            this.addImageSurvey(survey, "base");
                        }
                        i++;
                    });
                } else {
                    const survey = await Aladin.createImageSurvey(options.survey, "base");
                    this.setImageSurvey(survey, "base");
                }
            })();

        }
        this.view.showCatalog(options.showCatalog);


        var aladin = this;
        $(aladinDiv).find('.aladin-frameChoice').change(function () {
            aladin.setFrame($(this).val());
        });


        $(aladinDiv).find('.aladin-target-form').submit(function () {
            aladin.gotoObject($(this).find('input').val(), function () {
                $(aladinDiv).find('.aladin-target-form input').addClass('aladin-unknownObject');
            });
            return false;
        });

        var zoomPlus = $(aladinDiv).find('.zoomPlus');
        zoomPlus.click(function () {
            aladin.increaseZoom();
            return false;
        });
        zoomPlus.bind('mousedown', function (e) {
            e.preventDefault(); // to prevent text selection
        });

        var zoomMinus = $(aladinDiv).find('.zoomMinus');
        zoomMinus.click(function () {
            aladin.decreaseZoom();
            return false;
        });
        zoomMinus.bind('mousedown', function (e) {
            e.preventDefault(); // to prevent text selection
        });



        this.callbacksByEventName = {}; // we store the callback functions (on 'zoomChanged', 'positionChanged', ...) here

        // initialize the Vue components
        if (typeof Vue != "undefined") {
            //Vue.component("layers", Layers)
            this.discoverytree = new _DiscoveryTree_js__WEBPACK_IMPORTED_MODULE_24__["DiscoveryTree"](this);
        }

        this.view.redraw();

        // go to full screen ?
        if (options.fullScreen) {
            // strange behaviour to wait for a sec
            window.setTimeout(function () { self.toggleFullscreen(self.options.realFullscreen); }, 10);
        }
    };

    /**** CONSTANTS ****/
    Aladin.VERSION = "{ALADIN-LITE-VERSION-NUMBER}"; // will be filled by the build.sh script

    Aladin.JSONP_PROXY = "https://alasky.unistra.fr/cgi/JSONProxy";
    //Aladin.JSONP_PROXY = "https://alaskybis.unistra.fr/cgi/JSONProxy";

    // access to WASM libraries
    Aladin.wasmLibs = {};
    Aladin.webglAPI = [];
    Aladin.DEFAULT_OPTIONS = {
        target: "0 +0",
        cooFrame: "J2000",
        survey: "P/DSS2/color",
        fov: 60,
        showReticle: true,
        showZoomControl: true,
        showFullscreenControl: true,
        showLayersControl: true,
        showGotoControl: true,
        showSimbadPointerControl: false,
        showShareControl: false,
        showCatalog: true, // TODO: still used ??
        showFrame: true,
        showCooGrid: false,
        fullScreen: false,
        reticleColor: "rgb(178, 50, 178)",
        reticleSize: 22,
        log: true,
        allowFullZoomout: false,
        realFullscreen: false,
        showAllskyRing: false,
        allskyRingColor: '#c8c8ff',
        allskyRingWidth: 8,
        pixelateCanvas: true
    };


    // realFullscreen: AL div expands not only to the size of its parent, but takes the whole available screen estate 
    Aladin.prototype.toggleFullscreen = function (realFullscreen) {
        realFullscreen = Boolean(realFullscreen);

        this.fullScreenBtn.toggleClass('aladin-maximize aladin-restore');
        var isInFullscreen = this.fullScreenBtn.hasClass('aladin-restore');
        this.fullScreenBtn.attr('title', isInFullscreen ? 'Restore original size' : 'Full screen');
        $(this.aladinDiv).toggleClass('aladin-fullscreen');

        if (realFullscreen) {
            // go to "real" full screen mode
            if (isInFullscreen) {
                var d = this.aladinDiv;

                if (d.requestFullscreen) {
                    d.requestFullscreen();
                }
                else if (d.webkitRequestFullscreen) {
                    d.webkitRequestFullscreen();
                }
                else if (d.mozRequestFullScreen) { // notice the difference in capitalization for Mozilla functions ...
                    d.mozRequestFullScreen();
                }
                else if (d.msRequestFullscreen) {
                    d.msRequestFullscreen();
                }
            }
            // exit from "real" full screen mode
            else {
                if (document.exitFullscreen) {
                    document.exitFullscreen();
                }
                else if (document.webkitExitFullscreen) {
                    document.webkitExitFullscreen();
                }
                else if (document.mozCancelFullScreen) {
                    document.mozCancelFullScreen();
                }
                else if (document.webkitExitFullscreen) {
                    document.webkitExitFullscreen();
                }
            }
        }

        this.view.fixLayoutDimensions();

        // force call to zoomChanged callback
        var fovChangedFn = this.callbacksByEventName['zoomChanged'];
        (typeof fovChangedFn === 'function') && fovChangedFn(this.view.fov);

        var fullScreenToggledFn = this.callbacksByEventName['fullScreenToggled'];
        (typeof fullScreenToggledFn === 'function') && fullScreenToggledFn(isInFullscreen);
    };

    Aladin.prototype.updateSurveysDropdownList = function (surveys) {
        surveys = surveys.sort(function (a, b) {
            if (!a.order) {
                return a.id > b.id;
            }
            return a.order && a.order > b.order ? 1 : -1;
        });
        var select = $(this.aladinDiv).find('.aladin-surveySelection');
        select.empty();
        for (var i = 0; i < surveys.length; i++) {
            var isCurSurvey = this.view.imageSurvey.id == surveys[i].id;
            select.append($("<option />").attr("selected", isCurSurvey).val(surveys[i].id).text(surveys[i].name));
        };
    };

    Aladin.prototype.setAngleRotation = function (theta) {
        this.view.setAngleRotation(theta)
    }

    Aladin.prototype.getOptionsFromQueryString = function () {
        var options = {};
        var requestedTarget = $.urlParam('target');
        if (requestedTarget) {
            options.target = requestedTarget;
        }
        var requestedFrame = $.urlParam('frame');
        if (requestedFrame && _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"][requestedFrame]) {
            options.frame = requestedFrame;
        }
        var requestedSurveyId = $.urlParam('survey');
        if (requestedSurveyId && _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_17__["HpxImageSurvey"].getSurveyInfoFromId(requestedSurveyId)) {
            options.survey = requestedSurveyId;
        }
        var requestedZoom = $.urlParam('zoom');
        if (requestedZoom && requestedZoom > 0 && requestedZoom < 180) {
            options.zoom = requestedZoom;
        }

        var requestedShowreticle = $.urlParam('showReticle');
        if (requestedShowreticle) {
            options.showReticle = requestedShowreticle.toLowerCase() == 'true';
        }

        var requestedCooFrame = $.urlParam('cooFrame');
        if (requestedCooFrame) {
            options.cooFrame = requestedCooFrame;
        }

        var requestedFullscreen = $.urlParam('fullScreen');
        if (requestedFullscreen !== undefined) {
            options.fullScreen = requestedFullscreen;
        }

        return options;
    };

    // @API
    Aladin.prototype.setFoV = Aladin.prototype.setFov = function (fovDegrees) {
        this.view.setZoom(fovDegrees);
    };

    // @API
    // (experimental) try to adjust the FoV to the given object name. Does nothing if object is not known from Simbad
    Aladin.prototype.adjustFovForObject = function (objectName) {
        var self = this;
        this.getFovForObject(objectName, function (fovDegrees) {
            self.setFoV(fovDegrees);
        });
    };


    Aladin.prototype.getFovForObject = function (objectName, callback) {
        var query = "SELECT galdim_majaxis, V FROM basic JOIN ident ON oid=ident.oidref JOIN allfluxes ON oid=allfluxes.oidref WHERE id='" + objectName + "'";
        var url = '//simbad.u-strasbg.fr/simbad/sim-tap/sync?query=' + encodeURIComponent(query) + '&request=doQuery&lang=adql&format=json&phase=run';

        var ajax = _Utils_js__WEBPACK_IMPORTED_MODULE_2__["Utils"].getAjaxObject(url, 'GET', 'json', false)
        ajax.done(function (result) {
            var defaultFov = 4 / 60; // 4 arcmin
            var fov = defaultFov;

            if ('data' in result && result.data.length > 0) {
                var galdimMajAxis = _Utils_js__WEBPACK_IMPORTED_MODULE_2__["Utils"].isNumber(result.data[0][0]) ? result.data[0][0] / 60.0 : null; // result gives galdim in arcmin
                var magV = _Utils_js__WEBPACK_IMPORTED_MODULE_2__["Utils"].isNumber(result.data[0][1]) ? result.data[0][1] : null;

                if (galdimMajAxis !== null) {
                    fov = 2 * galdimMajAxis;
                }
                else if (magV !== null) {
                    if (magV < 10) {
                        fov = 2 * Math.pow(2.0, (6 - magV / 2.0)) / 60;
                    }
                }
            }

            (typeof callback === 'function') && callback(fov);
        });
    };

    Aladin.prototype.setFrame = function (frameName) {
        if (!frameName) {
            return;
        }
        var newFrame = _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].fromString(frameName, _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].J2000);
        if (newFrame == this.view.cooFrame) {
            return;
        }

        this.view.changeFrame(newFrame);
        // màj select box
        $(this.aladinDiv).find('.aladin-frameChoice').val(newFrame.label);
    };

    Aladin.prototype.setProjection = function (projectionName) {
        if (!projectionName) {
            return;
        }
        projectionName = projectionName.toLowerCase();
        /*console.log('setProj', projectionName);

        let projectionOptionElt = document.getElementById(projectionName);
        console.log("jKJHKSDJHF")

        console.log(projectionOptionElt);
        if (projectionOptionElt) {
            console.log("jKJHKSDJHF")
            projectionOptionElt.selected = 'selected';
        }*/
        
        this.view.changeProjection(projectionName);
        //this.view.fov_limit = this.webglAPI.getMaxFieldOfView() * 180 / Math.PI;
    };

    /** point view to a given object (resolved by Sesame) or position
     * @api
     *
     * @param: target; object name or position
     * @callbackOptions: (optional) the object with key 'success' and/or 'error' containing the success and error callback functions.
     *
     */
    Aladin.prototype.gotoObject = function (targetName, callbackOptions, options) {
        let successCallback = undefined;
        let errorCallback   = undefined;
        if (typeof callbackOptions === 'object') {
            if (callbackOptions.hasOwnProperty('success')) {
                successCallback = callbackOptions.success;
            }
            if (callbackOptions.hasOwnProperty('error')) {
                errorCallback = callbackOptions.error;
            }
        }
        // this is for compatibility reason with the previous method signature which was function(targetName, errorCallback)
        else if (typeof callbackOptions === 'function') {
            errorCallback = callbackOptions;
        }


        var isObjectName = /[a-zA-Z]/.test(targetName);

        // try to parse as a position
        if (!isObjectName) {
            var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_18__["Coo"]();

            coo.parse(targetName);
            var lonlat = [coo.lon, coo.lat];
            // Convert it to icrs if the coo system is galactic
            if (this.view.aladin.webglAPI.cooSystem() === Aladin.wasmLibs.webgl.GALCooSys()) {
                lonlat = this.view.aladin.webglAPI.Gal2J2000(coo.lon, coo.lat);
            }
            this.view.pointTo(lonlat[0], lonlat[1], options);

            (typeof successCallback === 'function') && successCallback(this.getRaDec());
        }
        // ask resolution by Sesame
        else {
            var self = this;
            _Sesame_js__WEBPACK_IMPORTED_MODULE_12__["Sesame"].resolve(targetName,
                function (data) { // success callback
                    // Location given in icrs at J2000
                    var ra = data.Target.Resolver.jradeg;
                    var dec = data.Target.Resolver.jdedeg;

                    self.view.pointTo(ra, dec, options);

                    (typeof successCallback === 'function') && successCallback(self.getRaDec());
                },
                function (data) { // errror callback
                    if (console) {
                        console.log("Could not resolve object name " + targetName);
                        console.log(data);
                    }
                    (typeof errorCallback === 'function') && errorCallback();
                });
        }
    };



    /**
     * go to a given position, expressed in the current coordinate frame
     * 
     * @API
     */
    Aladin.prototype.gotoPosition = function (lon, lat) {
        var radec;
        // first, convert to J2000 if needed
        if (this.view.cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].GAL) {
            radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].GalacticToJ2000([lon, lat]);
        }
        else {
            radec = [lon, lat];
        }
        this.view.pointTo(radec[0], radec[1]);
    };


    var doAnimation = function (aladin) {
        var params = aladin.animationParams;
        if (params == null || !params['running']) {
            return;
        }
        var now = new Date().getTime();
        // this is the animation end: set the view to the end position, and call complete callback 
        if (now > params['end']) {
            aladin.gotoRaDec(params['raEnd'], params['decEnd']);

            if (params['complete']) {
                params['complete']();
            }

            return;
        }

        // compute current position
        var fraction = (now - params['start']) / (params['end'] - params['start']);
        var curPos = intermediatePoint(params['raStart'], params['decStart'], params['raEnd'], params['decEnd'], fraction);
        var curRa = curPos[0];
        var curDec = curPos[1];
        //var curRa =  params['raStart'] + (params['raEnd'] - params['raStart']) * (now-params['start']) / (params['end'] - params['start']);
        //var curDec = params['decStart'] + (params['decEnd'] - params['decStart']) * (now-params['start']) / (params['end'] - params['start']);

        aladin.gotoRaDec(curRa, curDec);

        setTimeout(function () { doAnimation(aladin); }, 50);

    };

    /*
     * Stop all animations that have been initiated  by animateToRaDec or by zoomToFoV
     * @API
     *
     */
    Aladin.prototype.stopAnimation = function () {
        if (this.zoomAnimationParams) {
            this.zoomAnimationParams['running'] = false;
        }
        if (this.animationParams) {
            this.animationParams['running'] = false;
        }
    }

    /*
     * animate smoothly from the current position to the given ra, dec
     * 
     * the total duration (in seconds) of the animation can be given (otherwise set to 5 seconds by default)
     * 
     * complete: a function to call once the animation has completed
     * 
     * @API
     * 
     */
    Aladin.prototype.animateToRaDec = function (ra, dec, duration, complete) {
        duration = duration || 5;

        this.animationParams = null;

        var animationParams = {};
        animationParams['start'] = new Date().getTime();
        animationParams['end'] = new Date().getTime() + 1000 * duration;
        var raDec = this.getRaDec();
        animationParams['raStart'] = raDec[0];
        animationParams['decStart'] = raDec[1];
        animationParams['raEnd'] = ra;
        animationParams['decEnd'] = dec;
        animationParams['complete'] = complete;
        animationParams['running'] = true;

        this.animationParams = animationParams;

        doAnimation(this);
    };

    var doZoomAnimation = function (aladin) {
        var params = aladin.zoomAnimationParams;
        if (params == null || !params['running']) {
            return;
        }
        var now = new Date().getTime();
        // this is the zoom animation end: set the view to the end fov, and call complete callback 
        if (now > params['end']) {
            aladin.setFoV(params['fovEnd']);

            if (params['complete']) {
                params['complete']();
            }

            return;
        }

        // compute current position
        var fraction = (now - params['start']) / (params['end'] - params['start']);
        var curFov = params['fovStart'] + (params['fovEnd'] - params['fovStart']) * Math.sqrt(fraction);

        aladin.setFoV(curFov);

        setTimeout(function () { doZoomAnimation(aladin); }, 50);

    };
    /*
     * zoom smoothly from the current FoV to the given new fov to the given ra, dec
     * 
     * the total duration (in seconds) of the animation can be given (otherwise set to 5 seconds by default)
     * 
     * complete: a function to call once the animation has completed
     * 
     * @API
     * 
     */
    Aladin.prototype.zoomToFoV = function (fov, duration, complete) {
        duration = duration || 5;

        this.zoomAnimationParams = null;

        var zoomAnimationParams = {};
        zoomAnimationParams['start'] = new Date().getTime();
        zoomAnimationParams['end'] = new Date().getTime() + 1000 * duration;
        var fovArray = this.getFov();
        zoomAnimationParams['fovStart'] = Math.max(fovArray[0], fovArray[1]);
        zoomAnimationParams['fovEnd'] = fov;
        zoomAnimationParams['complete'] = complete;
        zoomAnimationParams['running'] = true;

        this.zoomAnimationParams = zoomAnimationParams;
        doZoomAnimation(this);
    };



    /**
     *  Compute intermediate point between points (lng1, lat1) and (lng2, lat2)
     *  at distance fraction times the total distance (fraction between 0 and 1)
     *
     *  Return intermediate points in degrees
     *
     */
    function intermediatePoint(lng1, lat1, lng2, lat2, fraction) {
        function degToRad(d) {
            return d * Math.PI / 180;
        }
        function radToDeg(r) {
            return r * 180 / Math.PI;
        }
        var lat1 = degToRad(lat1);
        var lng1 = degToRad(lng1);
        var lat2 = degToRad(lat2);
        var lng2 = degToRad(lng2);
        var d = 2 * Math.asin(
            Math.sqrt(Math.pow((Math.sin((lat1 - lat2) / 2)),
                2) +
                Math.cos(lat1) * Math.cos(lat2) *
                Math.pow(Math.sin((lng1 - lng2) / 2), 2)));
        var A = Math.sin((1 - fraction) * d) / Math.sin(d);
        var B = Math.sin(fraction * d) / Math.sin(d);
        var x = A * Math.cos(lat1) * Math.cos(lng1) + B *
            Math.cos(lat2) * Math.cos(lng2);
        var y = A * Math.cos(lat1) * Math.sin(lng1) + B *
            Math.cos(lat2) * Math.sin(lng2);
        var z = A * Math.sin(lat1) + B * Math.sin(lat2);
        var lon = Math.atan2(y, x);
        var lat = Math.atan2(z, Math.sqrt(Math.pow(x, 2) +
            Math.pow(y, 2)));

        return [radToDeg(lon), radToDeg(lat)];
    };




    /**
     * get current [ra, dec] position of the center of the view
     * 
     * @API
     */
    Aladin.prototype.getRaDec = function () {
        /*if (this.view.cooFrame.system == CooFrameEnum.SYSTEMS.J2000) {
            return [this.view.viewCenter.lon, this.view.viewCenter.lat];
        }
        else {
            var radec = CooConversion.GalacticToJ2000([this.view.viewCenter.lon, this.view.viewCenter.lat]);
            return radec;

        }*/
        let radec = this.webglAPI.getCenter();
        return radec;
    };


    /**
     * point to a given position, expressed as a ra,dec coordinate
     * 
     * @API
     */
    Aladin.prototype.gotoRaDec = function (ra, dec) {
        this.view.pointTo(ra, dec);
    };

    Aladin.prototype.showHealpixGrid = function (show) {
        this.view.showHealpixGrid(show);
    };

    Aladin.prototype.showSurvey = function (show) {
        this.view.showSurvey(show);
    };
    Aladin.prototype.showCatalog = function (show) {
        this.view.showCatalog(show);
    };
    Aladin.prototype.showReticle = function (show) {
        this.view.showReticle(show);
        $('#displayReticle').attr('checked', show);
    };
    Aladin.prototype.removeLayers = function () {
        this.view.removeLayers();
    };

    // these 3 methods should be merged into a unique "add" method
    Aladin.prototype.addCatalog = function (catalog) {
        this.view.addCatalog(catalog);
    };
    Aladin.prototype.addOverlay = function (overlay) {
        this.view.addOverlay(overlay);
    };
    Aladin.prototype.addMOC = function (moc) {
        this.view.addMOC(moc);
    };
    /*Aladin.prototype.addImageSurveyLayer = function (layer) {
        console.log("add layer", layer)
        this.view.addImageSurveyLayer(layer)
    };*/

    // @api
    /*Aladin.prototype.getBaseImageLayers = function () {
        return this.view.imageSurvey;
    };*/
    // @param imageSurvey : HpxImageSurvey object or image survey identifier
    // @api
    // @old

    Aladin.createImageSurvey = async function(rootUrlOrId) {
        const survey = await _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_17__["HpxImageSurvey"].create(rootUrlOrId);
        return survey;
    }

    Aladin.prototype.setImageSurvey = function (survey, layer) {
        let layerName;
        if (layer) {
            layerName = layer;
        } else {
            layerName = "base";
        }

        this.view.setImageSurvey(survey, layerName);
    };

    Aladin.prototype.setImageSurveysLayer = function (surveys, layer) {
        let layerName;
        if (layer) {
            layerName = layer;
        } else {
            layerName = "base";
        }

        this.view.setImageSurveysLayer(surveys, layerName);
    };

    Aladin.prototype.removeImageSurveysLayer = function (layer) {
        let layerName;
        if (layer) {
            layerName = layer;
        } else {
            layerName = "base";
        }

        this.view.removeImageSurveysLayer(layerName);
    };

    Aladin.prototype.moveImageSurveysLayerForward = function (layer) {
        this.view.moveImageSurveysLayerForward(surveys, layerName);
    };

    Aladin.prototype.addImageSurvey = function (survey, layer) {
        let layerName;
        if (layer) {
            layerName = layer;
        } else {
            layerName = "base";
        }
        this.view.addImageSurvey(survey, layerName);
    };

    Aladin.prototype.setOpacityLayer = function(opacity, layer) {
        let layerName;
        if (layer) {
            layerName = layer;
        } else {
            layerName = "base";
        }
        this.webglAPI.setOpacityLayer(opacity, layer)
    }

    // @api
    Aladin.prototype.setBaseImageSurveysLayer = function (surveys) {
        this.view.setImageSurveysLayer(surveys, 'base');
    };
    Aladin.prototype.setBaseImageSurvey = function (survey) {
        this.view.setImageSurvey(survey, 'base');
    };
    /*
    // @api
    Aladin.prototype.getOverlayImageLayer = function () {
        return this.view.overlayImageSurvey;
    };
    // @api
    Aladin.prototype.setOverlayImageLayer = function (imageSurvey, callback) {
        this.view.setOverlayImageSurvey(imageSurvey, callback);
    };
    */

    Aladin.prototype.increaseZoom = function (step) {
        //if (!step) {
        //    step = 5;
        //}
        //this.view.setZoomLevel(this.view.zoomLevel + step);
        this.view.increaseZoom();
    };

    Aladin.prototype.decreaseZoom = function (step) {
        //if (!step) {
        //    step = 5;
        //}
        //this.view.setZoomLevel(this.view.zoomLevel - step);
        this.view.decreaseZoom();
    };


    Aladin.prototype.createProgressiveCatalog = function (url, frame, maxOrder, options) {
        return new _ProgressiveCat_js__WEBPACK_IMPORTED_MODULE_11__["ProgressiveCat"](url, frame, maxOrder, options);
    };

    Aladin.prototype.createOverlay = function (options) {
        return new _Overlay_js__WEBPACK_IMPORTED_MODULE_3__["Overlay"](options);
    };




    Aladin.AVAILABLE_CALLBACKS = ['select', 'objectClicked', 'objectHovered', 'footprintClicked', 'footprintHovered', 'positionChanged', 'zoomChanged', 'click', 'mouseMove', 'fullScreenToggled', 'catalogReady'];
    // API
    //
    // setting callbacks
    Aladin.prototype.on = function (what, myFunction) {
        if (Aladin.AVAILABLE_CALLBACKS.indexOf(what) < 0) {
            return;
        }

        this.callbacksByEventName[what] = myFunction;
    };

    Aladin.prototype.select = function () {
        this.fire('selectstart');
    };

    Aladin.prototype.fire = function (what, params) {
        if (what === 'selectstart') {
            this.view.setMode(_View_js__WEBPACK_IMPORTED_MODULE_0__["View"].SELECT);
        }
        else if (what === 'selectend') {
            this.view.setMode(_View_js__WEBPACK_IMPORTED_MODULE_0__["View"].PAN);
            var callbackFn = this.callbacksByEventName['select'];
            (typeof callbackFn === 'function') && callbackFn(params);
        }
    };

    Aladin.prototype.hideBoxes = function () {
        if (this.boxes) {
            for (var k = 0; k < this.boxes.length; k++) {
                this.boxes[k].hide();
            }
        }
    };

    // ?
    Aladin.prototype.updateCM = function () {

    };

    // TODO : LayerBox (or Stack?) must be extracted as a separate object
    Aladin.prototype.showLayerBox = function () {
        var self = this;

        // first, update
        var layerBox = $(this.aladinDiv).find('.aladin-layerBox');
        layerBox.empty();
        layerBox.append('<a class="aladin-closeBtn">&times;</a>' +
            '<div style="clear: both;"></div>' +
            '</div>');

        layerBox.append('<div class="aladin-label">Projection</div>' +
        '<select id="projectionChoice"><option id="sinus" value="sinus">SINUS</option><option id="aitoff" value="aitoff">AITOFF</option><option id="mollweide" value="mollweide">MOLLWEIDE</option><option id="mercator" value="mercator">MERCATOR</option><option id="arc" value="arc">ARC</option><option id="tan" value="tan">TAN</option></select><br/>');

        $('#projectionChoice').change(function () {
            //$(this).selected = $(this).val();
            aladin.setProjection($(this).val());
        });
        
        layerBox.append('<div class="aladin-box-separator"></div>' +
        '<div class="aladin-label">Overlay layers</div>');

        //var cmDiv = layerBox.find('.aladin-cmap');

        // fill color maps options
        /*var cmSelect = layerBox.find('.aladin-cmSelection');
        for (var k = 0; k < ColorMap.MAPS_NAMES.length; k++) {
            cmSelect.append($("<option />").text(ColorMap.MAPS_NAMES[k]));
        }
        console.log(self.getBaseImageLayer())
        console.log(self.getBaseImageLayer().getColorMap())
        cmSelect.val(self.getBaseImageLayer().getColorMap().mapName);*/


        // loop over all overlay layers
        var layers = this.view.allOverlayLayers;
        var str = '<ul>';
        for (var k = layers.length - 1; k >= 0; k--) {
            var layer = layers[k];
            var name = layer.name;
            var checked = '';
            if (layer.isShowing) {
                checked = 'checked="checked"';
            }

            var tooltipText = '';
            var iconSvg = '';
            if (layer.type == 'catalog' || layer.type == 'progressivecat') {
                var nbSources = layer.getSources().length;
                tooltipText = nbSources + ' source' + (nbSources > 1 ? 's' : '');

                iconSvg = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_8__["AladinUtils"].SVG_ICONS.CATALOG;
            }
            else if (layer.type == 'moc') {
                tooltipText = 'Coverage: ' + (100 * layer.skyFraction()).toFixed(3) + ' % of sky';

                iconSvg = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_8__["AladinUtils"].SVG_ICONS.MOC;
            }
            else if (layer.type == 'overlay') {
                iconSvg = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_8__["AladinUtils"].SVG_ICONS.OVERLAY;
            }

            var rgbColor = $('<div></div>').css('color', layer.color).css('color'); // trick to retrieve the color as 'rgb(,,)' - does not work for named colors :(
            var labelColor = _Color_js__WEBPACK_IMPORTED_MODULE_20__["Color"].getLabelColorForBackground(rgbColor);

            // retrieve SVG icon, and apply the layer color
            var svgBase64 = window.btoa(iconSvg.replace(/FILLCOLOR/g, layer.color));
            str += '<li><div class="aladin-stack-icon" style=\'background-image: url("data:image/svg+xml;base64,' + svgBase64 + '");\'></div>';
            str += '<input type="checkbox" ' + checked + ' id="aladin_lite_' + name + '"></input><label for="aladin_lite_' + name + '" class="aladin-layer-label" style="background: ' + layer.color + '; color:' + labelColor + ';" title="' + tooltipText + '">' + name + '</label></li>';
        }
        str += '</ul>';
        layerBox.append(str);

        layerBox.append('<div class="aladin-blank-separator"></div>');

        // gestion du réticule
        var checked = '';
        if (this.view.displayReticle) {
            checked = 'checked="checked"';
        }
        var reticleCb = $('<input type="checkbox" ' + checked + ' id="displayReticle" />');
        layerBox.append(reticleCb).append('<label for="displayReticle">Reticle</label><br/>');
        reticleCb.change(function () {
            self.showReticle($(this).is(':checked'));
        });

        // Gestion grille Healpix
        checked = '';
        if (this.view.displayHpxGrid) {
            checked = 'checked="checked"';
        }
        var hpxGridCb = $('<input type="checkbox" ' + checked + ' id="displayHpxGrid"/>');
        layerBox.append(hpxGridCb).append('<label for="displayHpxGrid">HEALPix grid</label><br/>');
        hpxGridCb.change(function () {
            self.showHealpixGrid($(this).is(':checked'));
        });

        // Equatorial grid plot
        checked = '';
        if (this.view.showGrid) {
            checked = 'checked="checked"';
        }
        var equatorialGridCb = $('<input type="checkbox" ' + checked + ' id="displayEquatorialGrid"/>');
        layerBox.append(equatorialGridCb).append('<label for="displayEquatorialGrid">Equatorial grid</label><br/>');
        equatorialGridCb.change(function () {
            let isChecked = $(this).is(':checked');
            self.view.setShowGrid(isChecked);
        });


        layerBox.append('<div class="aladin-box-separator"></div>' +
            '<div class="aladin-label">Tools</div>');
        var exportBtn = $('<button class="aladin-btn" type="button">Export view as PNG</button>');
        layerBox.append(exportBtn);
        exportBtn.click(function () {
            self.exportAsPNG();
        });

        layerBox.find('.aladin-closeBtn').click(function () { self.hideBoxes(); return false; });

        // update list of surveys
        /*this.updateSurveysDropdownList(HpxImageSurvey.getAvailableSurveys());
        var surveySelection = $(this.aladinDiv).find('.aladin-surveySelection');
        surveySelection.change(function () {
            var survey = HpxImageSurvey.getAvailableSurveys()[$(this)[0].selectedIndex];
            self.setImageSurvey(survey.id, function () {
                var baseImgLayer = self.getBaseImageLayer();

                if (baseImgLayer.useCors) {
                    // update color map list with current value color map
                    cmSelect.val(baseImgLayer.getColorMap().mapName);
                    cmDiv.show();

                    exportBtn.show();
                }
                else {
                    cmDiv.hide();

                    exportBtn.hide();
                }
            });



        });

        //// COLOR MAP management ////////////////////////////////////////////
        // update color map
        cmDiv.find('.aladin-cmSelection').change(function () {
            var cmName = $(this).find(':selected').val();
            self.getBaseImageLayer().getColorMap().update(cmName);
        });

        // reverse color map
        cmDiv.find('.aladin-reverseCm').click(function () {
            self.getBaseImageLayer().getColorMap().reverse();
        });
        if (this.getBaseImageLayer().useCors) {
            cmDiv.show();
            exportBtn.show();
        }
        else {
            cmDiv.hide();
            exportBtn.hide();
        }
        layerBox.find('.aladin-reverseCm').parent().attr('disabled', true);
        */
        //////////////////////////////////////////////////////////////////////


        // handler to hide/show overlays
        $(this.aladinDiv).find('.aladin-layerBox ul input').change(function () {
            var layerName = ($(this).attr('id').substr(12));
            var layer = self.layerByName(layerName);
            if ($(this).is(':checked')) {
                layer.show();
            }
            else {
                layer.hide();
            }
        });

        // finally show
        layerBox.show();

    };

    Aladin.prototype.layerByName = function (name) {
        var c = this.view.allOverlayLayers;
        for (var k = 0; k < c.length; k++) {
            if (name == c[k].name) {
                return c[k];
            }
        }
        return null;
    };

    // TODO : integrate somehow into API ?
    Aladin.prototype.exportAsPNG = function (imgFormat) {
        var w = window.open();
        w.document.write('<img src="' + this.getViewDataURL() + '">');
        w.document.title = 'Aladin Lite snapshot';
    };

    /**
     * Return the current view as a data URL (base64-formatted string)
     * Parameters:
     * - options (optional): object with attributs
     *     * format (optional): 'image/png' or 'image/jpeg'
     *     * width: width in pixels of the image to output
     *     * height: height in pixels of the image to output
     *
     * @API
    */
    Aladin.prototype.getViewDataURL = function (options) {
        var options = options || {};
        // support for old API signature
        if (typeof options !== 'object') {
            var imgFormat = options;
            options = { format: imgFormat };
        }

        return this.view.getCanvasDataURL(options.format, options.width, options.height);
    }

    /**
     * Return the current view WCS as a key-value dictionary
     * Can be useful in coordination with getViewDataURL
     *
     * @API
    */
    Aladin.prototype.getViewWCS = function (options) {
        var raDec = this.getRaDec();
        var fov = this.getFov();
        // TODO: support for other projection methods than SIN
        return {
            NAXIS: 2,
            NAXIS1: this.view.width,
            NAXIS2: this.view.height,
            RADECSYS: 'ICRS',
            CRPIX1: this.view.width / 2,
            CRPIX2: this.view.height / 2,
            CRVAL1: raDec[0],
            CRVAL2: raDec[1],
            CTYPE1: 'RA---SIN',
            CTYPE2: 'DEC--SIN',
            CD1_1: fov[0] / this.view.width,
            CD1_2: 0.0,
            CD2_1: 0.0,
            CD2_2: fov[1] / this.view.height
        }
    }

    /** restrict FOV range
     * @API
     * @param minFOV in degrees when zoom in at max
     * @param maxFOV in degreen when zoom out at max
    */
    Aladin.prototype.setFovRange = Aladin.prototype.setFOVRange = function (minFOV, maxFOV) {
        if (minFOV > maxFOV) {
            var tmp = minFOV;
            minFOV = maxFOV;
            maxFOV = tmp;
        }

        this.view.minFOV = minFOV;
        this.view.maxFOV = maxFOV;

    };

    /**
     * Transform pixel coordinates to world coordinates
     * 
     * Origin (0,0) of pixel coordinates is at top left corner of Aladin Lite view
     * 
     * @API
     * 
     * @param x
     * @param y
     * 
     * @return a [ra, dec] array with world coordinates in degrees. Returns undefined is something went wrong
     * 
     */
    Aladin.prototype.pix2world = function (x, y) {
        // this might happen at early stage of initialization
        if (!this.view) {
            return undefined;
        }

        //var xy = AladinUtils.viewToXy(x, y, this.view.width, this.view.height, this.view.largestDim, this.view.zoomFactor);

        var radec;
        try {
            //radec = this.view.projection.unproject(xy.x, xy.y);
            radec = this.view.aladin.webglAPI.screenToWorld(x, y);
        }
        catch (e) {
            return undefined;
        }

        var res;
        // Convert it to icrs j2000
        if (this.view.aladin.webglAPI.cooSystem() === Aladin.wasmLibs.webgl.GALCooSys()) {
            res = this.view.aladin.webglAPI.Gal2J2000(radec[0], radec[1]);
        }

        return res;
    };

    /**
     * Transform world coordinates to pixel coordinates in the view
     * 
     * @API
     * 
     * @param ra  
     * @param dec
     * 
     * @return a [x, y] array with pixel coordinates in the view. Returns null if the projection failed somehow
     *   
     */
    Aladin.prototype.world2pix = function (ra, dec) {
        // this might happen at early stage of initialization
        if (!this.view) {
            return;
        }

        var xy;
        if (this.view.cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_13__["CooFrameEnum"].GAL) {
            var lonlat = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].J2000ToGalactic([ra, dec]);
            xy = this.view.projection.project(lonlat[0], lonlat[1]);
        }
        else {
            xy = this.view.projection.project(ra, dec);
        }
        if (xy) {
            var xyview = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_8__["AladinUtils"].xyToView(xy.X, xy.Y, this.view.width, this.view.height, this.view.largestDim, this.view.zoomFactor);
            return [xyview.vx, xyview.vy];
        }
        else {
            return null;
        }
    };

    /**
     * 
     * @API
     * 
     * @param ra  
     * @param nbSteps the number of points to return along each side (the total number of points returned is 4*nbSteps)
     * 
     * @return set of points along the current FoV with the following format: [[ra1, dec1], [ra2, dec2], ..., [ra_n, dec_n]]
     *   
     */
    Aladin.prototype.getFovCorners = function (nbSteps) {
        // default value: 1
        if (!nbSteps || nbSteps < 1) {
            nbSteps = 1;
        }

        var points = [];
        var x1, y1, x2, y2;
        for (var k = 0; k < 4; k++) {
            x1 = (k == 0 || k == 3) ? 0 : this.view.width - 1;
            y1 = (k < 2) ? 0 : this.view.height - 1;
            x2 = (k < 2) ? this.view.width - 1 : 0;
            y2 = (k == 1 || k == 2) ? this.view.height - 1 : 0;

            for (var step = 0; step < nbSteps; step++) {
                let radec = this.webglAPI.screenToWorld(x1 + step / nbSteps * (x2 - x1), y1 + step / nbSteps * (y2 - y1));
                points.push(radec);
            }
        }

        return points;

    };

    /**
     * @API
     * 
     * @return the current FoV size in degrees as a 2-elements array
     */
    Aladin.prototype.getFov = function () {
        var fovX = this.view.fov;
        var s = this.getSize();
        var fovY = s[1] / s[0] * fovX;
        // TODO : take into account AITOFF projection where fov can be larger than 180
        fovX = Math.min(fovX, 180);
        fovY = Math.min(fovY, 180);

        return [fovX, fovY];
    };

    /**
     * @API
     * 
     * @return the size in pixels of the Aladin Lite view
     */
    Aladin.prototype.getSize = function () {
        return [this.view.width, this.view.height];
    };

    /**
     * @API
     * 
     * @return the jQuery object representing the DIV element where the Aladin Lite instance lies
     */
    Aladin.prototype.getParentDiv = function () {
        return $(this.aladinDiv);
    };

    return Aladin;
})();

///////////////////////////////
/////// Aladin Lite API ///////
///////////////////////////////
let A = {};
//// New API ////
// For developers using Aladin lite: all objects should be created through the API, 
// rather than creating directly the corresponding JS objects
// This facade allows for more flexibility as objects can be updated/renamed harmlessly

//@API
A.aladin = function (divSelector, options) {
    return new Aladin($(divSelector)[0], options);
};

/*//@API
// TODO : lecture de properties
A.imageLayer = function (rootURLOrHiPSDefinition, options) {
    return new HpxImageSurvey(rootURLOrHiPSDefinition, options);
};*/

// @API
A.source = function (ra, dec, data, options) {
    return new _Source_js__WEBPACK_IMPORTED_MODULE_16__["Source"](ra, dec, data, options);
};

// @API
A.marker = function (ra, dec, options, data) {
    options = options || {};
    options['marker'] = true;
    return A.source(ra, dec, data, options);
};

A.createImageSurvey = async function(rootUrlOrId) {
    const survey = await _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_17__["HpxImageSurvey"].create(rootUrlOrId);
    return survey;
}

// @API
A.polygon = function (raDecArray) {
    var l = raDecArray.length;
    if (l > 0) {
        // close the polygon if needed
        if (raDecArray[0][0] != raDecArray[l - 1][0] || raDecArray[0][1] != raDecArray[l - 1][1]) {
            raDecArray.push([raDecArray[0][0], raDecArray[0][1]]);
        }
    }
    return new _Footprint_js__WEBPACK_IMPORTED_MODULE_4__["Footprint"](raDecArray);
};

//@API
A.polyline = function (raDecArray, options) {
    return new _Polyline_js__WEBPACK_IMPORTED_MODULE_7__["Polyline"](raDecArray, options);
};


// @API
A.circle = function (ra, dec, radiusDeg, options) {
    return new _Circle_js__WEBPACK_IMPORTED_MODULE_5__["Circle"]([ra, dec], radiusDeg, options);
};

/**
 * 
 * @API
 * 
 * @param ra 
 * @param dec
 * @param radiusRaDeg the radius along the ra axis in degrees
 * @param radiusDecDeg the radius along the dec axis in degrees
 * @param rotationDeg the rotation angle in degrees
 *   
 */
A.ellipse = function (ra, dec, radiusRaDeg, radiusDecDeg, rotationDeg, options) {
    return new _Ellipse_js__WEBPACK_IMPORTED_MODULE_6__["Ellipse"]([ra, dec], radiusRaDeg, radiusDecDeg, rotationDeg, options);
};

// @API
A.graphicOverlay = function (options) {
    return new _Overlay_js__WEBPACK_IMPORTED_MODULE_3__["Overlay"](options);
};

// Create a new image survey layer
//
// One can attach multiple surveys to 1 layer.
// Those survey colors are blended together.
// Layers are overlaid to each other
A.imageSurveyLayer = function(name) {
    return new _ImageSurveyLayer_js__WEBPACK_IMPORTED_MODULE_25__["ImageSurveyLayer"](name);
}

// @API
A.catalog = function (options) {
    return new _Catalog_js__WEBPACK_IMPORTED_MODULE_10__["Catalog"](options);
};

// @API
A.catalogHiPS = function (rootURL, options) {
    return new _ProgressiveCat_js__WEBPACK_IMPORTED_MODULE_11__["ProgressiveCat"](rootURL, null, null, options);
};

// @API
/*
 * return a Box GUI element to insert content
 */
Aladin.prototype.box = function (options) {
    var box = new Box(options);
    box.$parentDiv.appendTo(this.aladinDiv);

    return box;
};

// @API
/*
 * show popup at ra, dec position with given title and content
 */
Aladin.prototype.showPopup = function (ra, dec, title, content) {
    this.view.catalogForPopup.removeAll();
    var marker = A.marker(ra, dec, { popupTitle: title, popupDesc: content, useMarkerDefaultIcon: false });
    this.view.catalogForPopup.addSources(marker);
    this.view.catalogForPopup.show();

    this.view.popup.setTitle(title);
    this.view.popup.setText(content);
    this.view.popup.setSource(marker);
    this.view.popup.show();
};

// @API
/*
 * hide popup
 */
Aladin.prototype.hidePopup = function () {
    this.view.popup.hide();
};

// @API
/*
 * return a URL allowing to share the current view
 */
Aladin.prototype.getShareURL = function () {
    var radec = this.getRaDec();
    var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_18__["Coo"]();
    coo.prec = 7;
    coo.lon = radec[0];
    coo.lat = radec[1];

    return 'https://aladin.unistra.fr/AladinLite/?target=' + encodeURIComponent(coo.format('s')) +
        '&fov=' + this.getFov()[0].toFixed(2) + '&survey=' + encodeURIComponent(this.getBaseImageLayer().id || this.getBaseImageLayer().rootUrl);
};

// @API
/*
 * return, as a string, the HTML embed code
 */
Aladin.prototype.getEmbedCode = function () {
    var radec = this.getRaDec();
    var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_18__["Coo"]();
    coo.prec = 7;
    coo.lon = radec[0];
    coo.lat = radec[1];

    var survey = this.getBaseImageLayer().id;
    var fov = this.getFov()[0];
    var s = '';
    s += '<link rel="stylesheet" href="https://aladin.unistra.fr/AladinLite/api/v2/latest/aladin.min.css" />\n';
    s += '<script type="text/javascript" src="https://code.jquery.com/jquery-1.9.1.min.js" charset="utf-8"></script>\n';
    s += '<div id="aladin-lite-div" style="width:400px;height:400px;"></div>\n';
    s += '<script type="text/javascript" src="https://aladin.unistra.fr/AladinLite/api/v2/latest/aladin.min.js" charset="utf-8"></script>\n';
    s += '<script type="text/javascript">\n';
    s += 'var aladin = A.aladin("#aladin-lite-div", {survey: "' + survey + 'P/DSS2/color", fov: ' + fov.toFixed(2) + ', target: "' + coo.format('s') + '"});\n';
    s += '</script>';
    return s;
};

// @API
/*
 * Creates remotely a HiPS from a FITS image URL and displays it
 */
Aladin.prototype.displayFITS = function (url, layerName, options, successCallback, errorCallback) {
    options = options || {};
    var data = { url: url };
    if (options.color) {
        data.color = true;
    }
    if (options.outputFormat) {
        data.format = options.outputFormat;
    }
    if (options.order) {
        data.order = options.order;
    }
    if (options.nocache) {
        data.nocache = options.nocache;
    }
    let self = this;
    $.ajax({
        url: 'https://alasky.unistra.fr/cgi/fits2HiPS',
        data: data,
        method: 'GET',
        dataType: 'json',
        success: function (response) {
            if (response.status != 'success') {
                console.error('An error occured: ' + response.message);
                if (errorCallback) {
                    errorCallback(response.message);
                }
                return;
            }
            var label = options.label || "FITS image";
            var meta = response.data.meta;

            (async () => {
                let survey = await Aladin.createImageSurvey(response.data.url);
                var transparency = (options && options.transparency) || 1.0;
    
                var executeDefaultSuccessAction = true;
                if (successCallback) {
                    executeDefaultSuccessAction = successCallback(meta.ra, meta.dec, meta.fov);
                }
                if (executeDefaultSuccessAction === true) {
                    self.webglAPI.setCenter(meta.ra, meta.dec);
                    self.setFoV(meta.fov);
                }
                // TODO! set an image survey once the already loaded surveys
                // are READY! Otherwise it can lead to some congestion and avoid
                // downloading the base tiles of the other surveys loading!
                // This has to be fixed in the backend but a fast fix is just to wait
                // before setting a new image survey
                
                    self.setImageSurvey(survey, layerName)
                    // set transparency
                    self.setOpacityLayer(transparency, layerName)

            })();
        }
    });

};

// @API
/*
 * Creates remotely a HiPS from a JPEG or PNG image with astrometry info
 * and display it
 */
Aladin.prototype.displayJPG = Aladin.prototype.displayPNG = function (url, layerName, options, successCallback, errorCallback) {
    options = options || {};
    options.color = true;
    options.label = "JPG/PNG image";
    options.outputFormat = 'png';
    this.displayFITS(url, layerName, options, successCallback, errorCallback);
};

Aladin.prototype.setReduceDeformations = function (reduce) {
    this.reduceDeformations = reduce;
    this.view.requestRedraw();
}

// API
A.footprintsFromSTCS = function (stcs) {
    var footprints = _Overlay_js__WEBPACK_IMPORTED_MODULE_3__["Overlay"].parseSTCS(stcs);

    return footprints;
}

// API
A.MOCFromURL = function (url, options, successCallback) {
    var moc = new _MOC_js__WEBPACK_IMPORTED_MODULE_1__["MOC"](options);
    moc.dataFromFITSURL(url, successCallback);

    return moc;
};

// API
A.MOCFromJSON = function (jsonMOC, options) {
    var moc = new _MOC_js__WEBPACK_IMPORTED_MODULE_1__["MOC"](options);
    moc.dataFromJSON(jsonMOC);

    return moc;
};


// TODO: try first without proxy, and then with, if param useProxy not set
// API
A.catalogFromURL = function (url, options, successCallback, useProxy) {
    var catalog = A.catalog(options);
    // TODO: should be self-contained in Catalog class
    _Catalog_js__WEBPACK_IMPORTED_MODULE_10__["Catalog"].parseVOTable(url, function (sources) {
        catalog.addSources(sources);
        if (successCallback) {
            successCallback(sources);
        }
    },
        catalog.maxNbSources, useProxy,
        catalog.raField, catalog.decField
    );

    return catalog;
};

// API
// @param target: can be either a string representing a position or an object name, or can be an object with keys 'ra' and 'dec' (values being in decimal degrees)
A.catalogFromSimbad = function (target, radius, options, successCallback) {
    options = options || {};
    if (!('name' in options)) {
        options['name'] = 'Simbad';
    }
    var url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildSimbadCSURL(target, radius);
    return A.catalogFromURL(url, options, successCallback, false);
};

// API
A.catalogFromNED = function (target, radius, options, successCallback) {
    options = options || {};
    if (!('name' in options)) {
        options['name'] = 'NED';
    }
    var url;
    if (target && (typeof target === "object")) {
        if ('ra' in target && 'dec' in target) {
            url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildNEDPositionCSURL(target.ra, target.dec, radius);
        }
    }
    else {
        var isObjectName = /[a-zA-Z]/.test(target);
        if (isObjectName) {
            url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildNEDObjectCSURL(target, radius);
        }
        else {
            var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_18__["Coo"]();
            coo.parse(target);
            url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildNEDPositionCSURL(coo.lon, coo.lat, radius);
        }
    }

    return A.catalogFromURL(url, options, successCallback);
};

// API
A.catalogFromVizieR = function (vizCatId, target, radius, options, successCallback) {
    options = options || {};
    if (!('name' in options)) {
        options['name'] = 'VizieR:' + vizCatId;
    }
    var url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildVizieRCSURL(vizCatId, target, radius, options);
    console.log(url);
    return A.catalogFromURL(url, options, successCallback, false);
};

// API
A.catalogFromSkyBot = function (ra, dec, radius, epoch, queryOptions, options, successCallback) {
    queryOptions = queryOptions || {};
    options = options || {};
    if (!('name' in options)) {
        options['name'] = 'SkyBot';
    }
    var url = _URLBuilder_js__WEBPACK_IMPORTED_MODULE_22__["URLBuilder"].buildSkyBotCSURL(ra, dec, radius, epoch, queryOptions);
    return A.catalogFromURL(url, options, successCallback, false);
};

A.hipsDefinitionFromURL = function(url, successCallback) {
    _HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_23__["HiPSDefinition"].fromURL(url, successCallback);
};


A.init = Promise.all([Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(2)]).then(__webpack_require__.bind(null, /*! @fxpineau/healpix */ "./node_modules/@fxpineau/healpix/healpix.js")), Promise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.bind(null, /*! ../core/pkg */ "./src/core/pkg/index.js"))]).then(async (values) => {
    let [hpxAPI, webglAPI] = values;

    // HEALPix library
    Aladin.wasmLibs.hpx = hpxAPI;
    // WebGL library
    Aladin.wasmLibs.webgl = webglAPI;
});

// this is ugly for sure and there must be a better way using Webpack magic
window.A = A;



/***/ }),

/***/ "./src/js/AladinUtils.js":
/*!*******************************!*\
  !*** ./src/js/AladinUtils.js ***!
  \*******************************/
/*! exports provided: AladinUtils */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AladinUtils", function() { return AladinUtils; });
/* harmony import */ var _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/astro/projection.js */ "./src/js/libs/astro/projection.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File AladinUtils
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/




let AladinUtils = (function() {

    return {
    	/**
    	 * passage de xy projection à xy dans la vue écran 
    	 * @param x
    	 * @param y
    	 * @param width
    	 * @param height
    	 * @param largestDim largest dimension of the view
    	 * @returns position in the view
    	 */
    	xyToView: function(x, y, width, height, largestDim, zoomFactor, round) {
    	    if (round==undefined) {
                // we round by default
    	        round = true;
    	    }

    	    if (round) {
    	        // we round the result for potential performance gains
    	        return {vx: AladinUtils.myRound(largestDim/2*(1+zoomFactor*x)-(largestDim-width)/2), vy: AladinUtils.myRound(largestDim/2*(1+zoomFactor*y)-(largestDim-height)/2)};

    	    }
    	    else {
                return {vx: largestDim/2*(1+zoomFactor*x)-(largestDim-width)/2, vy: largestDim/2*(1+zoomFactor*y)-(largestDim-height)/2};
    	    }
    	},
    	
    	/**
    	 * passage de xy dans la vue écran à xy projection
    	 * @param vx
    	 * @param vy
    	 * @param width
    	 * @param height
    	 * @param largestDim
    	 * @param zoomFactor
    	 * @returns position in xy projection
    	 */
    	viewToXy: function(vx, vy, width, height, largestDim, zoomFactor) {
    		return {x: ((2*vx+(largestDim-width))/largestDim-1)/zoomFactor, y: ((2*vy+(largestDim-height))/largestDim-1)/zoomFactor};
    	},

    	/**
    	 * convert a 
    	 * @returns position x,y in the view. Null if projection is impossible
    	 */
        /*radecToViewXy: function(ra, dec, currentProjection, currentFrame, width, height, largestDim, zoomFactor) {
            var xy;
            if (currentFrame.system != CooFrameEnum.SYSTEMS.J2000) {
                var lonlat = CooConversion.J2000ToGalactic([ra, dec]);
                xy = currentProjection.project(lonlat[0], lonlat[1]);
            }
            else {
                xy = currentProjection.project(ra, dec);
            }
            if (!xy) {
                return null;
            }

            return AladinUtils.xyToView(xy.X, xy.Y, width, height, largestDim, zoomFactor, false);
        },*/
        radecToViewXy: function(ra, dec, view) {
            //var xy;
            //if (currentFrame.system != CooFrameEnum.SYSTEMS.J2000) {
            //    var lonlat = CooConversion.J2000ToGalactic([ra, dec]);
            //    xy = view.aladin.webglAPI.worldToScreen(lonlat[0], lonlat[1]);
            //}
            //else {
            //var lonlat = CooConversion.J2000ToGalactic([ra, dec]);
            let xy = view.aladin.webglAPI.worldToScreen(ra, dec);
            //}
            //if (!xy) {
            //    return null;
            //}

            return xy;
        },
    	
    	myRound: function(a) {
    		if (a<0) {
    			return -1*( (-a) | 0);
    		}
    		else {
    			return a | 0;
    		}
    	},
    	
    	/**
    	 * Test whether a xy position is the view
    	 * @param vx
    	 * @param vy
    	 * @param width
    	 * @param height
    	 * @returns a boolean whether (vx, vy) is in the screen
    	 */
    	isInsideViewXy: function(vx, vy, width, height) {
    		return vx >= 0 && vx < width && vy >= 0 && vy < height
    	},
    	
    	/**
    	 * tests whether a healpix pixel is visible or not
    	 * @param pixCorners array of position (xy view) of the corners of the pixel
    	 * @param viewW
    	 */
    	isHpxPixVisible: function(pixCorners, viewWidth, viewHeight) {
    		for (var i = 0; i<pixCorners.length; i++) {
    			if ( pixCorners[i].vx>=-20 && pixCorners[i].vx<(viewWidth+20) &&
    				 pixCorners[i].vy>=-20 && pixCorners[i].vy<(viewHeight+20) ) {
    				return true;
    			}
    		}
    		return false;
    	},
    	
    	ipixToIpix: function(npixIn, norderIn, norderOut) {
    		var npixOut = [];
    		if (norderIn>=norderOut) {
    		}
    	},
        // Zoom is handled in the backend
        /*getZoomFactorForAngle: function(angleInDegrees, projectionMethod) {
            var p1 = {ra: 0, dec: 0};
            var p2 = {ra: angleInDegrees, dec: 0};
            var projection = new Projection(angleInDegrees/2, 0);
            projection.setProjection(projectionMethod);
            var p1Projected = projection.project(p1.ra, p1.dec);
            var p2Projected = projection.project(p2.ra, p2.dec);
           
            var zoomFactor = 1/Math.abs(p1Projected.X - p2Projected.Y);

            return zoomFactor;
        },*/

        counterClockwiseTriangle: function(x1, y1, x2, y2, x3, y3) {
            // From: https://math.stackexchange.com/questions/1324179/how-to-tell-if-3-connected-points-are-connected-clockwise-or-counter-clockwise
            // | x1, y1, 1 |
            // | x2, y2, 1 | > 0 => the triangle is given in anticlockwise order
            // | x3, y3, 1 |
    
            return x1*y2 + y1*x3 + x2*y3 - x3*y2 - y3*x1 - x2*y1 >= 0;
        },

        // grow array b of vx,vy view positions by *val* pixels
        grow2: function(b, val) {
            var j=0;
            for ( var i=0; i<4; i++ ) {
                if ( b[i]==null ) {
                    j++;
                }
            }

            if( j>1 ) {
                return b;
            }

            var b1 = [];
            for ( var i=0; i<4; i++ ) {
                b1.push( {vx: b[i].vx, vy: b[i].vy} );
            }
    
            for ( var i=0; i<2; i++ ) {
                var a = i==1 ? 1 : 0;
                var c = i==1 ? 3 : 2;

                if ( b1[a]==null ) {
                    var d,g;
                    if ( a==0 || a==3 ) {
                        d=1;
                        g=2;
                    }
                    else {
                        d=0;
                        g=3;
                    }
                    b1[a] = {vx: (b1[d].vx+b1[g].vx)/2, vy: (b1[d].vy+b1[g].vy)/2};
                }
                if ( b1[c]==null ) {
                    var d,g;
                    if ( c==0 || c==3 ) {
                        d=1;
                        g=2;
                    }
                    else {
                        d=0;
                        g=3;
                    }
                    b1[c] = {vx: (b1[d].vx+b1[g].vx)/2, vy: (b1[d].vy+b1[g].vy)/2};
                }
                if( b1[a]==null || b1[c]==null ) {
                    continue;
                }

                var angle = Math.atan2(b1[c].vy-b1[a].vy, b1[c].vx-b1[a].vx);
                var chouilla = val*Math.cos(angle);
                b1[a].vx -= chouilla;
                b1[c].vx += chouilla;
                chouilla = val*Math.sin(angle);
                b1[a].vy-=chouilla;
                b1[c].vy+=chouilla;
            }
            return b1;
        },

        // SVG icons templates are stored here rather than in a CSS, as to allow
        // to dynamically change the fill color
        // Pretty ugly, haven't found a prettier solution yet
        //
        // TODO: store this in the Stack class once it will exist
        //
        SVG_ICONS: {
            CATALOG: '<svg xmlns="http://www.w3.org/2000/svg"><polygon points="1,0,5,0,5,3,1,3"  fill="FILLCOLOR" /><polygon points="7,0,9,0,9,3,7,3"  fill="FILLCOLOR" /><polygon points="10,0,12,0,12,3,10,3"  fill="FILLCOLOR" /><polygon points="13,0,15,0,15,3,13,3"  fill="FILLCOLOR" /><polyline points="1,5,5,9"  stroke="FILLCOLOR" /><polyline points="1,9,5,5" stroke="FILLCOLOR" /><line x1="7" y1="7" x2="15" y2="7" stroke="FILLCOLOR" stroke-width="2" /><polyline points="1,11,5,15"  stroke="FILLCOLOR" /><polyline points="1,15,5,11"  stroke="FILLCOLOR" /><line x1="7" y1="13" x2="15" y2="13" stroke="FILLCOLOR" stroke-width="2" /></svg>',
            MOC: '<svg xmlns="http://www.w3.org/2000/svg"><polyline points="0.5,7,2.5,7,2.5,5,7,5,7,3,10,3,10,5,13,5,13,7,15,7,15,9,13,9,13,12,10,12,10,14,7,14,7,12,2.5,12,2.5,10,0.5,10,0.5,7" stroke-width="1" stroke="FILLCOLOR" fill="transparent" /><line x1="1" y1="10" x2="6" y2="5" stroke="FILLCOLOR" stroke-width="0.5" /><line x1="2" y1="12" x2="10" y2="4" stroke="FILLCOLOR" stroke-width="0.5" /><line x1="5" y1="12" x2="12" y2="5" stroke="FILLCOLOR" stroke-width="0.5" /><line x1="7" y1="13" x2="13" y2="7" stroke="FILLCOLOR" stroke-width="0.5" /><line x1="10" y1="13" x2="13" y2="10" stroke="FILLCOLOR" stroke-width="0.5" /></svg>',
            OVERLAY: '<svg xmlns="http://www.w3.org/2000/svg"><polygon points="10,5,10,1,14,1,14,14,2,14,2,9,6,9,6,5" fill="transparent" stroke="FILLCOLOR" stroke-width="2"/></svg>'
        }
 
    };

})();



/***/ }),

/***/ "./src/js/Catalog.js":
/*!***************************!*\
  !*** ./src/js/Catalog.js ***!
  \***************************/
/*! exports provided: Catalog */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Catalog", function() { return Catalog; });
/* harmony import */ var _Source_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Source.js */ "./src/js/Source.js");
/* harmony import */ var _Color_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Color.js */ "./src/js/Color.js");
/* harmony import */ var _libs_healpix_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./libs/healpix.js */ "./src/js/libs/healpix.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//




/******************************************************************************
 * Aladin Lite project
 * 
 * File Catalog
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/









// TODO : harmoniser parsing avec classe ProgressiveCat
let Catalog = (function() {

   function Catalog(options) {
        options = options || {};

        this.type = 'catalog';    	this.name = options.name || "catalog";
    	this.color = options.color || _Color_js__WEBPACK_IMPORTED_MODULE_1__["Color"].getNextColor();
    	this.sourceSize = options.sourceSize || 8;
    	this.markerSize = options.sourceSize || 12;
    	this.shape = options.shape || "square";
        this.maxNbSources = options.limit || undefined;
        this.onClick = options.onClick || undefined;

        this.raField = options.raField || undefined; // ID or name of the field holding RA
        this.decField = options.decField || undefined; // ID or name of the field holding dec

    	this.indexationNorder = 5; // à quel niveau indexe-t-on les sources
    	this.sources = [];
    	this.hpxIdx = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_2__["HealpixIndex"](this.indexationNorder);
    	this.hpxIdx.init();

        this.displayLabel = options.displayLabel || false;
        this.labelColor = options.labelColor || this.color;
        this.labelFont = options.labelFont || '10px sans-serif';
        if (this.displayLabel) {
            this.labelColumn = options.labelColumn;
            if (!this.labelColumn) {
                this.displayLabel = false;
            }
        }
    	
        if (this.shape instanceof Image || this.shape instanceof HTMLCanvasElement) {
            this.sourceSize = this.shape.width;
        }
        this._shapeIsFunction = false; // if true, the shape is a function drawing on the canvas
        if ($.isFunction(this.shape)) {
            this._shapeIsFunction = true;
        }
        
    	this.selectionColor = '#00ff00';
    	

        // create this.cacheCanvas    	
    	// cacheCanvas permet de ne créer le path de la source qu'une fois, et de le réutiliser (cf. http://simonsarris.com/blog/427-increasing-performance-by-caching-paths-on-canvas)
        this.updateShape(options);

        this.cacheMarkerCanvas = document.createElement('canvas');
        this.cacheMarkerCanvas.width = this.markerSize;
        this.cacheMarkerCanvas.height = this.markerSize;
        var cacheMarkerCtx = this.cacheMarkerCanvas.getContext('2d');
        cacheMarkerCtx.fillStyle = this.color;
        cacheMarkerCtx.beginPath();
        var half = (this.markerSize)/2.;
        cacheMarkerCtx.arc(half, half, half-2, 0, 2 * Math.PI, false);
        cacheMarkerCtx.fill();
        cacheMarkerCtx.lineWidth = 2;
        cacheMarkerCtx.strokeStyle = '#ccc';
        cacheMarkerCtx.stroke();
        

        this.isShowing = true;
    };
    
    Catalog.createShape = function(shapeName, color, sourceSize) {
        if (shapeName instanceof Image || shapeName instanceof HTMLCanvasElement) { // in this case, the shape is already created
            return shapeName;
        }
        var c = document.createElement('canvas');
        c.width = c.height = sourceSize;
        var ctx= c.getContext('2d');
        ctx.beginPath();
        ctx.strokeStyle = color;
        ctx.lineWidth = 2.0;
        if (shapeName=="plus") {
            ctx.moveTo(sourceSize/2., 0);
            ctx.lineTo(sourceSize/2., sourceSize);
            ctx.stroke();
            
            ctx.moveTo(0, sourceSize/2.);
            ctx.lineTo(sourceSize, sourceSize/2.);
            ctx.stroke();
        }
        else if (shapeName=="cross") {
            ctx.moveTo(0, 0);
            ctx.lineTo(sourceSize-1, sourceSize-1);
            ctx.stroke();
            
            ctx.moveTo(sourceSize-1, 0);
            ctx.lineTo(0, sourceSize-1);
            ctx.stroke();
        }
        else if (shapeName=="rhomb") {
            ctx.moveTo(sourceSize/2, 0);
            ctx.lineTo(0, sourceSize/2);
            ctx.lineTo(sourceSize/2, sourceSize);
            ctx.lineTo(sourceSize, sourceSize/2);
            ctx.lineTo(sourceSize/2, 0);
            ctx.stroke();
        }
        else if (shapeName=="triangle") {
            ctx.moveTo(sourceSize/2, 0);
            ctx.lineTo(0, sourceSize-1);
            ctx.lineTo(sourceSize-1, sourceSize-1);
            ctx.lineTo(sourceSize/2, 0);
            ctx.stroke();
        }
        else if (shapeName=="circle") {
            ctx.arc(sourceSize/2, sourceSize/2, sourceSize/2 - 1, 0, 2*Math.PI, true);
            ctx.stroke();
        }
        else { // default shape: square
            ctx.moveTo(1, 0);
            ctx.lineTo(1,  sourceSize-1);
            ctx.lineTo( sourceSize-1,  sourceSize-1);
            ctx.lineTo( sourceSize-1, 1);
            ctx.lineTo(1, 1);
            ctx.stroke();
        }
        
        return c;
        
    };
    

        // find RA, Dec fields among the given fields
        //
        // @param fields: list of objects with ucd, unit, ID, name attributes
        // @param raField:  index or name of right ascension column (might be undefined)
        // @param decField: index or name of declination column (might be undefined)
        //
        function findRADecFields(fields, raField, decField) {
            var raFieldIdx,  decFieldIdx;
            raFieldIdx = decFieldIdx = null;

            // first, look if RA/DEC fields have been already given
            if (raField) { // ID or name of RA field given at catalogue creation
                for (var l=0, len=fields.length; l<len; l++) {
                    var field = fields[l];
                    if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isInt(raField) && raField<fields.length) { // raField can be given as an index
                        raFieldIdx = raField;
                        break;
                    } 
                    if ( (field.ID && field.ID===raField) || (field.name && field.name===raField)) {
                        raFieldIdx = l;
                        break;
                    }
                }
            }
            if (decField) { // ID or name of dec field given at catalogue creation
                for (var l=0, len=fields.length; l<len; l++) {
                    var field = fields[l];
                    if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isInt(decField) && decField<fields.length) { // decField can be given as an index
                        decFieldIdx = decField;
                        break;
                    } 
                    if ( (field.ID && field.ID===decField) || (field.name && field.name===decField)) {
                        decFieldIdx = l;
                        break;
                    }
                }
            }
            // if not already given, let's guess position columns on the basis of UCDs
            for (var l=0, len=fields.length; l<len; l++) {
                if (raFieldIdx!=null && decFieldIdx!=null) {
                    break;
                }

                var field = fields[l];
                if ( ! raFieldIdx) {
                    if (field.ucd) {
                        var ucd = $.trim(field.ucd.toLowerCase());
                        if (ucd.indexOf('pos.eq.ra')==0 || ucd.indexOf('pos_eq_ra')==0) {
                            raFieldIdx = l;
                            continue;
                        }
                    }
                }
                    
                if ( ! decFieldIdx) {
                    if (field.ucd) {
                        var ucd = $.trim(field.ucd.toLowerCase());
                        if (ucd.indexOf('pos.eq.dec')==0 || ucd.indexOf('pos_eq_dec')==0) {
                            decFieldIdx = l;
                            continue;
                        }
                    }
                }
            }

            // still not found ? try some common names for RA and Dec columns
            if (raFieldIdx==null && decFieldIdx==null) {
                for (var l=0, len=fields.length; l<len; l++) {
                    var field = fields[l];
                    var name = field.name || field.ID || '';
                    name = name.toLowerCase();
                    
                    if ( ! raFieldIdx) {
                        if (name.indexOf('ra')==0 || name.indexOf('_ra')==0 || name.indexOf('ra(icrs)')==0 || name.indexOf('_ra')==0 || name.indexOf('alpha')==0) {
                            raFieldIdx = l;
                            continue;
                        }
                    }

                    if ( ! decFieldIdx) {
                        if (name.indexOf('dej2000')==0 || name.indexOf('_dej2000')==0 || name.indexOf('de')==0 || name.indexOf('de(icrs)')==0 || name.indexOf('_de')==0 || name.indexOf('delta')==0) {
                            decFieldIdx = l;
                            continue;
                        }
                    }
                    
                }
            }

            // last resort: take two first fieds
            if (raFieldIdx==null || decFieldIdx==null) {
                raFieldIdx  = 0;
                decFieldIdx = 1
            }

            return [raFieldIdx, decFieldIdx];
        };
        
    
    
    // return an array of Source(s) from a VOTable url
    // callback function is called each time a TABLE element has been parsed
    Catalog.parseVOTable = function(url, callback, maxNbSources, useProxy, raField, decField) {

        // adapted from votable.js
        function getPrefix($xml) {
            var prefix;
            // If Webkit chrome/safari/... (no need prefix)
            if($xml.find('RESOURCE').length>0) {
                prefix = '';
            }
            else {
                // Select all data in the document
                prefix = $xml.find("*").first();

                if (prefix.length==0) {
                    return '';
                }

                // get name of the first tag
                prefix = prefix.prop("tagName");

                var idx = prefix.indexOf(':');

                prefix = prefix.substring(0, idx) + "\\:";


            }

            return prefix;
        }

        function doParseVOTable(xml, callback) {
            xml = xml.replace(/^\s+/g, ''); // we need to trim whitespaces at start of document
            var attributes = ["name", "ID", "ucd", "utype", "unit", "datatype", "arraysize", "width", "precision"];
            
            var fields = [];
            var k = 0;
            var $xml = $($.parseXML(xml));
            var prefix = getPrefix($xml);
            $xml.find(prefix + "FIELD").each(function() {
                var f = {};
                for (var i=0; i<attributes.length; i++) {
                    var attribute = attributes[i];
                    if ($(this).attr(attribute)) {
                        f[attribute] = $(this).attr(attribute);
                    }
                }
                if ( ! f.ID) {
                    f.ID = "col_" + k;
                }
                fields.push(f);
                k++;
            });
                
            var raDecFieldIdxes = findRADecFields(fields, raField, decField);
            var raFieldIdx,  decFieldIdx;
            raFieldIdx = raDecFieldIdxes[0];
            decFieldIdx = raDecFieldIdxes[1];

            var sources = [];
            
            var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_6__["Coo"]();
            var ra, dec;
            $xml.find(prefix + "TR").each(function() {
               var mesures = {};
               var k = 0;
               $(this).find(prefix + "TD").each(function() {
                   var key = fields[k].name ? fields[k].name : fields[k].id;
                   mesures[key] = $(this).text();
                   k++;
               });
               var keyRa = fields[raFieldIdx].name ? fields[raFieldIdx].name : fields[raFieldIdx].id;
               var keyDec = fields[decFieldIdx].name ? fields[decFieldIdx].name : fields[decFieldIdx].id;

               if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(mesures[keyRa]) && _Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(mesures[keyDec])) {
                   ra = parseFloat(mesures[keyRa]);
                   dec = parseFloat(mesures[keyDec]);
               }
               else {
                   coo.parse(mesures[keyRa] + " " + mesures[keyDec]);
                   ra = coo.lon;
                   dec = coo.lat;
               }
               sources.push(new _Source_js__WEBPACK_IMPORTED_MODULE_0__["Source"](ra, dec, mesures));
               if (maxNbSources && sources.length==maxNbSources) {
                   return false; // break the .each loop
               }
                
            });
            if (callback) {
                callback(sources);
            }
        }
        
        var ajax = _Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].getAjaxObject(url, 'GET', 'text', useProxy);
        ajax.done(function(xml) {
            doParseVOTable(xml, callback);
        });
    };

    // API
    Catalog.prototype.updateShape = function(options) {
        options = options || {};
    	this.color = options.color || this.color || _Color_js__WEBPACK_IMPORTED_MODULE_1__["Color"].getNextColor();
    	this.sourceSize = options.sourceSize || this.sourceSize || 6;
    	this.shape = options.shape || this.shape || "square";

        this.selectSize = this.sourceSize + 2;

        this.cacheCanvas = Catalog.createShape(this.shape, this.color, this.sourceSize); 
        this.cacheSelectCanvas = Catalog.createShape('square', this.selectionColor, this.selectSize);

        this.reportChange();
    };
    
    // API
    Catalog.prototype.addSources = function(sourcesToAdd) {
        sourcesToAdd = [].concat(sourcesToAdd); // make sure we have an array and not an individual source
    	this.sources = this.sources.concat(sourcesToAdd);
    	for (var k=0, len=sourcesToAdd.length; k<len; k++) {
    	    sourcesToAdd[k].setCatalog(this);
    	}
        this.reportChange();
    };

    // API
    //
    // create sources from a 2d array and add them to the catalog
    //
    // @param columnNames: array with names of the columns
    // @array: 2D-array, each item being a 1d-array with the same number of items as columnNames
    Catalog.prototype.addSourcesAsArray = function(columnNames, array) {
        var fields = [];
        for (var colIdx=0 ; colIdx<columnNames.length; colIdx++) {
            fields.push({name: columnNames[colIdx]});
        }
        var raDecFieldIdxes = findRADecFields(fields, this.raField, this.decField);
        var raFieldIdx,  decFieldIdx;
        raFieldIdx = raDecFieldIdxes[0];
        decFieldIdx = raDecFieldIdxes[1];


        var newSources = [];
        var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_6__["Coo"]();
        var ra, dec, row, dataDict;
        for (var rowIdx=0 ; rowIdx<array.length ; rowIdx++) {
            row = array[rowIdx];
            if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(row[raFieldIdx]) && _Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(row[decFieldIdx])) {
                   ra = parseFloat(row[raFieldIdx]);
                   dec = parseFloat(row[decFieldIdx]);
            }
               else {
                   coo.parse(row[raFieldIdx] + " " + row[decFieldIdx]);
                   ra = coo.lon;
                   dec = coo.lat;
               }

            dataDict = {};
            for (var colIdx=0 ; colIdx<columnNames.length; colIdx++) {
                dataDict[columnNames[colIdx]] = row[colIdx];
            }

            newSources.push(A.source(ra, dec, dataDict));
        }

        this.addSources(newSources);
    };
    
    // return the current list of Source objects
    Catalog.prototype.getSources = function() {
        return this.sources;
    };
    
    // TODO : fonction générique traversant la liste des sources
    Catalog.prototype.selectAll = function() {
        if (! this.sources) {
            return;
        }
        
        for (var k=0; k<this.sources.length; k++) {
            this.sources[k].select();
        }
    };
    
    Catalog.prototype.deselectAll = function() {
        if (! this.sources) {
            return;
        }
        
        for (var k=0; k<this.sources.length; k++) {
            this.sources[k].deselect();
        }
    };
    
    // return a source by index
    Catalog.prototype.getSource = function(idx) {
        if (idx<this.sources.length) {
            return this.sources[idx];
        }
        else {
            return null;
        }
    };
    
    Catalog.prototype.setView = function(view) {
        this.view = view;
        this.reportChange();
    };

    // remove a source
    Catalog.prototype.remove = function(source) {
        var idx = this.sources.indexOf(source);
        if (idx<0) {
            return;
        }

        this.sources[idx].deselect();
        this.sources.splice(idx, 1);

        this.reportChange();
    };
    
    Catalog.prototype.removeAll = Catalog.prototype.clear = function() {
        // TODO : RAZ de l'index
        this.sources = [];
    };
    
    Catalog.prototype.draw = function(ctx, projection, frame, width, height, largestDim, zoomFactor) {
        if (! this.isShowing) {
            return;
        }
        // tracé simple
        //ctx.strokeStyle= this.color;

        //ctx.lineWidth = 1;
    	//ctx.beginPath();
        if (this._shapeIsFunction) {
            ctx.save();
        }
        var sourcesInView = [];
 	    for (var k=0, len = this.sources.length; k<len; k++) {
		    var inView = Catalog.drawSource(this, this.sources[k], ctx, projection, frame, width, height, largestDim, zoomFactor);
            if (inView) {
                sourcesInView.push(this.sources[k]);
            }
        }
        if (this._shapeIsFunction) {
            ctx.restore();
        }
        //ctx.stroke();

    	// tracé sélection
        ctx.strokeStyle= this.selectionColor;
        //ctx.beginPath();
        var source;
        for (var k=0, len = sourcesInView.length; k<len; k++) {
            source = sourcesInView[k];
            if (! source.isSelected) {
                continue;
            }
            Catalog.drawSourceSelection(this, source, ctx);
            
        }
        // NEEDED ?
    	//ctx.stroke();

        // tracé label
        if (this.displayLabel) {
            ctx.fillStyle = this.labelColor;
            ctx.font = this.labelFont;
            for (var k=0, len = sourcesInView.length; k<len; k++) {
                Catalog.drawSourceLabel(this, sourcesInView[k], ctx);
            }
        }
    };
    
    
    
    Catalog.drawSource = function(catalogInstance, s, ctx, projection, frame, width, height, largestDim, zoomFactor) {
        if (! s.isShowing) {
            return false;
        }
        var sourceSize = catalogInstance.sourceSize;
        //console.log('COMPUTE', aladin.webglAPI.worldToScreen(s.ra, s.dec));
        var xy = catalogInstance.view.aladin.webglAPI.worldToScreen(s.ra, s.dec);

        /*
        // TODO : we could factorize this code with Aladin.world2pix
        var xy;
        if (frame.system != CooFrameEnum.SYSTEMS.J2000) {
            var lonlat = CooConversion.J2000ToGalactic([s.ra, s.dec]);
            xy = projection.project(lonlat[0], lonlat[1]);
        }
        else {
            xy = projection.project(s.ra, s.dec);
        }
        */

        if (xy) {
            //var xyview = AladinUtils.xyToView(xy.X, xy.Y, width, height, largestDim, zoomFactor, true);
            var xyview = {vx: xy[0], vy: xy[1]};
            var max = s.popup ? 100 : s.sourceSize;
            if (xyview) {
                // TODO : index sources by HEALPix cells at level 3, 4 ?

                // check if source is visible in view
                if (xyview.vx>(width+max)  || xyview.vx<(0-max) ||
                    xyview.vy>(height+max) || xyview.vy<(0-max)) {
                    s.x = s.y = undefined;
                    return false;
                }
                
                s.x = xyview.vx;
                s.y = xyview.vy;
                if (catalogInstance._shapeIsFunction) {
                    catalogInstance.shape(s, ctx, catalogInstance.view.getViewParams());
                }
                else if (s.marker && s.useMarkerDefaultIcon) {
                    ctx.drawImage(catalogInstance.cacheMarkerCanvas, s.x-sourceSize/2, s.y-sourceSize/2);
                }
                else {
                    ctx.drawImage(catalogInstance.cacheCanvas, s.x-catalogInstance.cacheCanvas.width/2, s.y-catalogInstance.cacheCanvas.height/2);
                }


                // has associated popup ?
                if (s.popup) {
                    s.popup.setPosition(s.x, s.y);
                }
                
                
            }
            return true;
        }
        else {
            return false;
        }

        
    };
    
    Catalog.drawSourceSelection = function(catalogInstance, s, ctx) {
        if (!s || !s.isShowing || !s.x || !s.y) {
            return;
        }
        var sourceSize = catalogInstance.selectSize;
        
        ctx.drawImage(catalogInstance.cacheSelectCanvas, s.x-sourceSize/2, s.y-sourceSize/2);
    };

    Catalog.drawSourceLabel = function(catalogInstance, s, ctx) {
        if (!s || !s.isShowing || !s.x || !s.y) {
            return;
        }

        var label = s.data[catalogInstance.labelColumn];
        if (!label) {
            return;
        }

        ctx.fillText(label, s.x, s.y);
    };

    
    // callback function to be called when the status of one of the sources has changed
    Catalog.prototype.reportChange = function() {
        this.view && this.view.requestRedraw();
    };
    
    Catalog.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        this.reportChange();
    };
    
    Catalog.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.view && this.view.popup && this.view.popup.source && this.view.popup.source.catalog==this) {
            this.view.popup.hide();
        }

        this.reportChange();
    };

    return Catalog;
})();


/***/ }),

/***/ "./src/js/Circle.js":
/*!**************************!*\
  !*** ./src/js/Circle.js ***!
  \**************************/
/*! exports provided: Circle */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Circle", function() { return Circle; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _Aladin_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./Aladin.js */ "./src/js/Aladin.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Circle
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/






// TODO : Circle and Footprint should inherit from the same root object
let Circle = (function() {
    // constructor
    let Circle = function(centerRaDec, radiusDegrees, options) {
        options = options || {};

        this.color = options['color'] || undefined;

        // TODO : all graphic overlays should have an id
        this.id = 'circle-' + _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].uuidv4();

        this.setCenter(centerRaDec);
        this.setRadius(radiusDegrees);
    	this.overlay = null;
    	
    	this.isShowing = true;
    	this.isSelected = false;
    };

    Circle.prototype.setOverlay = function(overlay) {
        this.overlay = overlay;
    };
    
    Circle.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Circle.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Circle.prototype.dispatchClickEvent = function() {
        if (this.overlay) {
            // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
            //window.dispatchEvent(new CustomEvent("footprintClicked", {
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
        }
    };
    
    Circle.prototype.select = function() {
        if (this.isSelected) {
            return;
        }
        this.isSelected = true;
        if (this.overlay) {
/*
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
*/

            this.overlay.reportChange();
        }
    };

    Circle.prototype.deselect = function() {
        if (! this.isSelected) {
            return;
        }
        this.isSelected = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };


    
    Circle.prototype.setCenter = function(centerRaDec) {
        this.centerRaDec = centerRaDec;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    Circle.prototype.setRadius = function(radiusDegrees) {
        this.radiusDegrees = radiusDegrees;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    // TODO
    Circle.prototype.draw = function(ctx, view, projection, frame, width, height, largestDim, zoomFactor, noStroke) {
        if (! this.isShowing) {
            return;
        }
        noStroke = noStroke===true || false;

        /*var centerXy;
        if (frame.system != CooFrameEnum.SYSTEMS.J2000) {
            var lonlat = CooConversion.J2000ToGalactic([this.centerRaDec[0], this.centerRaDec[1]]);
            centerXy = projection.project(lonlat[0], lonlat[1]);
        }
        else {
            centerXy = projection.project(this.centerRaDec[0], this.centerRaDec[1]);
        }
        if (!centerXy) {
            return;
        }
        var centerXyview = AladinUtils.xyToView(centerXy.X, centerXy.Y, width, height, largestDim, zoomFactor, false);*/
        var centerXyview = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__["AladinUtils"].radecToViewXy(this.centerRaDec[0], this.centerRaDec[1], view);
        if (!centerXyview) {
            // the center goes out of the projection
            // we do not draw it
            return;
        }
        // compute value of radius in pixels in current projection
        var ra = this.centerRaDec[0];
        var dec = this.centerRaDec[1] + (ra>0 ? - this.radiusDegrees : this.radiusDegrees);
        /*
        var circlePtXy;
        if (frame.system != CooFrameEnum.SYSTEMS.J2000) {
            var lonlat = CooConversion.J2000ToGalactic([ra, dec]);
            circlePtXy = projection.project(lonlat[0], lonlat[1]);
        }
        else {
            circlePtXy = projection.project(ra, dec);
        }
        if (!circlePtXy) {
            return;
        }
        var circlePtXyView = AladinUtils.xyToView(circlePtXy.X, circlePtXy.Y, width, height, largestDim, zoomFactor, false);
        */
        let circlePtXyView = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__["AladinUtils"].radecToViewXy(ra, dec, view);
        if (!circlePtXyView) {
            // the circle border goes out of the projection
            // we do not draw it
            return;
        }
        var dx = circlePtXyView[0] - centerXyview[0];
        var dy = circlePtXyView[1] - centerXyview[1];
        var radiusInPix = Math.sqrt(dx*dx + dy*dy);

        // TODO : check each 4 point until show
        var baseColor = this.color;
        if (! baseColor && this.overlay) {
            baseColor = this.overlay.color;
        }
        if (! baseColor) {
            baseColor = '#ff0000';
        }
        
        if (this.isSelected) {
            ctx.strokeStyle= Overlay.increaseBrightness(baseColor, 50);
        }
        else {
            ctx.strokeStyle= baseColor;
        }

        ctx.beginPath();
        ctx.arc(centerXyview[0], centerXyview[1], radiusInPix, 0, 2*Math.PI, false);
        if (!noStroke) {
            ctx.stroke();
        }
    }; 
    
    return Circle;
})();


/***/ }),

/***/ "./src/js/Color.js":
/*!*************************!*\
  !*** ./src/js/Color.js ***!
  \*************************/
/*! exports provided: Color */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Color", function() { return Color; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Color
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let Color = (function() {


    let Color = {};
    
    Color.curIdx = 0;
    Color.colors = ['#ff0000', '#0000ff', '#99cc00', '#ffff00','#000066', '#00ffff', '#9900cc', '#0099cc', '#cc9900', '#cc0099', '#00cc99', '#663333', '#ffcc9a', '#ff9acc', '#ccff33', '#660000', '#ffcc33', '#ff00ff', '#00ff00', '#ffffff'];

    
    Color.getNextColor = function() {
        var c = Color.colors[Color.curIdx % (Color.colors.length)];
        Color.curIdx++;
        return c;
    };

    /** return most suited (ie readable) color for a label, given a background color
     * bkgdColor: color, given as a 'rgb(<r value>, <g value>, <v value>)' . This is returned by $(<element>).css('background-color')
     * 
     * example call: Color.getLabelColorForBackground('rgb(3, 123, 42)')
     * adapted from http://stackoverflow.com/questions/1855884/determine-font-color-based-on-background-color
     */
    Color.getLabelColorForBackground = function(rgbBkgdColor) {
        var lightLabel = '#eee' 
        var darkLabel = '#111' 
        var rgb = rgbBkgdColor.match(/^rgb\((\d+),\s*(\d+),\s*(\d+)\)$/);
        if (rgb==null) {
            // we return the dark label color if we can't parse the color
            return darkLabel
        }
        var r = parseInt(rgb[1]);
        var g = parseInt(rgb[2]);
        var b = parseInt(rgb[3]);
        
        var d = 0;
        // Counting the perceptive luminance - human eye favors green color... 
        var a = 1 - ( 0.299 * r + 0.587 * g + 0.114 * b) / 255;

        if (a < 0.5) {
            return darkLabel; // bright color --> dark font
        }
        else {
            return lightLabel; // dark color --> light font
        }
    };
    
    return Color;
})();



/***/ }),

/***/ "./src/js/ColorMap.js":
/*!****************************!*\
  !*** ./src/js/ColorMap.js ***!
  \****************************/
/*! exports provided: ColorMap */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ColorMap", function() { return ColorMap; });
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File ColorMap.js
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/



let ColorMap = (function() {
    
    
    // constructor
    let ColorMap = function(view) {
        this.view = view;
        this.reversed = false;
        this.mapName = 'native';
        this.sig = this.signature();
    };
    
ColorMap.MAPS = {};
    
    ColorMap.MAPS['eosb'] = {
            name: 'Eos B',
            r: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,9,18,27,36,45,49,57,72,81,91,100,109,118,127,
                136,131,139,163,173,182,191,200,209,218,227,213,221,255,255,255,255,255,
                255,255,255,229,229,255,255,255,255,255,255,255,255,229,229,255,255,255,
                255,255,255,255,255,229,229,255,255,255,255,255,255,255,255,229,229,255,
                255,255,255,255,255,255,255,229,229,255,255,255,255,255,255,255,255,229,
                229,255,255,255,255,255,255,255,255,229,229,255,255,255,255,255,255,255,
                255,229,229,255,255,255,255,255,255,255,255,229,229,255,253,251,249,247,
                245,243,241,215,214,235,234,232,230,228,226,224,222,198,196,216,215,213,
                211,209,207,205,203,181,179,197,196,194,192,190,188,186,184,164,162,178,
                176,175,173,171,169,167,165,147,145,159,157,156,154,152,150,148,146,130,
                128,140,138,137,135,133,131,129,127,113,111,121,119,117,117],
            g: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,15,23,31,39,47,55,57,64,79,87,95,
                103,111,119,127,135,129,136,159,167,175,183,191,199,207,215,200,207,239,
                247,255,255,255,255,255,255,229,229,255,255,255,255,255,255,255,255,229,
                229,255,255,255,255,255,255,255,255,229,229,255,250,246,242,238,233,229,
                225,198,195,212,208,204,199,195,191,187,182,160,156,169,165,161,157,153,
                148,144,140,122,118,127,125,123,121,119,116,114,112,99,97,106,104,102,
                99,97,95,93,91,80,78,84,82,80,78,76,74,72,70,61,59,63,61,59,57,55,53,50,
                48,42,40,42,40,38,36,33,31,29,27,22,21,21,19,16,14,12,13,8,6,3,1,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
            b: [116,121,127,131,136,140,144,148,153,
                157,145,149,170,174,178,182,187,191,195,199,183,187,212,216,221,225,229,
                233,238,242,221,225,255,247,239,231,223,215,207,199,172,164,175,167,159,
                151,143,135,127,119,100,93,95,87,79,71,63,55,47,39,28,21,15,7,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0]
    };
    ColorMap.MAPS['rainbow'] = {
            name: 'Rainbow',
            r: [0,4,9,13,18,22,27,31,36,40,45,50,54,
                58,61,64,68,69,72,74,77,79,80,82,83,85,84,86,87,88,86,87,87,87,85,84,84,
                84,83,79,78,77,76,71,70,68,66,60,58,55,53,46,43,40,36,33,25,21,16,12,4,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4,8,12,21,25,29,33,42,
                46,51,55,63,67,72,76,80,89,93,97,101,110,114,119,123,131,135,140,144,153,
                157,161,165,169,178,182,187,191,199,203,208,212,221,225,229,233,242,246,
                250,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255],
            g: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,4,8,16,21,25,29,38,42,46,51,55,63,67,72,76,84,89,93,97,
                106,110,114,119,127,131,135,140,144,152,157,161,165,174,178,182,187,195,
                199,203,208,216,220,225,229,233,242,246,250,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,250,242,238,233,229,221,216,212,208,199,195,191,187,178,174,170,165,
                161,153,148,144,140,131,127,123,119,110,106,102,97,89,85,80,76,72,63,59,
                55,51,42,38,34,29,21,17,12,8,0],
            b: [0,3,7,10,14,19,23,28,32,38,43,48,53,
                59,63,68,72,77,81,86,91,95,100,104,109,113,118,122,127,132,136,141,145,
                150,154,159,163,168,173,177,182,186,191,195,200,204,209,214,218,223,227,
                232,236,241,245,250,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,
                255,255,255,255,255,255,246,242,238,233,225,220,216,212,203,199,195,191,
                187,178,174,170,165,157,152,148,144,135,131,127,123,114,110,106,102,97,
                89,84,80,76,67,63,59,55,46,42,38,34,25,21,16,12,8,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
                0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    };
    ColorMap.MAPS['cubehelix'] = {
            name: 'Cubehelix',
            r: [0,1,3,4,6,8,9,10,12,13,14,15,17,18,
                19,20,20,21,22,23,23,24,24,25,25,25,26,26,26,26,26,26,26,26,26,26,26,25,
                25,25,25,24,24,24,23,23,23,23,22,22,22,21,21,21,21,21,21,20,20,20,21,21,
                21,21,21,22,22,22,23,23,24,25,26,27,27,28,30,31,32,33,35,36,38,39,41,43,
                45,47,49,51,53,55,57,60,62,65,67,70,72,75,78,81,83,86,89,92,95,98,101,104,
                107,110,113,116,120,123,126,129,132,135,138,141,144,147,150,153,155,158,
                161,164,166,169,171,174,176,178,181,183,185,187,189,191,193,194,196,198,
                199,201,202,203,204,205,206,207,208,209,209,210,211,211,211,212,212,212,
                212,212,212,212,212,211,211,211,210,210,210,209,208,208,207,207,206,205,
                205,204,203,203,202,201,201,200,199,199,198,197,197,196,196,195,195,194,
                194,194,193,193,193,193,193,193,193,193,193,193,194,194,195,195,196,196,
                197,198,199,200,200,202,203,204,205,206,208,209,210,212,213,215,217,218,
                220,222,223,225,227,229,231,232,234,236,238,240,242,244,245,247,249,251,
                253,255],
            g: [0,0,1,1,2,2,3,4,4,5,6,6,7,8,9,10,
                11,11,12,13,14,15,17,18,19,20,21,22,24,25,26,28,29,31,32,34,35,37,38,40,
                41,43,45,46,48,50,52,53,55,57,58,60,62,64,66,67,69,71,73,74,76,78,79,81,
                83,84,86,88,89,91,92,94,95,97,98,99,101,102,103,104,106,107,108,109,110,
                111,112,113,114,114,115,116,116,117,118,118,119,119,120,120,120,121,121,
                121,121,122,122,122,122,122,122,122,122,122,122,122,122,122,122,122,121,
                121,121,121,121,121,121,121,121,120,120,120,120,120,120,120,120,120,120,
                121,121,121,121,121,122,122,122,123,123,124,124,125,125,126,127,127,128,
                129,130,131,131,132,133,135,136,137,138,139,140,142,143,144,146,147,149,
                150,152,154,155,157,158,160,162,164,165,167,169,171,172,174,176,178,180,
                182,183,185,187,189,191,193,194,196,198,200,202,203,205,207,208,210,212,
                213,215,216,218,219,221,222,224,225,226,228,229,230,231,232,233,235,236,
                237,238,239,240,240,241,242,243,244,244,245,246,247,247,248,248,249,250,
                250,251,251,252,252,253,253,254,255],
            b: [0,1,3,4,6,8,9,11,13,15,17,19,21,23,
                25,27,29,31,33,35,37,39,41,43,45,47,48,50,52,54,56,57,59,60,62,63,65,66,
                67,69,70,71,72,73,74,74,75,76,76,77,77,77,78,78,78,78,78,78,78,77,77,77,
                76,76,75,75,74,73,73,72,71,70,69,68,67,66,66,65,64,63,61,60,59,58,58,57,
                56,55,54,53,52,51,51,50,49,49,48,48,47,47,47,46,46,46,46,46,47,47,47,48,
                48,49,50,50,51,52,53,55,56,57,59,60,62,64,65,67,69,71,74,76,78,81,83,86,
                88,91,94,96,99,102,105,108,111,114,117,120,124,127,130,133,136,140,143,
                146,149,153,156,159,162,165,169,172,175,178,181,184,186,189,192,195,197,
                200,203,205,207,210,212,214,216,218,220,222,224,226,227,229,230,231,233,
                234,235,236,237,238,239,239,240,241,241,242,242,242,243,243,243,243,243,
                243,243,243,243,243,242,242,242,242,241,241,241,241,240,240,240,239,239,
                239,239,239,238,238,238,238,238,238,238,238,239,239,239,240,240,240,241,
                242,242,243,244,245,246,247,248,249,250,252,253,255]
    };


    
    ColorMap.MAPS_CUSTOM = ['cubehelix', 'eosb', 'rainbow'];
    ColorMap.MAPS_NAMES = ['native', 'grayscale'].concat(ColorMap.MAPS_CUSTOM);
    
    ColorMap.prototype.reverse = function(val) {
        if (val) {
            this.reversed = val;
        }
        else {
            this.reversed = ! this.reversed;
        }
        this.sig = this.signature();
        this.view.requestRedraw();
    };
    
    
    ColorMap.prototype.signature = function() {
        var s = this.mapName;
        
        if (this.reversed) {
            s += ' reversed';
        }
        
        return s;
    };
    
    ColorMap.prototype.update = function(mapName) {
        this.mapName = mapName;
        this.sig = this.signature();
        this.view.requestRedraw();
    };
    
    ColorMap.prototype.apply = function(img) {
        if ( this.sig=='native' ) {
            return img;
        }
        
        if (img.cmSig==this.sig) {
            return img.cmImg; // return cached pixels
        }
        
        var canvas = document.createElement("canvas");
        canvas.width = img.width;
        canvas.height = img.height;
        var ctx = canvas.getContext("2d");
        ctx.drawImage(img, 0, 0);
        
        var imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        var pixelData = imageData.data;
        var length = pixelData.length;
        var a, b, c;
        var switchCase = 3;
        if (this.mapName=='grayscale') {
            switchCase = 1;
        }
        else if (ColorMap.MAPS_CUSTOM.indexOf(this.mapName)>=0) {
            switchCase = 2;
        }
        for (var i = 0; i < length; i+= 4) {
            switch(switchCase) {
                case 1:
                    a = b = c = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_0__["AladinUtils"].myRound((pixelData[i]+pixelData[i+1]+pixelData[i+2])/3);
                    break;
                case 2:
                    if (this.reversed) {
                        a = ColorMap.MAPS[this.mapName].r[255-pixelData[i]];
                        b = ColorMap.MAPS[this.mapName].g[255-pixelData[i+1]];
                        c = ColorMap.MAPS[this.mapName].b[255-pixelData[i+2]];
                    }
                    else {
                        a = ColorMap.MAPS[this.mapName].r[pixelData[i]];
                        b = ColorMap.MAPS[this.mapName].g[pixelData[i+1]];
                        c = ColorMap.MAPS[this.mapName].b[pixelData[i+2]];
                    }
                    break;
                default:
                    a = pixelData[i];
                    b = pixelData[i + 1];
                    c = pixelData[i + 2];
                    
            }
            if (switchCase!=2 && this.reversed) {
                a = 255-a;
                b = 255-b;
                c = 255-c;
              
            }
            pixelData[i]     = a;
            pixelData[i + 1] = b;
            pixelData[i + 2] = c;
            
        }
        //imageData.data = pixelData;  // not needed, and create an error in strict mode !
        ctx.putImageData(imageData, 0, 0);
        
        // cache image with color map applied
        img.cmSig = this.sig;
        img.cmImg = canvas;

        return img.cmImg;
    };
    
    return ColorMap;
})();
    


/***/ }),

/***/ "./src/js/CooConversion.js":
/*!*********************************!*\
  !*** ./src/js/CooConversion.js ***!
  \*********************************/
/*! exports provided: CooConversion */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "CooConversion", function() { return CooConversion; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



let CooConversion = (function() {

    let CooConversion = {};
    
    CooConversion.GALACTIC_TO_J2000 = [
       -0.0548755604024359,  0.4941094279435681, -0.8676661489811610,
       -0.8734370902479237, -0.4448296299195045, -0.1980763734646737,
       -0.4838350155267381,  0.7469822444763707,  0.4559837762325372 ];
    
    CooConversion.J2000_TO_GALACTIC = [
        -0.0548755604024359, -0.873437090247923, -0.4838350155267381,
         0.4941094279435681, -0.4448296299195045, 0.7469822444763707,
        -0.8676661489811610, -0.1980763734646737, 0.4559837762325372 ];
    
    // adapted from www.robertmartinayers.org/tools/coordinates.html
    // radec : array of ra, dec in degrees
    // return coo in degrees
    CooConversion.Transform = function( radec, matrix ) {// returns a radec array of two elements
        radec[0] = radec[0]*Math.PI/180;
        radec[1] = radec[1]*Math.PI/180;
      var r0 = new Array ( 
       Math.cos(radec[0]) * Math.cos(radec[1]),
       Math.sin(radec[0]) * Math.cos(radec[1]),
       Math.sin(radec[1]) );
        
     var s0 = new Array (
       r0[0]*matrix[0] + r0[1]*matrix[1] + r0[2]*matrix[2], 
       r0[0]*matrix[3] + r0[1]*matrix[4] + r0[2]*matrix[5], 
       r0[0]*matrix[6] + r0[1]*matrix[7] + r0[2]*matrix[8] ); 
     
      var r = Math.sqrt ( s0[0]*s0[0] + s0[1]*s0[1] + s0[2]*s0[2] ); 
    
      var result = new Array ( 0.0, 0.0 );
      result[1] = Math.asin ( s0[2]/r ); // New dec in range -90.0 -- +90.0 
      // or use sin^2 + cos^2 = 1.0  
      var cosaa = ( (s0[0]/r) / Math.cos(result[1] ) );
      var sinaa = ( (s0[1]/r) / Math.cos(result[1] ) );
      result[0] = Math.atan2 (sinaa,cosaa);
      if ( result[0] < 0.0 ) result[0] = result[0] + 2*Math.PI;
    
        result[0] = result[0]*180/Math.PI;
        result[1] = result[1]*180/Math.PI;
      return result;
    };
    
    // coo : array of lon, lat in degrees
    CooConversion.GalacticToJ2000 = function(coo) {
        return CooConversion.Transform(coo, CooConversion.GALACTIC_TO_J2000);
    };
    // coo : array of lon, lat in degrees
    CooConversion.J2000ToGalactic = function(coo) {
        return CooConversion.Transform(coo, CooConversion.J2000_TO_GALACTIC);
    };
    return CooConversion;
})();


/***/ }),

/***/ "./src/js/CooFrameEnum.js":
/*!********************************!*\
  !*** ./src/js/CooFrameEnum.js ***!
  \********************************/
/*! exports provided: CooFrameEnum */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "CooFrameEnum", function() { return CooFrameEnum; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File CooFrameEnum
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/
 
let CooFrameEnum = (function() {

    var systems = {J2000: 'J2000', GAL: 'Galactic'};
    return {
        SYSTEMS: systems,

        J2000: {label: "J2000", system: systems.J2000},
        J2000d: {label: "J2000d", system: systems.J2000},
        GAL:  {label: "Galactic", system: systems.GAL},

        fromString: function(str, defaultValue) {
            if (! str) {
                return defaultValue ? defaultValue : null;
            }
            
            str = str.toLowerCase().replace(/^\s+|\s+$/g, ''); // convert to lowercase and trim
            
            if (str.indexOf('j2000d')==0 || str.indexOf('icrsd')==0) {
                return CooFrameEnum.J2000d;
            }
            else if (str.indexOf('j2000')==0 || str.indexOf('icrs')==0) {
                return CooFrameEnum.J2000;
            }
            else if (str.indexOf('gal')==0) {
                return CooFrameEnum.GAL;
            }
            else {
                return defaultValue ? defaultValue : null;
            }
        }
    };
 
})();







/***/ }),

/***/ "./src/js/DiscoveryTree.js":
/*!*********************************!*\
  !*** ./src/js/DiscoveryTree.js ***!
  \*********************************/
/*! exports provided: DiscoveryTree */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "DiscoveryTree", function() { return DiscoveryTree; });
let DiscoveryTree = (function () {
    // Constructor
    var DiscoveryTree = function (aladin) {
        // activate Vue on the <div> that contains the component
        new Vue({
            el: '#ui',
            methods: {
                // Define the methods for the discovery-tree component
                // to interact with the aladin viewer
                getFovCorners() {
                    return aladin.getFovCorners();
                },
                getCenter() {
                    return aladin.getRaDec();
                },
                // Called when the user add a image survey
                addImage(metadata) {
                    const order = (+metadata.hips_order);
                    const hipsTileFormat = metadata.hips_tile_format.split(' ');
            
                    let tileFormat;
                    let color;
                    if (hipsTileFormat.indexOf('fits') >= 0) {
                        tileFormat = {
                            FITSImage: {
                                bitpix: parseInt(metadata.hips_pixel_bitpix)
                            }
                        };
                        color = {
                            Grayscale2Color: {
                                color: [1.0, 1.0, 1.0],
                                k: 1.0,
                                transfer: "asinh"
                            }
                        };
                    } else {
                        color = "Color";

                        if (hipsTileFormat.indexOf('png') >= 0) {
                            tileFormat = {
                                Image: {
                                    format: "png"
                                }
                            };
                        } else {
                            tileFormat = {
                                Image: {
                                    format: "jpeg"
                                }
                            };
                        }
                    }

                    let cuts = [undefined, undefined];
                    if (metadata.hips_pixel_cut) {
                        cuts = metadata.hips_pixel_cut.split(" ");
                    }
                    let tileSize = 512;
                    // Verify the validity of the tile width
                    if (metadata.hips_tile_width) {
                        let hipsTileWidth = parseInt(metadata.hips_tile_width);
                        let isPowerOfTwo = hipsTileWidth && !(hipsTileWidth & (hipsTileWidth - 1));

                        if (isPowerOfTwo === true) {
                            tileSize = hipsTileWidth;
                        }
                    }
                    let url = metadata.hips_service_url;
                    if (url.startsWith('http://alasky')) {
                        // From alasky one can directly use the https access
                        url = url.replace('http', 'https');
                    } else {
                        // Pass by a proxy for extern http urls
                        url = 'https://alasky.u-strasbg.fr/cgi/JSONProxy?url=' + url;
                    }
                    let survey = {
                        properties: {
                            url: url,
                            maxOrder:  parseInt(metadata.hips_order),
                            frame: {
                                label: "J2000",
                                system: "J2000"
                            },
                            tileSize: tileSize,
                            format: tileFormat,
                            minCutout: parseFloat(cuts[0]),
                            maxCutout: parseFloat(cuts[1]),
                        },
                        color: color
                    };

                    aladin.webglAPI.setHiPS([survey]);
                },
                // Called when the user add a catalog survey
                addCatalog(metadata, center, radius) {
                    if (metadata.hips_service_url) {
                        const hips = A.catalogHiPS(metadata.hips_service_url, {
                            onClick: 'showTable',
                            name: metadata.ID,
                        });
                        aladin.addCatalog(hips);
                    } else {
                        console.log(metadata.obs_id, "center, ", center, " radius, ", radius)
                        const catalog = A.catalogFromVizieR(
                            metadata.obs_id,
                            {
                                ra: center[0],
                                dec: center[1]
                            },
                            radius, {
                                onClick: 'showTable',
                                limit: 1000,
                            }
                        );
                        aladin.addCatalog(catalog);
                    }
                },
                // Called when the user add a HEALPix coverage
                addCoverage(metadata) {
                    const moc = A.MOCFromURL(metadata.moc_access_url);
                    aladin.addMOC(moc);
                },
            },
        });
    }

    return DiscoveryTree;
})();


/***/ }),

/***/ "./src/js/Downloader.js":
/*!******************************!*\
  !*** ./src/js/Downloader.js ***!
  \******************************/
/*! exports provided: Downloader */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Downloader", function() { return Downloader; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Downloader
 * Queue downloading for image elements
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let Downloader = (function() {

	var NB_MAX_SIMULTANEOUS_DL = 4;
	// TODO : le fading ne marche pas bien actuellement
	var FADING_ENABLED = false;
	var FADING_DURATION = 700; // in milliseconds
	
	
	let Downloader = function(view) {
		this.view = view; // reference to the view to be able to request redraw
		this.nbDownloads = 0; // number of current downloads
		this.dlQueue = []; // queue of items being downloaded
        this.urlsInQueue = {};
	};

	Downloader.prototype.emptyQueue = function() {
		this.dlQueue = [];
        this.urlsInQueue = {};
    };
	
	Downloader.prototype.requestDownload = function(img, url, cors) {
        // first check if url already in queue
        if (url in this.urlsInQueue)  {
            return;
        }
		// put in queue
		this.dlQueue.push({img: img, url: url, cors: cors});
		this.urlsInQueue[url] = 1;
		
		this.tryDownload();
	};
	
	// try to download next items in queue if possible
	Downloader.prototype.tryDownload = function() {
	    //if (this.dlQueue.length>0 && this.nbDownloads<NB_MAX_SIMULTANEOUS_DL) {
		while (this.dlQueue.length>0 && this.nbDownloads<NB_MAX_SIMULTANEOUS_DL) {
			this.startDownloadNext();
		}
	};
	
	Downloader.prototype.startDownloadNext = function() {
		// get next in queue
		var next = this.dlQueue.shift();
		if ( ! next) {
			return;
		}

		this.nbDownloads++;
		var downloaderRef = this;
		next.img.onload = function() {
			downloaderRef.completeDownload(this, true); // in this context, 'this' is the Image
		};
			
		next.img.onerror = function(e) {
			downloaderRef.completeDownload(this, false); // in this context, 'this' is the Image
		};
		if (next.cors) {
		    next.img.crossOrigin = 'anonymous';
		}
		
		else {
		    if (next.img.crossOrigin !== undefined) {
		        delete next.img.crossOrigin;
		    }
		}
		
		
		next.img.src = next.url;
	};
	
	Downloader.prototype.completeDownload = function(img, success) {
        delete this.urlsInQueue[img.src];
		img.onerror = null;
		img.onload = null;
		this.nbDownloads--;
		if (success) {
			if (FADING_ENABLED) {
				var now = new Date().getTime();
				img.fadingStart = now;
				img.fadingEnd = now + FADING_DURATION;
			}
			this.view.requestRedraw();
		}
		else {
		    img.dlError = true;
		}
		
		this.tryDownload();
	};
	
	
	
	return Downloader;
})();


/***/ }),

/***/ "./src/js/Ellipse.js":
/*!***************************!*\
  !*** ./src/js/Ellipse.js ***!
  \***************************/
/*! exports provided: Ellipse */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Ellipse", function() { return Ellipse; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _Aladin_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./Aladin.js */ "./src/js/Aladin.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Ellipse
 * 
 * Author: Matthieu Baumann[CDS]
 * 
 *****************************************************************************/






// TODO : Ellipse, Circle and Footprint should inherit from the same root object
let Ellipse = (function() {
    // constructor
    let Ellipse = function(centerRaDec, rayonXDegrees, rayonYDegrees, rotationDegrees, options) {
        options = options || {};

        this.color = options['color'] || undefined;

        // TODO : all graphic overlays should have an id
        this.id = 'ellipse-' + _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].uuidv4();

        this.setCenter(centerRaDec);
        this.setRadiuses(rayonXDegrees, rayonYDegrees);
        this.setRotation(rotationDegrees);
    	this.overlay = null;
    	
    	this.isShowing = true;
        this.isSelected = false;
    };

    Ellipse.prototype.setOverlay = function(overlay) {
        this.overlay = overlay;
    };
    
    Ellipse.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Ellipse.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Ellipse.prototype.dispatchClickEvent = function() {
        if (this.overlay) {
            // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
            //window.dispatchEvent(new CustomEvent("footprintClicked", {
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
        }
    };
    
    Ellipse.prototype.select = function() {
        if (this.isSelected) {
            return;
        }
        this.isSelected = true;
        if (this.overlay) {
/*
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
*/

            this.overlay.reportChange();
        }
    };

    Ellipse.prototype.deselect = function() {
        if (! this.isSelected) {
            return;
        }
        this.isSelected = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };


    
    Ellipse.prototype.setCenter = function(centerRaDec) {
        this.centerRaDec = centerRaDec;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    Ellipse.prototype.setRotation = function(rotationDegrees) {
        // radians
        let theta = rotationDegrees * Math.PI / 180;
        this.rotation = theta;
        // rotation in clockwise in the 2d canvas
        // we must transform it so that it is a north to east rotation
        //this.rotation = -theta - Math.PI/2;

        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    Ellipse.prototype.setRadiuses = function(radiusXDegrees, radiusYDegrees) {
        this.radiusXDegrees = radiusXDegrees;
        this.radiusYDegrees = radiusYDegrees;

        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    // TODO
    Ellipse.prototype.draw = function(ctx, view, projection, frame, width, height, largestDim, zoomFactor, noStroke) {
        if (! this.isShowing) {
            return;
        }
        noStroke = noStroke===true || false;

        var centerXyview = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__["AladinUtils"].radecToViewXy(this.centerRaDec[0], this.centerRaDec[1], view);
        if (!centerXyview) {
            // the center goes out of the projection
            // we do not draw it
            return;
        }

        let circlePtXyViewRa = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__["AladinUtils"].radecToViewXy(this.centerRaDec[0] + this.radiusXDegrees, this.centerRaDec[1], view);
        let circlePtXyViewDec = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_1__["AladinUtils"].radecToViewXy(this.centerRaDec[0], this.centerRaDec[1] + this.radiusYDegrees, view);

        if (!circlePtXyViewRa || !circlePtXyViewDec) {
            // the circle border goes out of the projection
            // we do not draw it
            return;
        }

        var dxRa = circlePtXyViewRa[0] - centerXyview[0];
        var dyRa = circlePtXyViewRa[1] - centerXyview[1];
        var radiusInPixX = Math.sqrt(dxRa*dxRa + dyRa*dyRa);

        var dxDec = circlePtXyViewDec[0] - centerXyview[0];
        var dyDec = circlePtXyViewDec[1] - centerXyview[1];
        var radiusInPixY = Math.sqrt(dxDec*dxDec + dyDec*dyDec);

        // Ellipse crossing the projection
        if ((dxRa*dyDec - dxDec*dyRa) <= 0.0) {
            // We do not draw it
            return;
        }
        // TODO : check each 4 point until show
        var baseColor = this.color;
        if (! baseColor && this.overlay) {
            baseColor = this.overlay.color;
        }
        if (! baseColor) {
            baseColor = '#ff0000';
        }
        
        if (this.isSelected) {
            ctx.strokeStyle= Overlay.increaseBrightness(baseColor, 50);
        }
        else {
            ctx.strokeStyle= baseColor;
        }

        // 1. Find the spherical tangent vector going to the north
        let origin = this.centerRaDec;
        let toNorth = [this.centerRaDec[0], this.centerRaDec[1] + 1e-3];

        // 2. Project it to the screen
        let originScreen = this.overlay.view.aladin.webglAPI.worldToScreen(origin[0], origin[1]);
        let toNorthScreen = this.overlay.view.aladin.webglAPI.worldToScreen(toNorth[0], toNorth[1]);

        // 3. normalize this vector
        let toNorthVec = [toNorthScreen[0] - originScreen[0], toNorthScreen[1] - originScreen[1]];
        let norm = Math.sqrt(toNorthVec[0]*toNorthVec[0] + toNorthVec[1]*toNorthVec[1]);
        
        toNorthVec = [toNorthVec[0] / norm, toNorthVec[1] / norm];
        let toWestVec = [1.0, 0.0];

        let x1 = toWestVec[0];
        let y1 = toWestVec[1];
        let x2 = toNorthVec[0];
        let y2 = toNorthVec[1];
        // 4. Compute the west to north angle
        let westToNorthAngle = Math.atan2(x1*y2-y1*x2, x1*x2+y1*y2);

        // 5. Get the correct ellipse angle
        let theta = -this.rotation + westToNorthAngle;

        ctx.beginPath();
        ctx.ellipse(centerXyview[0], centerXyview[1], radiusInPixX, radiusInPixY, theta, 0, 2*Math.PI, false);
        if (!noStroke) {
            ctx.stroke();
        }
    }; 
    
    return Ellipse;
})();


/***/ }),

/***/ "./src/js/Footprint.js":
/*!*****************************!*\
  !*** ./src/js/Footprint.js ***!
  \*****************************/
/*! exports provided: Footprint */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Footprint", function() { return Footprint; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Footprint
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/



let Footprint = (function() {
    // constructor
    let Footprint = function(polygons) {
        this.polygons = polygons;
    	this.overlay = null;

        // TODO : all graphic overlays should have an id
        this.id = 'footprint-' + _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].uuidv4();
    	
    	this.isShowing = true;
    	this.isSelected = false;
    };
    
    Footprint.prototype.setOverlay = function(overlay) {
        this.overlay = overlay;
    };
    
    Footprint.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Footprint.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };

    Footprint.prototype.dispatchClickEvent = function() {
        if (this.overlay) {
            // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
            //window.dispatchEvent(new CustomEvent("footprintClicked", {
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
        }
    };
    
    Footprint.prototype.select = function() {
        if (this.isSelected) {
            return;
        }
        this.isSelected = true;
        if (this.overlay) {
/*
            // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
            //window.dispatchEvent(new CustomEvent("footprintClicked", {
            this.overlay.view.aladinDiv.dispatchEvent(new CustomEvent("footprintClicked", {
                detail: {
                    footprintId: this.id,
                    overlayName: this.overlay.name
                }
            }));
*/

            this.overlay.reportChange();
        }
    };

    Footprint.prototype.deselect = function() {
        if (! this.isSelected) {
            return;
        }
        this.isSelected = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    return Footprint;
})();


/***/ }),

/***/ "./src/js/HealpixCache.js":
/*!********************************!*\
  !*** ./src/js/HealpixCache.js ***!
  \********************************/
/*! exports provided: HealpixCache */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "HealpixCache", function() { return HealpixCache; });
/* harmony import */ var _libs_healpix_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/healpix.js */ "./src/js/libs/healpix.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File HealpixCache
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

// class holding some HEALPix computations for better performances
//
// it is made of :
// - a static cache for HEALPix corners at nside=8 
// - a dynamic cache for 



let HealpixCache = (function() {

    let HealpixCache = {};
    
    HealpixCache.staticCache = {corners: {nside8: []}};
    // TODO : utilisation du dynamicCache
    HealpixCache.dynamicCache = {};
    
    HealpixCache.lastNside = 8;
    
    HealpixCache.hpxIdxCache = null;
    
    // TODO : conserver en cache le dernier résultat ?
    
    HealpixCache.init = function() {
    	// pre-compute corners position for nside=8
    	var hpxIdx = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_0__["HealpixIndex"](8);
    	hpxIdx.init();
    	var npix = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_0__["HealpixIndex"].nside2Npix(8);
        let corners;
    	for (var ipix=0; ipix<npix; ipix++) {
            corners =  hpxIdx.corners_nest(ipix, 1);
    		HealpixCache.staticCache.corners.nside8.push(corners);
    	}
    	
    	HealpixCache.hpxIdxCache = hpxIdx;
    };

    HealpixCache.init();
    
    HealpixCache.corners_nest = function(ipix, nside) {
    	if (nside==8) {
    		return HealpixCache.staticCache.corners.nside8[ipix];
    	}
    	
    	if (nside != HealpixCache.lastNside) {
    		HealpixCache.hpxIdxCache = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_0__["HealpixIndex"](nside);
    		HealpixCache.hpxIdxCache.init();
    		HealpixCache.lastNside = nside;
    	}
    	
    	return HealpixCache.hpxIdxCache.corners_nest(ipix, 1);
    	
    };
    
    return HealpixCache;
})();
	


/***/ }),

/***/ "./src/js/HealpixGrid.js":
/*!*******************************!*\
  !*** ./src/js/HealpixGrid.js ***!
  \*******************************/
/*! exports provided: HealpixGrid */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "HealpixGrid", function() { return HealpixGrid; });
// Copyright 2015 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File HealpixGrid
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let HealpixGrid = (function() {
	function HealpixGrid() {
	}
	
	HealpixGrid.prototype.redraw = function(ctx, cornersXYViewMap, fov, norder) {
		// on dessine les lignes
		ctx.lineWidth = 1;
		ctx.strokeStyle = "rgb(150,150,220)";
		ctx.beginPath();
		var cornersXYView;
		var ipix;
		for (var k=0, len=cornersXYViewMap.length; k<len; k++) {
			cornersXYView = cornersXYViewMap[k];
			ipix = cornersXYView.ipix;
			
			// draw pixel
			ctx.moveTo(cornersXYView[0].vx, cornersXYView[0].vy);
			ctx.lineTo(cornersXYView[1].vx, cornersXYView[1].vy);
			ctx.lineTo(cornersXYView[2].vx, cornersXYView[2].vy);
			//ctx.lineTo(cornersXYView[3].vx, cornersXYView[3].vy);
			

            //ctx.strokeText(ipix, (cornersXYView[0].vx + cornersXYView[2].vx)/2, (cornersXYView[0].vy + cornersXYView[2].vy)/2);
		}
		ctx.stroke();
		
		// on dessine les numéros de pixel HEALpix
        ctx.strokeStyle="#FFDDDD";
		ctx.beginPath();
		for (var k=0, len=cornersXYViewMap.length; k<len; k++) {
			cornersXYView = cornersXYViewMap[k];
			ipix = cornersXYView.ipix;

            ctx.strokeText(norder + '/' + ipix, (cornersXYView[0].vx + cornersXYView[2].vx)/2, (cornersXYView[0].vy + cornersXYView[2].vy)/2);
		}
		ctx.stroke();
	};

	
	
	return HealpixGrid;
})();


/***/ }),

/***/ "./src/js/HiPSDefinition.js":
/*!**********************************!*\
  !*** ./src/js/HiPSDefinition.js ***!
  \**********************************/
/*! exports provided: HiPSDefinition */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "HiPSDefinition", function() { return HiPSDefinition; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
// Copyright 2013-2017 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File HiPSDefinition
 * 
 * Author: Thomas Boch [CDS]
 * 
 *****************************************************************************/



let HiPSDefinition = (function() {

    // constructor
    function HiPSDefinition(properties) {
        this.properties = properties; // key-value object corresponding to the properties file

        this.id = this.getID();
        this.obsTitle = properties['obs_title'];
        this.frame = properties['hips_frame'];
        this.order = parseInt(properties['hips_order']);
        this.clientSortKey = properties['client_sort_key'];
        this.tileFormats = properties.hasOwnProperty('hips_tile_format') && properties['hips_tile_format'].split(' ');
        this.urls = [];
        this.urls.push(properties['hips_service_url']);
        var k = 1;
        while (properties.hasOwnProperty('hips_service_url_' + k)) {
            this.urls.push(properties['hips_service_url_' + k]);
            k++;
        }

        this.clientApplications = properties['client_application'];
    };

    HiPSDefinition.prototype = {

        getServiceURLs: function(httpsOnly) {
            httpsOnly = httpsOnly === true;

            // TODO: TO BE COMPLETED
        },

        // return the ID according to the properties
        getID: function() {
            // ID is explicitely given
            if (this.properties.hasOwnProperty('ID')) {
                return this.properties['ID'];
            }

            var id = null;
            // ID might be built from different fields
            if (this.properties.hasOwnProperty('creator_did')) {
                id = this.properties['creator_did'];
            }
            if (id==null && this.properties.hasOwnProperty('publisher_did')) {
                id = this.properties['publisher_did'];
            }

            if (id != null) {
                // remove ivo:// prefix
                if (id.slice(0, 6) === 'ivo://') {
                    id = id.slice(6);
                }

                // '?' are replaced by '/'
                id = id.replace(/\?/g, '/')
            }

            return id;
        }



    };

    // cache (at the source code level) of the list of HiPS
    // this is the result to a query to http://alasky.u-strasbg.fr/MocServer/query?dataproduct_type=image&client_application=AladinLite&fmt=json&fields=ID,obs_title,client_sort_key,client_application,hips_service_url*,hips_order,hips_tile_format,hips_frame
    var AL_CACHE_CLASS_LEVEL = [
    /*{
    "ID": "CDS/P/2MASS/color",
    "obs_title": "2MASS color J (1.23 microns), H (1.66 microns), K (2.16 microns)",
    "client_sort_key": "04-001-00",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/2MASS/Color",
    "hips_service_url_1": "http://alaskybis.unistra.fr/2MASS/Color",
    "hips_service_url_2": "https://alaskybis.unistra.fr/2MASS/Color"
    }, {
    "ID": "CDS/P/AKARI/FIS/Color",
    "obs_title": "AKARI Far-infrared All-Sky Survey - color composition WideL/WideS/N60",
    "client_sort_key": "04-05-00",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "5",
    "hips_frame": "equatorial",
    "hips_tile_format": "png jpeg",
    "hips_service_url": "http://alasky.unistra.fr/AKARI-FIS/ColorLSN60",
    "hips_service_url_1": "http://alaskybis.unistra.fr/AKARI-FIS/ColorLSN60",
    "hips_service_url_2": "https://alaskybis.unistra.fr/AKARI-FIS/ColorLSN60"
    }, {
    "ID": "CDS/P/DECaLS/DR3/color",
    "obs_title": "DECaLS DR3 color",
    "hips_frame": "equatorial",
    "hips_order": "11",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/DECaLS/DR3/color"
}, {
    "ID": "CDS/P/DSS2/blue",
    "obs_title": "DSS2 Blue (XJ+S)",
    "client_sort_key": "03-01-03",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg fits",
    "hips_service_url": "http://alasky.unistra.fr/DSS/DSS2-blue-XJ-S",
    "hips_service_url_1": "http://alaskybis.unistra.fr/DSS/DSS2-blue-XJ-S",
    "hips_service_url_2": "https://alaskybis.unistra.fr/DSS/DSS2-blue-XJ-S",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/DSS2Blue"
}, {
    "ID": "CDS/P/DSS2/color",
    "obs_title": "DSS colored",
    "client_sort_key": "03-00",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/DSS/DSSColor",
    "hips_service_url_1": "http://alaskybis.unistra.fr/DSS/DSSColor",
    "hips_service_url_2": "https://alaskybis.unistra.fr/DSS/DSSColor",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/DSSColorNew",
    "hips_service_url_4": "http://skies.esac.esa.int/DSSColor/"
}, {
    "ID": "CDS/P/DSS2/red",
    "obs_title": "DSS2 Red (F+R)",
    "client_sort_key": "03-01-02",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg fits",
    "hips_service_url": "http://alasky.unistra.fr/DSS/DSS2Merged",
    "hips_service_url_1": "http://alaskybis.unistra.fr/DSS/DSS2Merged",
    "hips_service_url_2": "https://alaskybis.unistra.fr/DSS/DSS2Merged",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/DSS2Merged"
}, {
    "ID": "P/PanSTARRS/DR1/g",
    "hips_service_url": "http://alasky.u-strasbg.fr/Pan-STARRS/DR1/g",
    "obs_title": "PanSTARRS DR1 g",
    "hips_order": 11,
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg fits"
}, {
    "ID": "CDS/P/Fermi/color",
    "obs_title": "Fermi Color HEALPix survey",
    "client_sort_key": "00-01-01",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "3",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/Fermi/Color",
    "hips_service_url_1": "http://alaskybis.unistra.fr/Fermi/Color",
    "hips_service_url_2": "https://alaskybis.unistra.fr/Fermi/Color"
}, {
    "ID": "CDS/P/Finkbeiner",
    "obs_title": "Finkbeiner Halpha composite survey",
    "client_sort_key": "06-01",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "3",
    "hips_frame": "galactic",
    "hips_tile_format": "jpeg fits",
    "hips_service_url": "http://alasky.unistra.fr/FinkbeinerHalpha",
    "hips_service_url_1": "http://alaskybis.unistra.fr/FinkbeinerHalpha",
    "hips_service_url_2": "https://alaskybis.unistra.fr/FinkbeinerHalpha"
}, {
    "ID": "CDS/P/GALEXGR6/AIS/color",
    "obs_title": "GALEX GR6 AIS (until March 2014)- Color composition",
    "client_sort_key": "02-01-01",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "8",
    "hips_frame": "equatorial",
    "hips_tile_format": "png jpeg",
    "hips_service_url": "http://alasky.unistra.fr/GALEX/GR6-03-2014/AIS-Color",
    "hips_service_url_1": "http://alaskybis.unistra.fr/GALEX/GR6-03-2014/AIS-Color",
    "hips_service_url_2": "https://alaskybis.unistra.fr/GALEX/GR6-03-2014/AIS-Color"
}, {
    "ID": "CDS/P/IRIS/color",
    "obs_title": "IRAS-IRIS HEALPix survey, color",
    "client_sort_key": "04-02-01",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "3",
    "hips_frame": "galactic",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/IRISColor",
    "hips_service_url_1": "http://alaskybis.unistra.fr/IRISColor",
    "hips_service_url_2": "https://alaskybis.unistra.fr/IRISColor",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/IRISColor",
    "hips_service_url_4": "http://skies.esac.esa.int/IRISColor/"
}, {
    "ID": "CDS/P/Mellinger/color",
    "obs_title": "Mellinger optical survey, color",
    "client_sort_key": "03-03",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "4",
    "hips_frame": "galactic",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/MellingerRGB",
    "hips_service_url_1": "http://alaskybis.unistra.fr/MellingerRGB",
    "hips_service_url_2": "https://alaskybis.unistra.fr/MellingerRGB"
}, {
    "ID": "CDS/P/SDSS9/color",
    "obs_title": "SDSS 9 color",
    "client_sort_key": "03-02-01",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "10",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/SDSS/DR9/color",
    "hips_service_url_1": "http://alaskybis.unistra.fr/SDSS/DR9/color",
    "hips_service_url_2": "https://alaskybis.unistra.fr/SDSS/DR9/color",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/SDSS9Color",
    "hips_service_url_4": "http://skies.esac.esa.int/SDSS9Color/"
}, {
    "ID": "CDS/P/SPITZER/color",
    "obs_title": "IRAC HEALPix survey, color",
    "client_sort_key": "04-03-00",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "galactic",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/SpitzerI1I2I4color",
    "hips_service_url_1": "http://alaskybis.unistra.fr/SpitzerI1I2I4color",
    "hips_service_url_2": "https://alaskybis.unistra.fr/SpitzerI1I2I4color",
    "hips_service_url_3": "http://healpix.ias.u-psud.fr/SPITZERColor"
}, {
    "ID": "CDS/P/allWISE/color",
    "obs_title": "AllWISE color  Red (W4) , Green (W2) , Blue (W1) from raw Atlas Images",
    "client_sort_key": "04-003-00",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "8",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://alasky.unistra.fr/AllWISE/RGB-W4-W2-W1",
    "hips_service_url_1": "http://alaskybis.unistra.fr/AllWISE/RGB-W4-W2-W1",
    "hips_service_url_2": "https://alaskybis.unistra.fr/AllWISE/RGB-W4-W2-W1"
}, {
    "ID": "IPAC/P/GLIMPSE360",
    "obs_title": "GLIMPSE360: Spitzer's Infrared Milky Way",
    "client_sort_key": "04-03-0",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "9",
    "hips_frame": "equatorial",
    "hips_tile_format": "jpeg",
    "hips_service_url": "http://www.spitzer.caltech.edu/glimpse360/aladin/data"
}, {
    "ID": "JAXA/P/MAXI_SSC_SUM",
    "hips_tile_format": "png",
    "hips_frame": "equatorial",
    "obs_title": "MAXI SSC all-sky image integrated for 4.5 years",
    "hips_order": "6",
    "hips_service_url": "http://darts.isas.jaxa.jp/pub/judo2/HiPS/maxi_ssc_sum",
    "hips_service_url_1": "http://alasky.unistra.fr//JAXA/JAXA_P_MAXI_SSC_SUM",
    "hips_service_url_2": "http://alaskybis.unistra.fr//JAXA/JAXA_P_MAXI_SSC_SUM",
    "hips_service_url_3": "https://alaskybis.unistra.fr//JAXA/JAXA_P_MAXI_SSC_SUM"
}, {
    "ID": "JAXA/P/SWIFT_BAT_FLUX",
    "hips_tile_format": "png",
    "hips_frame": "equatorial",
    "obs_title": "Swift-BAT 70-month all-sray hard X-ray survey image",
    "hips_order": "6",
    "hips_service_url": "http://darts.isas.jaxa.jp/pub/judo2/HiPS/swift_bat_flux/",
    "hips_service_url_1": "http://alasky.unistra.fr//JAXA/JAXA_P_SWIFT_BAT_FLUX",
    "hips_service_url_2": "http://alaskybis.unistra.fr//JAXA/JAXA_P_SWIFT_BAT_FLUX",
    "hips_service_url_3": "https://alaskybis.unistra.fr//JAXA/JAXA_P_SWIFT_BAT_FLUX"
}, {
    "ID": "ov-gso/P/VTSS/Ha",
    "obs_title": "Virginia Tech Spectral-Line Survey (VTSS) - Halpha image",
    "client_sort_key": "06-xx",
    "client_application":[ "AladinLite", "AladinDesktop"],
    "hips_order": "3",
    "hips_frame": ["galactic", "galactic"],
    "hips_tile_format": "png jpeg fits",
    "hips_service_url": "http://cade.irap.omp.eu/documents/Ancillary/4Aladin/VTSS",
    "hips_service_url_1": "http://alasky.unistra.fr/IRAP/VTSS",
    "hips_service_url_2": "http://alaskybis.unistra.fr/IRAP/VTSS",
    "hips_service_url_3": "https://alaskybis.unistra.fr/IRAP/VTSS"
}, {
    "ID": "xcatdb/P/XMM/EPIC",
    "obs_title": "XMM-Newton stacked EPIC images",
    "hips_frame": "equatorial",
    "hips_order": "7",
    "hips_service_url": "http://saada.u-strasbg.fr/xmmallsky",
    "hips_tile_format": "png fits",
    "hips_service_url_1": "http://alasky.unistra.fr/SSC/xmmallsky",
    "hips_service_url_2": "http://alaskybis.unistra.fr/SSC/xmmallsky",
    "hips_service_url_3": "https://alaskybis.unistra.fr/SSC/xmmallsky"
}, {
    "ID": "xcatdb/P/XMM/PN/color",
    "obs_title": "False color X-ray images (Red=0.5-1 Green=1-2 Blue=2-4.5)Kev",
    "hips_order": "7",
    "hips_frame": "equatorial",
    "hips_tile_format": "png jpeg",
    "hips_service_url": "http://saada.unistra.fr/PNColor",
    "hips_service_url_1": "http://alasky.u-strasbg.fr/SSC/xcatdb_P_XMM_PN_color",
    "hips_service_url_2": "http://alaskybis.u-strasbg.fr/SSC/xcatdb_P_XMM_PN_color",
    "hips_service_url_3": "https://alaskybis.u-strasbg.fr/SSC/xcatdb_P_XMM_PN_color"
}
*/
];

    var listHipsProperties = []; // this variable stores our current knowledge

    HiPSDefinition.LOCAL_STORAGE_KEY = 'aladin:hips-list';
    
    var RETRIEVAL_TIMESTAMP_KEY = '_timestamp_retrieved';
    var LAST_URL_KEY = '_last_used_url'; // URL previousy used to retrieve data from this HiPS
    // retrieve definitions previousy stored in local storage
    // @return an array with the HiPS definitions, empty array if nothing found or if an error occured
    HiPSDefinition.getLocalStorageDefinitions = function() {
        try {
            var defs = window.localStorage.getItem(HiPSDefinition.LOCAL_STORAGE_KEY);
            return defs === null ? [] : window.JSON.parse(defs);
        }
        catch(e) {
            // silently fail and return empty array
            return [];
        }
    };

    // store in local storage a list of HiPSDefinition objects
    // @return true if storage was successful
    HiPSDefinition.storeInLocalStorage = function(properties) {
        try {
            window.localStorage.setItem(HiPSDefinition.LOCAL_STORAGE_KEY, window.JSON.stringify(properties));
        }
        catch(e) {
            // silently fail and return false
            return false;
        }

        return true;
    };

    var MOCSERVER_MIRRORS_HTTP = ['http://alasky.u-strasbg.fr/MocServer/query', 'http://alaskybis.u-strasbg.fr/MocServer/query']; // list of base URL for MocServer mirrors, available in HTTP
    var MOCSERVER_MIRRORS_HTTPS = ['https://alasky.u-strasbg.fr/MocServer/query', 'https://alaskybis.unistra.fr/MocServer/query']; // list of base URL for MocServer mirrors, available in HTTPS

    // get HiPS definitions, by querying the MocServer
    // return data as dict-like objects
    HiPSDefinition.getRemoteDefinitions = function(params, successCallbackFn, failureCallbackFn) {
        var params = params || {client_application: 'AladinLite'}; // by default, retrieve only HiPS tagged "Aladin Lite"

        params['fmt'] = 'json';
        params['fields'] = 'ID,obs_title,client_sort_key,client_application,hips_service_url*,hips_order,hips_tile_format,hips_frame';

        var urls = _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].isHttpsContext() ? MOCSERVER_MIRRORS_HTTPS : MOCSERVER_MIRRORS_HTTP;

        var successCallback = function(data) {
            (typeof successCallbackFn === 'function') && successCallbackFn(data);
        };
        var failureCallback = function() {
            console.error('Could not load HiPS definitions from urls ' + urls);
            (typeof failureCallbackFn === 'function') && failureCallbackFn();
        };

        _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].loadFromMirrors(urls, {data: params, onSuccess: successCallback, onFailure: failureCallback, timeout: 5});
    };

    // complement the baseList with the items in newList
    var merge = function(baseList, newList) {
        var updatedList = [];
        var newListById = {};
        for (var k=0; k<newList.length; k++) {
            var item = newList[k];
            newListById[item.ID] = item;
        }

        for (var k=0; k<baseList.length; k++) {
            var item = baseList[k];
            var id = item.ID;
            if (newListById.hasOwnProperty(id)) {
                var itemToAdd = newListById[id];
                // we keep the last used URL property
                if (item.hasOwnProperty(LAST_URL_KEY) && ! itemToAdd.hasOwnProperty(LAST_URL_KEY)) {
                    itemToAdd[LAST_URL_KEY] = item[LAST_URL_KEY];
                }
                updatedList.push(itemToAdd);
            }
            else {
                updatedList.push(item);
            }
        }

        return updatedList;
    };

    HiPSDefinition.CACHE_RETENTION_TIME_SECONDS = 7 * 86400; // definitions can be kept 7 days
    HiPSDefinition.init = function() {
        // first, merge local definitions at class level with definitions in local storage
        listHipsProperties = AL_CACHE_CLASS_LEVEL;

        // second, remove old definitions (client != AladinLite and timestamp older than CACHE_RETENTION_TIME_SECONDS) and merge
        var localDefs = HiPSDefinition.getLocalStorageDefinitions();
        // 2.1 remove old defs
        var now = new Date().getTime();
        var indicesToRemove = [];
        for (var k=0; k<localDefs.length; k++) {
            var def = localDefs[k];
            if (def.hasOwnProperty(RETRIEVAL_TIMESTAMP_KEY) && (now - def[RETRIEVAL_TIMESTAMP_KEY]) > 1000 * HiPSDefinition.CACHE_RETENTION_TIME_SECONDS) {
                indicesToRemove.push(k);
            }
        }
        // we have to browse the array in reverse order in order not to mess up indices
        for (var k = indicesToRemove.length - 1; k >= 0; k--) {
            localDefs.splice(indicesToRemove[k],1);
        }
        // 2.2 merge
        listHipsProperties = merge(listHipsProperties, localDefs);

        // third, retrieve remote definitions, merge and save
        HiPSDefinition.getRemoteDefinitions({dataproduct_type: 'image', client_application: 'AladinLite'}, function(remoteDefs) {
            // adding timestamp of retrieval
            var now = new Date().getTime();
            for (var k=0; k<remoteDefs.length; k++) {
                remoteDefs[k][RETRIEVAL_TIMESTAMP_KEY] = now;
            }
            listHipsProperties = merge(listHipsProperties, remoteDefs);
            HiPSDefinition.storeInLocalStorage(listHipsProperties);
        });

    };

    // return list of HiPSDefinition objects, filtering out definitions whose client_application is not AladinLite
    HiPSDefinition.getALDefaultHiPSDefinitions = function() {
        // filter out definitions with client_application != 'AladinLite'
        var ret = [];
        for (var k=0; k<listHipsProperties.length; k++) {
            var properties = listHipsProperties[k];
            if ( ! properties.hasOwnProperty('client_application') || properties['client_application'].indexOf('AladinLite')<0) {
                continue;
            }

            ret.push(new HiPSDefinition(properties));
        }

        return ret;
    };

    // return list of known HiPSDefinition objects
    HiPSDefinition.getDefinitions = function() {
        var ret = [];
        for (var k=0; k<listHipsProperties.length; k++) {
            var properties = listHipsProperties[k];
            ret.push(new HiPSDefinition(properties));
        }

        return ret;
    };

    // parse a HiPS properties and return a dict-like object with corresponding key-values
    // return null if parsing failed
    HiPSDefinition.parseHiPSProperties = function(propertiesStr) {
        if (propertiesStr==null) {
            return null;
        }

        var propertiesDict = {};
        // remove CR characters
        propertiesStr = propertiesStr.replace(/[\r]/g, '');
        // split on LF
        var lines = propertiesStr.split('\n');
        for (var k=0; k<lines.length; k++)  {
            var l = $.trim(lines[k]);
            // ignore comments lines
            if (l.slice(0, 1)==='#') {
                continue;
            }
            var idx = l.indexOf('=');
            if (idx<0) {
                continue;
            }
            var key = $.trim(l.slice(0, idx));
            var value = $.trim(l.slice(idx+1));

            propertiesDict[key] = value;
        }

        return propertiesDict;
    };


    // find a HiPSDefinition by id.
    // look first locally, and remotely only if local search was unsuccessful
    //
    // call callback function with a list of HiPSDefinition candidates, empty array if nothing found

    HiPSDefinition.findByID = function(id, callback) {
        // look first locally
        var candidates = findByIDLocal(id);
        if (candidates.length>0) {
            (typeof callback === 'function') && callback(candidates);
            return;
        }

        // then remotely
        findByIDRemote(id, callback);
    };

    // find a HiPSDefinition by id.
    // search is done on the local knowledge of HiPSDefinitions
    HiPSDefinition.findByIDLocal = function(id2search, callback) {
        var candidates = [];
        for (var k=0; k<listHipsProperties.length; k++) {
            var properties = listHipsProperties[k];
            var id = properties['ID'];
            if (id.match(id2search) != null ) {
                candidates.push(new HiPSDefinition(properties));
            }
        }

        return candidates;
    };

    // find remotely a HiPSDefinition by ID
    HiPSDefinition.findByIDRemote = function(id, callback) {
        HiPSDefinition.findHiPSRemote({ID: '*' + id + '*'}, callback);
    };

    // search a HiPS according to some criteria
    HiPSDefinition.findHiPSRemote = function(searchOptions, callback) {
        searchOptions = searchOptions || {};
        if (! searchOptions.hasOwnProperty('dataproduct_type')) {
            searchOptions['dataproduct_type'] = 'image';
        }
        HiPSDefinition.getRemoteDefinitions(searchOptions, function(candidates) {
            var defs = [];
            for (var k=0; k<candidates.length; k++) {
                defs.push(new HiPSDefinition(candidates[k]));
            }
            (typeof callback === 'function') && callback(defs);
        });
    };


    // Create a HiPSDefinition object from a URL
    //
    // If the URL ends with 'properties', it is assumed to be the URL of the properties file
    // else, it is assumed to be the base URL of the HiPS
    //
    // return a HiPSDefinition if successful, null if it failed
    HiPSDefinition.fromURL = function(url, callback) {
        var hipsUrl, propertiesUrl;
        if (url.slice(-10) === 'properties') {
            propertiesUrl = url;
            hipsUrl = propertiesUrl.slice(0, -11);
        }
        else {
            if (url.slice(-1) === '/') {
                url = url.slice(0, -1);
            }
            hipsUrl = url;
            propertiesUrl = hipsUrl + '/properties';
        }

        var callbackWhenPropertiesLoaded = function(properties) {
            // Sometimes, hips_service_url is missing. That can happen for instance Hipsgen does not set the hips_service_url keyword
            // --> in that case, we add as an attribyte the URL that was given as input parameter
            var hipsPropertiesDict = HiPSDefinition.parseHiPSProperties(properties);
            if (! hipsPropertiesDict.hasOwnProperty('hips_service_url')) {
                hipsPropertiesDict['hips_service_url'] = hipsUrl;
            }
            (typeof callback === 'function') && callback(new HiPSDefinition(hipsPropertiesDict));
        };

        // try first without proxy
        var ajax = _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].getAjaxObject(propertiesUrl, 'GET', 'text', false);
        ajax
            .done(function(data) {
                callbackWhenPropertiesLoaded(data);
            })
            .fail(function() {
                // if not working, try with the proxy
                var ajax = _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].getAjaxObject(propertiesUrl, 'GET', 'text', true);
                ajax
                    .done(function(data) {
                        callbackWhenPropertiesLoaded(data);
                    })
                    .fail(function() {
                        (typeof callback === 'function') && callback(null);
                    })
            });
    };

    // HiPSDefinition generation from a properties dict-like object
    HiPSDefinition.fromProperties = function(properties) {
        return new HiPSDefinition(properties);
    };




    HiPSDefinition.init();

    return HiPSDefinition;

})();



/***/ }),

/***/ "./src/js/HpxImageSurvey.js":
/*!**********************************!*\
  !*** ./src/js/HpxImageSurvey.js ***!
  \**********************************/
/*! exports provided: HpxImageSurvey */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "HpxImageSurvey", function() { return HpxImageSurvey; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./HiPSDefinition.js */ "./src/js/HiPSDefinition.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File HpxImageSurvey
 * 
 * Authors: Thomas Boch & Matthieu Baumann [CDS]
 * 
 *****************************************************************************/



let HpxImageSurvey = (function() {
    /** Constructor
     * cooFrame and maxOrder can be set to null
     * They will be determined by reading the properties file
     *  
     */
    let HpxImageSurvey = function(rootURLOrId) {
        if (!rootURLOrId) {
            throw 'An hosting survey URL or an ID (i.e. DSS2/red) must be given';
        }

        let isUrl = false;
        console.log("root url", rootURLOrId)
        if (rootURLOrId.includes("http")) {
            isUrl = true;
        }

        const request = async (url) => {
            const response = await fetch(url);
            const json = await response.json();

            return json;
        };

        // If an HiPS id has been given
        let url = null;
        if (!isUrl) {
            // Use the MOCServer to retrieve the
            // properties
            const id = rootURLOrId;
            const MOCServerUrl = 'http://alasky.unistra.fr/MocServer/query?ID=*' + encodeURIComponent(id) + '*&get=record&fmt=json';

            return (async () => {
                let metadata = await request(MOCServerUrl);

                // We get the property here

                // 1. Ensure there is exactly one survey matching
                if (!metadata) {
                    throw 'no surveys matching';
                } else {
                    if (metadata.length > 1) {
                        let ids = [];
                        metadata.forEach((prop) => {
                            ids.push(prop.ID)
                        });
                        throw ids + ' surveys are matching. Please use one from this list.';
                    } else if (metadata.length === 0) {
                        throw 'no surveys matching';
                    } else {
                        // Exactly one matching
                        metadata = metadata[0];
                    }
                }
                // Let is build the survey object
                const survey = HpxImageSurvey.parseSurveyProperties(metadata);
                return survey
            })();
        } else {
            // Fetch the properties of the survey
            let rootURL = rootURLOrId;
            // Use the url for retrieving the HiPS properties
            // remove final slash
            if (rootURL.slice(-1) === '/') {
                rootURL = rootURL.substr(0, rootURL.length-1);
            }

            // make URL absolute
            rootURL = _Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].getAbsoluteURL(rootURL);

            // fast fix for HTTPS support --> will work for all HiPS served by CDS
            if (_Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].isHttpsContext() && ( /u-strasbg.fr/i.test(rootURL) || /unistra.fr/i.test(rootURL)  ) ) {
                rootURL = rootURL.replace('http://', 'https://');
            }

            console.log("ROOT URL", rootURL);
            url = rootURL + '/properties';


            return (async () => {
                console.log("properties url", url);
                let metadata = await fetch(url)
                    .then((response) => response.text());
                // We get the property here
                metadata = _HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_1__["HiPSDefinition"].parseHiPSProperties(metadata);
                console.log("metadata", metadata);

                // 1. Ensure there is exactly one survey matching
                if (!metadata) {
                    throw 'no surveys matching';
                }
                // Let is build the survey object
                const survey = HpxImageSurvey.parseSurveyProperties(metadata);
                console.log("survey ", survey);

                return survey
            })();
        }
    };

    HpxImageSurvey.parseSurveyProperties = function(metadata) {
        const order = (+metadata.hips_order);
        const hipsTileFormat = metadata.hips_tile_format.split(' ');

        let tileFormat;
        let color;
        if (hipsTileFormat.indexOf('fits') >= 0) {
            tileFormat = {
                FITSImage: {
                    bitpix: parseInt(metadata.hips_pixel_bitpix)
                }
            };
            color = {
                Grayscale2Color: {
                    color: [1.0, 1.0, 1.0],
                    k: 1.0,
                    transfer: "asinh"
                }
            };
        } else {
            color = "Color";

            if (hipsTileFormat.indexOf('png') >= 0) {
                tileFormat = {
                    Image: {
                        format: "png"
                    }
                };
            } else {
                tileFormat = {
                    Image: {
                        format: "jpeg"
                    }
                };
            }
        }


        let cuts = [undefined, undefined];
        if (metadata.hips_pixel_cut) {
            cuts = metadata.hips_pixel_cut.split(" ");
        }
        let tileSize = 512;
        // Verify the validity of the tile width
        if (metadata.hips_tile_width) {
            let hipsTileWidth = parseInt(metadata.hips_tile_width);
            let isPowerOfTwo = hipsTileWidth && !(hipsTileWidth & (hipsTileWidth - 1));

            if (isPowerOfTwo === true) {
                tileSize = hipsTileWidth;
            }
        }
        let url = metadata.hips_service_url;
        if (!url) {
            throw 'no valid service URL for retrieving the tiles'
        }

        if (url.startsWith('http://alasky')) {
            // From alasky one can directly use the https access
            url = url.replace('http', 'https');
        } else {
            // Pass by a proxy for extern http urls
            url = 'https://alasky.u-strasbg.fr/cgi/JSONProxy?url=' + url;
        }
        return {
            properties: {
                url: url,
                maxOrder:  parseInt(metadata.hips_order),
                frame: {
                    label: "J2000",
                    system: "J2000"
                },
                tileSize: tileSize,
                format: tileFormat,
                minCutout: parseFloat(cuts[0]),
                maxCutout: parseFloat(cuts[1]),
            },
            color: color
        };
    }

    HpxImageSurvey.create = async function(idOrRootUrl, options) {
        if (!idOrRootUrl) {
            return;
        }
    
        let survey = await new HpxImageSurvey(idOrRootUrl);
        return survey;
    };

    return HpxImageSurvey;
})();



/***/ }),

/***/ "./src/js/ImageSurveyLayer.js":
/*!************************************!*\
  !*** ./src/js/ImageSurveyLayer.js ***!
  \************************************/
/*! exports provided: ImageSurveyLayer */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ImageSurveyLayer", function() { return ImageSurveyLayer; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./HpxImageSurvey.js */ "./src/js/HpxImageSurvey.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File ImageSurveyLayer
 * 
 * Authors: Thomas Boch & Matthieu Baumann [CDS]
 * 
 *****************************************************************************/



let ImageSurveyLayer = (function() {
    /** Constructor
     * cooFrame and maxOrder can be set to null
     * They will be determined by reading the properties file
     *  
     */
    /** Constructor
     *  
     */
    let ImageSurveyLayer = function(name) {
        this.surveys = new Map();
        this.name = name;
    }

    ImageSurveyLayer.prototype.addImageSurvey = async function(rootUrlOrId) {
        const survey = await _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_1__["HpxImageSurvey"].create(rootUrlOrId);
        this.surveys.set(rootUrlOrId, survey);
    };

    ImageSurveyLayer.prototype.removeImageSurvey = function(rootUrlOrId) {
        this.surveys.remove(rootUrlOrId);
    };

    ImageSurveyLayer.prototype.clear = function() {
        this.surveys.clear();
    };

    ImageSurveyLayer.prototype.getSurveys = function() {
        return this.surveys.values();
    };

    return ImageSurveyLayer;
})();

/***/ }),

/***/ "./src/js/Line.js":
/*!************************!*\
  !*** ./src/js/Line.js ***!
  \************************/
/*! exports provided: Line */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Line", function() { return Line; });
// Copyright 2015 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * Class Line
 * 
 * A line is a graphical overlay connecting 2 points
 * 
 * Author: Matthieu Baumann[CDS]
 * 
 *****************************************************************************/

let Line = (function() {
    // constructor
    let Line = function(x1, y1, x2, y2) {
        this.x1 = x1;
        this.y1 = y1;
        this.x2 = x2;
        this.y2 = y2;
    };

    // Method for testing whether a line is inside the view
    // http://www.jeffreythompson.org/collision-detection/line-rect.php
    Line.prototype.isInsideView = function(rw, rh) {
        if (this.x1 >= 0 && this.x1 <= rw && this.y1 >= 0 && this.y1 <= rh) {
            return true;
        }
        if (this.x2 >= 0 && this.x2 <= rw && this.y2 >= 0 && this.y2 <= rh) {
            return true;
        }

        // check if the line has hit any of the rectangle's sides
        // uses the Line/Line function below
        let left =   Line.intersectLine(this.x1, this.y1, this.x2, this.y2, 0, 0, 0, rh);
        let right =  Line.intersectLine(this.x1, this.y1, this.x2, this.y2, rw, 0, rw, rh);
        let top =    Line.intersectLine(this.x1, this.y1, this.x2, this.y2, 0, 0, rw, 0);
        let bottom = Line.intersectLine(this.x1, this.y1, this.x2, this.y2, 0, rh, rw, rh);
    
        // if ANY of the above are true, the line
        // has hit the rectangle
        if (left || right || top || bottom) {
            return true;
        }

        return false;
    };

    Line.prototype.draw = function(ctx) {
        ctx.moveTo(this.x1, this.y1);
        ctx.lineTo(this.x2, this.y2);
    };

    Line.intersectLine = function(x1, y1, x2, y2, x3, y3, x4, y4) {
        // Calculate the direction of the lines
        let uA = ((x4-x3)*(y1-y3) - (y4-y3)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
        let uB = ((x2-x1)*(y1-y3) - (y2-y1)*(x1-x3)) / ((y4-y3)*(x2-x1) - (x4-x3)*(y2-y1));
    
        // If uA and uB are between 0-1, lines are colliding
        if (uA >= 0 && uA <= 1 && uB >= 0 && uB <= 1) {
            return true;
        }
        return false;
    };

    return Line;
})();

/***/ }),

/***/ "./src/js/Location.js":
/*!****************************!*\
  !*** ./src/js/Location.js ***!
  \****************************/
/*! exports provided: Location */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Location", function() { return Location; });
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Location.js
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/





let Location = (function () {
    // constructor
    function Location(locationDiv) {
        this.$div = $(locationDiv);
    };

    Location.prototype.update = function (lon, lat, cooFrame, isViewCenterPosition) {
        isViewCenterPosition = (isViewCenterPosition && isViewCenterPosition === true) || false;

        var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_0__["Coo"](lon, lat, 7);
        if (cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_1__["CooFrameEnum"].J2000) {
            this.$div.html(coo.format('s/'));
        }
        else if (cooFrame == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_1__["CooFrameEnum"].J2000d) {
            this.$div.html(coo.format('d/'));
        }
        else {
            this.$div.html(coo.format('d/'));
        }

        this.$div.toggleClass('aladin-reticleColor', isViewCenterPosition);
    };

    return Location;
})();



/***/ }),

/***/ "./src/js/Logger.js":
/*!**************************!*\
  !*** ./src/js/Logger.js ***!
  \**************************/
/*! exports provided: Logger */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Logger", function() { return Logger; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



// log 
let Logger = {};

Logger.log = function(action, params) {
    try {
        var logUrl = "//alasky.unistra.fr/cgi/AladinLiteLogger/log.py";
        var paramStr = "";
        if (params) {
            paramStr = JSON.stringify(params);
        }
        
        $.ajax({
            url: logUrl,
            data: {"action": action, "params": paramStr, "pageUrl": window.location.href, "referer": document.referrer ? document.referrer : ""},
            method: 'GET',
            dataType: 'json' // as alasky supports CORS, we do not need JSONP any longer
        });
        
    }
    catch(e) {
        window.console && console.log('Exception: ' + e);
    }

};


/***/ }),

/***/ "./src/js/MOC.js":
/*!***********************!*\
  !*** ./src/js/MOC.js ***!
  \***********************/
/*! exports provided: MOC */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "MOC", function() { return MOC; });
/* harmony import */ var _libs_healpix_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/healpix.js */ "./src/js/libs/healpix.js");
/* harmony import */ var _libs_fits_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./libs/fits.js */ "./src/js/libs/fits.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _HealpixCache_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./HealpixCache.js */ "./src/js/HealpixCache.js");
/* harmony import */ var _Aladin_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./Aladin.js */ "./src/js/Aladin.js");
/* harmony import */ var _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./ProjectionEnum.js */ "./src/js/ProjectionEnum.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _CooConversion_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./CooConversion.js */ "./src/js/CooConversion.js");
/******************************************************************************
 * Aladin Lite project
 * 
 * File MOC
 *
 * This class represents a MOC (Multi Order Coverage map) layer
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/











let MOC = (function() {
    let MOC = function(options) {
        this.order = undefined;

        this.type = 'moc';

        // TODO homogenize options parsing for all kind of overlay (footprints, catalog, MOC)
        options = options || {};
        this.name = options.name || "MOC";
        this.color = options.color || Color.getNextColor();
        this.opacity = options.opacity || 1;
        this.opacity = Math.max(0, Math.min(1, this.opacity)); // 0 <= this.opacity <= 1
        this.lineWidth = options["lineWidth"] || 1;
        this.adaptativeDisplay = options['adaptativeDisplay'] !== false;

        this.proxyCalled = false; // this is a flag to check whether we already tried to load the MOC through the proxy

        // index of MOC cells at high and low resolution
        this._highResIndexOrder3 = new Array(768);
        this._lowResIndexOrder3 = new Array(768);
        for (var k=0; k<768; k++) {
            this._highResIndexOrder3[k] = {};
            this._lowResIndexOrder3[k] = {};
        }

        this.nbCellsDeepestLevel = 0; // needed to compute the sky fraction of the MOC

        this.isShowing = true;
        this.ready = false;
    }

    
    function log2(val) {
        return Math.log(val) / Math.LN2;
    }

    // max norder we can currently handle (limitation of healpix.js)
    MOC.MAX_NORDER = 13; // NSIDE = 8192

    MOC.LOWRES_MAXORDER = 6; // 5 or 6 ??
    MOC.HIGHRES_MAXORDER = 11; // ??

    // TODO: options to modifiy this ?
    MOC.PIVOT_FOV = 30; // when do we switch from low res cells to high res cells (fov in degrees)

    // at end of parsing, we need to remove duplicates from the 2 indexes
    MOC.prototype._removeDuplicatesFromIndexes = function() {
        var a, aDedup;
        for (var k=0; k<768; k++) {
            for (var key in this._highResIndexOrder3[k]) {
                a = this._highResIndexOrder3[k][key];
                aDedup = uniq(a);
                this._highResIndexOrder3[k][key] = aDedup;
            }
            for (var key in this._lowResIndexOrder3[k]) {
                a = this._lowResIndexOrder3[k][key];
                aDedup = uniq(a);
                this._lowResIndexOrder3[k][key] = aDedup;
            }
        }
        
    }

    // add pixel (order, ipix)
    MOC.prototype._addPix = function(order, ipix) {
        var ipixOrder3 = Math.floor( ipix * Math.pow(4, (3 - order)) );
        // fill low and high level cells
        // 1. if order <= LOWRES_MAXORDER, just store value in low and high res cells
        if (order<=MOC.LOWRES_MAXORDER) {
            if (! (order in this._lowResIndexOrder3[ipixOrder3])) {
                this._lowResIndexOrder3[ipixOrder3][order] = [];
                this._highResIndexOrder3[ipixOrder3][order] = [];
            }
            this._lowResIndexOrder3[ipixOrder3][order].push(ipix);
            this._highResIndexOrder3[ipixOrder3][order].push(ipix);
        }
        // 2. if LOWRES_MAXORDER < order <= HIGHRES_MAXORDER , degrade ipix for low res cells
        else if (order<=MOC.HIGHRES_MAXORDER) {
            if (! (order in this._highResIndexOrder3[ipixOrder3])) {
                this._highResIndexOrder3[ipixOrder3][order] = [];
            }
            this._highResIndexOrder3[ipixOrder3][order].push(ipix);
            
            var degradedOrder = MOC.LOWRES_MAXORDER; 
            var degradedIpix  = Math.floor(ipix / Math.pow(4, (order - degradedOrder)));
            var degradedIpixOrder3 = Math.floor( degradedIpix * Math.pow(4, (3 - degradedOrder)) );
            if (! (degradedOrder in this._lowResIndexOrder3[degradedIpixOrder3])) {
                this._lowResIndexOrder3[degradedIpixOrder3][degradedOrder]= [];
            }
            this._lowResIndexOrder3[degradedIpixOrder3][degradedOrder].push(degradedIpix);
        }
        // 3. if order > HIGHRES_MAXORDER , degrade ipix for low res and high res cells
        else {
            // low res cells
            var degradedOrder = MOC.LOWRES_MAXORDER; 
            var degradedIpix  = Math.floor(ipix / Math.pow(4, (order - degradedOrder)));
            var degradedIpixOrder3 = Math.floor(degradedIpix * Math.pow(4, (3 - degradedOrder)) );
            if (! (degradedOrder in this._lowResIndexOrder3[degradedIpixOrder3])) {
                this._lowResIndexOrder3[degradedIpixOrder3][degradedOrder]= [];
            }
            this._lowResIndexOrder3[degradedIpixOrder3][degradedOrder].push(degradedIpix);

            
            // high res cells
            degradedOrder = MOC.HIGHRES_MAXORDER; 
            degradedIpix  = Math.floor(ipix / Math.pow(4, (order - degradedOrder)));
            var degradedIpixOrder3 = Math.floor(degradedIpix * Math.pow(4, (3 - degradedOrder)) );
            if (! (degradedOrder in this._highResIndexOrder3[degradedIpixOrder3])) {
                this._highResIndexOrder3[degradedIpixOrder3][degradedOrder]= [];
            }
            this._highResIndexOrder3[degradedIpixOrder3][degradedOrder].push(degradedIpix);
        }

        this.nbCellsDeepestLevel += Math.pow(4, (this.order - order));
    };


    /**
     *  Return a value between 0 and 1 denoting the fraction of the sky
     *  covered by the MOC
     */
    MOC.prototype.skyFraction = function() {
        return this.nbCellsDeepestLevel / (12 * Math.pow(4, this.order));
    };

    /**
     * set MOC data by parsing a MOC serialized in JSON
     * (as defined in IVOA MOC document, section 3.1.1)
     */
    MOC.prototype.dataFromJSON = function(jsonMOC) {
        var order, ipix;
        for (var orderStr in jsonMOC) {
            if (jsonMOC.hasOwnProperty(orderStr)) {
                order = parseInt(orderStr);
                if (this.order===undefined || order > this.order) {
                    this.order = order;
                }
                for (var k=0; k<jsonMOC[orderStr].length; k++) {
                    ipix = jsonMOC[orderStr][k];
                    this._addPix(order, ipix);
                }
            }
        }

        this.reportChange();
        this.ready = true;
    };

    /**
     * set MOC data by parsing a URL pointing to a FITS MOC file
     */
    MOC.prototype.dataFromFITSURL = function(mocURL, successCallback) {
        var self = this;
        var callback = function() {
            // note: in the callback, 'this' refers to the FITS instance

            // first, let's find MOC norder
            var hdr0;
            try {
                // A zero-length hdus array might mean the served URL does not have CORS header
                // --> let's try again through the proxy
                if (this.hdus.length == 0) {
                    if (self.proxyCalled !== true) {
                        self.proxyCalled = true;
                        var proxiedURL = _Aladin_js__WEBPACK_IMPORTED_MODULE_4__["Aladin"].JSONP_PROXY + '?url=' + encodeURIComponent(self.dataURL);
                        new _libs_fits_js__WEBPACK_IMPORTED_MODULE_1__["astro"].FITS(proxiedURL, callback);
                    }

                    return;
                }
                hdr0 = this.getHeader(0);
            }
            catch (e) {
                console.error('Could not get header of extension #0');
                return;
            }
            var hdr1 = this.getHeader(1);

            if (hdr0.contains('HPXMOC')) {
                self.order = hdr0.get('HPXMOC')
            }
            else if (hdr0.contains('MOCORDER')) {
                self.order = hdr0.get('MOCORDER')
            }
            else if (hdr1.contains('HPXMOC')) {
                self.order = hdr1.get('HPXMOC')
            }
            else if (hdr1.contains('MOCORDER')) {
                self.order = hdr1.get('MOCORDER')
            }
            else {
                console.error('Can not find MOC order in FITS file');
                return;
            }


            var data = this.getDataUnit(1);
            var colName = data.columns[0];
            data.getRows(0, data.rows, function(rows) {
                for (var k=0; k<rows.length; k++) {
                    var uniq = rows[k][colName];
                    var order = Math.floor(Math.floor(log2(Math.floor(uniq/4))) / 2);
                    var ipix = uniq - 4 *(Math.pow(4, order));



                    self._addPix(order, ipix);
                }

            });
            data = null; // this helps releasing memory

            self._removeDuplicatesFromIndexes();

            if (successCallback) {
                successCallback();
            }

            self.reportChange();
            self.ready = true;
        }; // end of callback function

        this.dataURL = mocURL;

        // instantiate the FITS object which will fetch the URL passed as parameter
        new _libs_fits_js__WEBPACK_IMPORTED_MODULE_1__["astro"].FITS(this.dataURL, callback);
    };

    MOC.prototype.setView = function(view) {
        this.view = view;
        this.reportChange();
    };
    
    MOC.prototype.draw = function(ctx, projection, viewFrame, width, height, largestDim, zoomFactor, fov) {
        if (! this.isShowing || ! this.ready) {
            return;
        }
        var mocCells = fov > MOC.PIVOT_FOV && this.adaptativeDisplay ? this._lowResIndexOrder3 : this._highResIndexOrder3;

        this._drawCells(ctx, mocCells, fov, projection, viewFrame, _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__["CooFrameEnum"].J2000, width, height, largestDim, zoomFactor);
    };

    MOC.prototype._drawCells = function(ctx, mocCellsIdxOrder3, fov, projection, viewFrame, surveyFrame, width, height, largestDim, zoomFactor) {
        ctx.lineWidth = this.lineWidth;
        // if opacity==1, we draw solid lines, else we fill each HEALPix cell
        if (this.opacity==1) {
            ctx.strokeStyle = this.color;
        }
        else {
            ctx.fillStyle = this.color;
            ctx.globalAlpha = this.opacity;
        }


        ctx.beginPath();

        var orderedKeys = [];
        for (var k=0; k<768; k++) {
            var mocCells = mocCellsIdxOrder3[k];
            for (var key in mocCells) {
                orderedKeys.push(parseInt(key));
            }
        }
        orderedKeys.sort(function(a, b) {return a - b;});
        var norderMax = orderedKeys[orderedKeys.length-1];

        var nside, xyCorners, ipix;
        var potentialVisibleHpxCellsOrder3 = this.view.getVisiblePixList(3, _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__["CooFrameEnum"].J2000);
        var visibleHpxCellsOrder3 = [];
        // let's test first all potential visible cells and keep only the one with a projection inside the view
        for (var k=0; k<potentialVisibleHpxCellsOrder3.length; k++) {
            var ipix = potentialVisibleHpxCellsOrder3[k];
            xyCorners = getXYCorners(8, ipix, viewFrame, surveyFrame, width, height, largestDim, zoomFactor, projection, this.view);
            if (xyCorners) {
                visibleHpxCellsOrder3.push(ipix);
            }
        }

        var counter = 0;
        var mocCells;
        var norder3Ipix;
        for (var norder=0; norder<=norderMax; norder++) {
            nside = 1 << norder;

            for (var i=0; i<visibleHpxCellsOrder3.length; i++) {
                var ipixOrder3 = visibleHpxCellsOrder3[i];
                mocCells = mocCellsIdxOrder3[ipixOrder3];
                if (typeof mocCells[norder]==='undefined') {
                    continue;
                }
            
                if (norder<=3) {
                    for (var j=0; j<mocCells[norder].length; j++) {
                        ipix = mocCells[norder][j];
                        var factor = Math.pow(4, (3-norder));
                        var startIpix = ipix * factor;
                        for (var k=0; k<factor; k++) {
                            norder3Ipix = startIpix + k;
                            xyCorners = getXYCorners(8, norder3Ipix, viewFrame, surveyFrame, width, height, largestDim, zoomFactor, projection, this.view);
                            if (xyCorners) {
                                drawCorners(ctx, xyCorners);
                            }
                        }
                    }
                }
                else {
                    for (var j=0; j<mocCells[norder].length; j++) {
                        ipix = mocCells[norder][j];
                        var parentIpixOrder3 = Math.floor(ipix/Math.pow(4, norder-3));
                        xyCorners = getXYCorners(nside, ipix, viewFrame, surveyFrame, width, height, largestDim, zoomFactor, projection, this.view);
                        if (xyCorners) {
                            drawCorners(ctx, xyCorners);
                        }
                    }
                }
            }
        }


        if (this.opacity==1) {
            ctx.stroke();
        }
        else {
            ctx.fill();
            ctx.globalAlpha = 1.0;
        }
    };

    var drawCorners = function(ctx, xyCorners) {
        //console.log(xyCorners);
        ctx.moveTo(xyCorners[0].vx, xyCorners[0].vy);
        ctx.lineTo(xyCorners[1].vx, xyCorners[1].vy);
        ctx.lineTo(xyCorners[2].vx, xyCorners[2].vy);
        ctx.lineTo(xyCorners[3].vx, xyCorners[3].vy);
        ctx.lineTo(xyCorners[0].vx, xyCorners[0].vy);
    }

    // remove duplicate items from array a
    var uniq = function(a) {
        var seen = {};
        var out = [];
        var len = a.length;
        var j = 0;
        for (var i = 0; i < len; i++) {
            var item = a[i];
            if (seen[item] !== 1) {
                seen[item] = 1;
                out[j++] = item;
            }
        }

        return out;
    };


    // TODO: merge with what is done in View.getVisibleCells
    //var _spVec = new SpatialVector();
    var getXYCorners = function(nside, ipix, viewFrame, surveyFrame, width, height, largestDim, zoomFactor, projection, view) {
        var cornersXYView = [];
        var cornersXY = [];

        //var spVec = _spVec;

        //var corners = HealpixCache.corners_nest(ipix, nside);
        var corners = _Aladin_js__WEBPACK_IMPORTED_MODULE_4__["Aladin"].wasmLibs.hpx.nestedVertices(Math.log2(nside), ipix);

        var ra, dec;
        var lon, lat;
        for (var k=0; k<4; k++) {
            //spVec.setXYZ(corners[k].x, corners[k].y, corners[k].z);

            ra = corners[2*k];
            dec = corners[2*k + 1];
            // need for frame transformation ?
            /*if (surveyFrame && surveyFrame.system != viewFrame.system) {
                if (surveyFrame.system == CooFrameEnum.SYSTEMS.J2000) {
                    var radec = CooConversion.J2000ToGalactic([ra, dec]);
                    lon = radec[0];
                    lat = radec[1];
                }
                else if (surveyFrame.system == CooFrameEnum.SYSTEMS.GAL) {
                    var radec = CooConversion.GalacticToJ2000([ra, dec]);
                    lon = radec[0];
                    lat = radec[1];
                }
            }
            else {
                lon = ra;
                lat = dec;
            }*/
            lon = ra;
            lat = dec;

            //cornersXY[k] = projection.project(lon, lat);
            cornersXYView[k] = view.aladin.webglAPI.worldToScreen(lon, lat);
            if (!cornersXYView[k]) {
                return null;
            } else {
                //console.log(lon, lat);
                cornersXYView[k] = {
                    vx: cornersXYView[k][0],
                    vy: cornersXYView[k][1],
                };
            }
            //console.log(cornersXYView[k]);
        }

        /*if (cornersXYView[0] == null ||  cornersXYView[1] == null  ||  cornersXYView[2] == null ||  cornersXYView[3] == null ) {
            return null;
        }*/
        /*if (cornersXY[0] == null ||  cornersXY[1] == null  ||  cornersXY[2] == null ||  cornersXY[3] == null ) {
            return null;
        }

        for (var k=0; k<4; k++) {
            cornersXYView[k] = AladinUtils.xyToView(cornersXY[k].X, cornersXY[k].Y, width, height, largestDim, zoomFactor);
        }*/
        
        // detect pixels outside view. Could be improved !
        // we minimize here the number of cells returned
        if( cornersXYView[0].vx<0 && cornersXYView[1].vx<0 && cornersXYView[2].vx<0 &&cornersXYView[3].vx<0) {
            return null;
        }
        if( cornersXYView[0].vy<0 && cornersXYView[1].vy<0 && cornersXYView[2].vy<0 &&cornersXYView[3].vy<0) {
            return null;
        }
        if( cornersXYView[0].vx>=width && cornersXYView[1].vx>=width && cornersXYView[2].vx>=width &&cornersXYView[3].vx>=width) {
            return null;
        }
        if( cornersXYView[0].vy>=height && cornersXYView[1].vy>=height && cornersXYView[2].vy>=height &&cornersXYView[3].vy>=height) {
            return null;
        }

        // check if we have a pixel at the edge of the view in allsky projections
        if (projection.PROJECTION!=_ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_5__["ProjectionEnum"].SIN && projection.PROJECTION!=_ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_5__["ProjectionEnum"].TAN) {
            // Faster approach: when a vertex from a cell gets to the other side of the projection
            // its vertices order change from counter-clockwise to clockwise!
            // So if the vertices describing a cell are given in clockwise order
            // we know it crosses the projection, so we do not plot them!
            if (!_AladinUtils_js__WEBPACK_IMPORTED_MODULE_6__["AladinUtils"].counterClockwiseTriangle(cornersXYView[0].vx, cornersXYView[0].vy, cornersXYView[1].vx, cornersXYView[1].vy, cornersXYView[2].vx, cornersXYView[2].vy) ||
                !_AladinUtils_js__WEBPACK_IMPORTED_MODULE_6__["AladinUtils"].counterClockwiseTriangle(cornersXYView[0].vx, cornersXYView[0].vy, cornersXYView[2].vx, cornersXYView[2].vy, cornersXYView[3].vx, cornersXYView[3].vy)) {
                return null;
            }
        }

        //cornersXYView = AladinUtils.grow2(cornersXYView, 1);
        return cornersXYView;
    };

    MOC.prototype.reportChange = function() {
        this.view && this.view.requestRedraw();
    };

    MOC.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        this.reportChange();
    };

    MOC.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        this.reportChange();
    };

    // Tests whether a given (ra, dec) point on the sky is within the current MOC object
    //
    // returns true if point is contained, false otherwise
    MOC.prototype.contains = function(ra, dec) {
        var hpxIdx = new HealpixIndex(Math.pow(2, this.order));
        hpxIdx.init();
        var polar = HealpixIndex.utils.radecToPolar(ra, dec);
        var ipix = hpxIdx.ang2pix_nest(polar.theta, polar.phi);
        var ipixMapByOrder = {};
        for (var curOrder=0; curOrder<=this.order; curOrder++) {
            ipixMapByOrder[curOrder] = Math.floor(ipix / Math.pow(4, this.order - curOrder));
        }

        // first look for large HEALPix cells (order<3)
        for (var ipixOrder3=0; ipixOrder3<768; ipixOrder3++) {
            var mocCells = this._highResIndexOrder3[ipixOrder3];
            for (var order in mocCells) {
                if (order<3) {
                    for (var k=mocCells[order].length; k>=0; k--) {
                        if (ipixMapByOrder[order] == mocCells[order][k]) {
                            return true;
                        }   
                    }
                }
            }
        }

        // look for finer cells
        var ipixOrder3 = ipixMapByOrder[3];
        var mocCells = this._highResIndexOrder3[ipixOrder3];
        for (var order in mocCells) {
            for (var k=mocCells[order].length; k>=0; k--) {
                if (ipixMapByOrder[order] == mocCells[order][k]) {
                    return true;
                }   
            }
        }

        return false;
    };



    return MOC;

})();

    


/***/ }),

/***/ "./src/js/MeasurementTable.js":
/*!************************************!*\
  !*** ./src/js/MeasurementTable.js ***!
  \************************************/
/*! exports provided: MeasurementTable */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "MeasurementTable", function() { return MeasurementTable; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File MeasurementTable
 *
 * Graphic object showing measurement of a catalog
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let MeasurementTable = (function() {


    // constructor
    function MeasurementTable(aladinLiteDiv) {
        this.isShowing = false;

        this.divEl = $('<div class="aladin-measurement-div"></div>');
        
        $(aladinLiteDiv).append(this.divEl);
    }

    // show measurement associated with a given source
    MeasurementTable.prototype.showMeasurement = function(source) {
        this.divEl.empty();
        var header = '<thead><tr>';
        var content = '<tr>';
        for (let key in source.data) {
            header += '<th>' + key + '</th>';
            content += '<td>' + source.data[key] + '</td>';
        }
        header += '</tr></thead>';
        content += '</tr>';
        this.divEl.append('<table>' + header + content + '</table>');
        this.show();
    };

    MeasurementTable.prototype.show = function() {
        this.divEl.show();
    };
    
    MeasurementTable.prototype.hide = function() {
        this.divEl.hide();
    };
    
    
    return MeasurementTable;
})();



/***/ }),

/***/ "./src/js/Overlay.js":
/*!***************************!*\
  !*** ./src/js/Overlay.js ***!
  \***************************/
/*! exports provided: Overlay */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Overlay", function() { return Overlay; });
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _Footprint_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Footprint.js */ "./src/js/Footprint.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _Line_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./Line.js */ "./src/js/Line.js");
// Copyright 2015 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Overlay
 *
 * Description: a plane holding overlays (footprints, polylines, circles)
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/






let Overlay = (function() {
   let Overlay = function(options) {
        options = options || {};

        this.type = 'overlay';

    	this.name = options.name || "overlay";
    	this.color = options.color || Color.getNextColor();
        
    	this.lineWidth = options["lineWidth"] || 2;
    	
    	//this.indexationNorder = 5; // at which level should we index overlays?
    	this.overlays = [];
    	this.overlay_items = []; // currently Circle or Polyline
    	//this.hpxIdx = new HealpixIndex(this.indexationNorder);
    	//this.hpxIdx.init();
    	
    	this.isShowing = true;
    };
    

    // TODO : show/hide methods should be integrated in a parent class 
    Overlay.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        this.reportChange();
    };
    
    Overlay.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        this.reportChange();
    };
    
    // return an array of Footprint from a STC-S string
    Overlay.parseSTCS = function(stcs) {
        var footprints = [];
        var parts = stcs.match(/\S+/g);
        var k = 0, len = parts.length;
        while(k<len) {
            var s = parts[k].toLowerCase();
            if(s=='polygon') {
                var curPolygon = [];
                k++;
                frame = parts[k].toLowerCase();
                if (frame=='icrs' || frame=='j2000' || frame=='fk5') {
                    while(k+2<len) {
                        var ra = parseFloat(parts[k+1]);
                        if (isNaN(ra)) {
                            break;
                        }
                        var dec = parseFloat(parts[k+2]);
                        curPolygon.push([ra, dec]);
                        k += 2;
                    }
                    curPolygon.push(curPolygon[0]);
                    footprints.push(new _Footprint_js__WEBPACK_IMPORTED_MODULE_1__["Footprint"](curPolygon));
                }
            }
            else if (s=='circle') {
                var frame;
                k++;
                frame = parts[k].toLowerCase();

                if (frame=='icrs' || frame=='j2000' || frame=='fk5') {
                    var ra, dec, radiusDegrees;

                    ra = parseFloat(parts[k+1]);
                    dec = parseFloat(parts[k+2]);
                    radiusDegrees = parseFloat(parts[k+3]);

                    footprints.push(A.circle(ra, dec, radiusDegrees)); 

                    k += 3;
                }
            }

            k++;
        }

        return footprints;
    };
    
    // ajout d'un tableau d'overlays (= objets Footprint, Circle ou Polyline)
    Overlay.prototype.addFootprints = function(overlaysToAdd) {
    	for (var k=0, len=overlaysToAdd.length; k<len; k++) {
            this.add(overlaysToAdd[k], false);
        }

        this.view.requestRedraw();
    };

    // TODO : item doit pouvoir prendre n'importe quoi en param (footprint, circle, polyline)
    Overlay.prototype.add = function(item, requestRedraw) {
        requestRedraw = requestRedraw !== undefined ? requestRedraw : true;

        if (item instanceof _Footprint_js__WEBPACK_IMPORTED_MODULE_1__["Footprint"]) {
            this.overlays.push(item);
        }
        else {
            this.overlay_items.push(item);
        }
        item.setOverlay(this);
        
        if (requestRedraw) {
            this.view.requestRedraw();
        }
    };

    
    // return a footprint by index
   Overlay.prototype.getFootprint = function(idx) {
        if (idx<this.footprints.length) {
            return this.footprints[idx];
        }
        else {
            return null;
        }
    };
    
    Overlay.prototype.setView = function(view) {
        this.view = view;
    };
    
    Overlay.prototype.removeAll = function() {
        // TODO : RAZ de l'index
        this.overlays = [];
        this.overlay_items = [];
    };
    
    Overlay.prototype.draw = function(ctx, projection, frame, width, height, largestDim, zoomFactor) {
        if (!this.isShowing) {
            return;
        }
        
        // simple drawing
        ctx.strokeStyle= this.color;

        // 1. Drawing polygons
        
        // TODO: les overlay polygons devrait se tracer lui meme (methode draw)
        ctx.lineWidth = this.lineWidth;
    	ctx.beginPath();
    	var xyviews = [];
    	for (var k=0, len = this.overlays.length; k<len; k++) {
    		xyviews.push(this.drawFootprint(this.overlays[k], ctx, projection, frame, width, height, largestDim, zoomFactor));
    	}
        ctx.stroke();

    	// selection drawing
        ctx.strokeStyle= Overlay.increaseBrightness(this.color, 50);
        ctx.beginPath();
        for (var k=0, len = this.overlays.length; k<len; k++) {
            if (! this.overlays[k].isSelected) {
                continue;
            }
            this.drawFootprintSelected(ctx, xyviews[k]);
            
        }
    	ctx.stroke();
    	
        // 2. Circle and polylines drawing
    	for (var k=0; k<this.overlay_items.length; k++) {
    	    this.overlay_items[k].draw(ctx, this.view, projection, frame, width, height, largestDim, zoomFactor);
    	}
    };

    Overlay.increaseBrightness = function(hex, percent){
        // strip the leading # if it's there
        hex = hex.replace(/^\s*#|\s*$/g, '');

        // convert 3 char codes --> 6, e.g. `E0F` --> `EE00FF`
        if(hex.length == 3){
            hex = hex.replace(/(.)/g, '$1$1');
        }

        var r = parseInt(hex.substr(0, 2), 16),
            g = parseInt(hex.substr(2, 2), 16),
            b = parseInt(hex.substr(4, 2), 16);

        return '#' +
                ((0|(1<<8) + r + (256 - r) * percent / 100).toString(16)).substr(1) +
                ((0|(1<<8) + g + (256 - g) * percent / 100).toString(16)).substr(1) +
                ((0|(1<<8) + b + (256 - b) * percent / 100).toString(16)).substr(1);
    };
    
    
    Overlay.prototype.drawFootprint = function(f, ctx, projection, frame, width, height, largestDim, zoomFactor) {
        if (! f.isShowing) {
            return null;
        }
        var xyviewArray = [];
        //var show = false;
        var radecArray = f.polygons;
        for(var l=0; l<radecArray.length-1; l++) {
            let pts = this.view.aladin.webglAPI.projectLine(radecArray[l][0], radecArray[l][1], radecArray[l+1][0], radecArray[l+1][1]);
            for(var k=0; k<pts.length; k+=4) {
                let line = new _Line_js__WEBPACK_IMPORTED_MODULE_3__["Line"](pts[k], pts[k+1], pts[k+2], pts[k+3]);
                if (line.isInsideView(width, height)) {
                    line.draw(ctx);
                }    
            }
        }

        // for
            /*for (var k=0, len=radecArray.length; k<len; k++) {
                var xy;
                if (frame.system != CooFrameEnum.SYSTEMS.J2000) {
                    var lonlat = CooConversion.J2000ToGalactic([radecArray[k][0], radecArray[k][1]]);
                    xy = projection.project(lonlat[0], lonlat[1]);
                }
                else {
                    xy = projection.project(radecArray[k][0], radecArray[k][1]);
                }
                if (!xy) {
                    return null;
                }
                var xyview = AladinUtils.xyToView(xy.X, xy.Y, width, height, largestDim, zoomFactor);
                xyviewArray.push(xyview);
                if (!show && xyview.vx<width  && xyview.vx>=0 && xyview.vy<=height && xyview.vy>=0) {
                    show = true;
                }
            }

            if (show) {
                ctx.moveTo(xyviewArray[0].vx, xyviewArray[0].vy);
                for (var k=1, len=xyviewArray.length; k<len; k++) {
                    ctx.lineTo(xyviewArray[k].vx, xyviewArray[k].vy);
                }
            }
            else {
                //return null;
            }
        // end for*/

        return xyviewArray;



    };

    Overlay.prototype.drawFootprintSelected = function(ctx, xyview) {
        if (!xyview) {
            return;
        }

        var xyviewArray = xyview;
        ctx.moveTo(xyviewArray[0].vx, xyviewArray[0].vy);
        for (var k=1, len=xyviewArray.length; k<len; k++) {
            ctx.lineTo(xyviewArray[k].vx, xyviewArray[k].vy);
        }
    };


    
    // callback function to be called when the status of one of the footprints has changed
    Overlay.prototype.reportChange = function() {
        this.view.requestRedraw();
    };

    return Overlay;
})();


/***/ }),

/***/ "./src/js/Polyline.js":
/*!****************************!*\
  !*** ./src/js/Polyline.js ***!
  \****************************/
/*! exports provided: Polyline */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Polyline", function() { return Polyline; });
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _Line_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Line.js */ "./src/js/Line.js");
// Copyright 2015 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * Class Polyline
 * 
 * A Polyline is a graphical overlay made of several connected points
 * 
 * TODO: Polyline and Circle should derive from a common base class
 * TODO: index polyline, Circle in HEALPix pixels to avoid unneeded calls to draw 
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/




let Polyline= (function() {
    // constructor
    let Polyline = function(radecArray, options) {
        options = options || {};
        this.color = options['color'] || undefined;
        
        this.radecArray = radecArray;
        this.overlay = null;
    	
    	this.isShowing = true;
    	this.isSelected = false;
    };
    
    Polyline.prototype.setOverlay = function(overlay) {
        this.overlay = overlay;
    };
    
    Polyline.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Polyline.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Polyline.prototype.select = function() {
        if (this.isSelected) {
            return;
        }
        this.isSelected = true;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Polyline.prototype.deselect = function() {
        if (! this.isSelected) {
            return;
        }
        this.isSelected = false;
        if (this.overlay) {
            this.overlay.reportChange();
        }
    };
    
    Polyline.prototype.draw = function(ctx, view, projection, frame, width, height, largestDim, zoomFactor) {
        if (! this.isShowing) {
            return;
        }

        if (! this.radecArray || this.radecArray.length<2) {
            return;
        }
        
        if (this.color) {
            ctx.strokeStyle= this.color;
        }
        /*var start = AladinUtils.radecToViewXy(this.radecArray[0][0], this.radecArray[0][1], projection, frame, width, height, largestDim, zoomFactor);
        if (! start) {
            return;
        }
       
        ctx.beginPath();
        ctx.moveTo(start.vx, start.vy);
        var pt;
        for (var k=1; k<this.radecArray.length; k++) {
            pt = AladinUtils.radecToViewXy(this.radecArray[k][0], this.radecArray[k][1], projection, frame, width, height, largestDim, zoomFactor);
            if (!pt) {
                break;
            }
            ctx.lineTo(pt.vx, pt.vy);
        }
        
        
        ctx.stroke();*/
        ctx.beginPath();
        for(var l=0; l<this.radecArray.length-1; l++) {
            let pts = view.aladin.webglAPI.projectLine(this.radecArray[l][0], this.radecArray[l][1], this.radecArray[l+1][0], this.radecArray[l+1][1]);
            for(var k=0; k<pts.length; k+=4) {
                let line = new _Line_js__WEBPACK_IMPORTED_MODULE_1__["Line"](pts[k], pts[k+1], pts[k+2], pts[k+3]);
                if (line.isInsideView(width, height)) {
                    line.draw(ctx);
                }    
            }
        }

        ctx.stroke();
    };

    return Polyline;
})();

/***/ }),

/***/ "./src/js/Popup.js":
/*!*************************!*\
  !*** ./src/js/Popup.js ***!
  \*************************/
/*! exports provided: Popup */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Popup", function() { return Popup; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Popup.js
 * 
 * Author: Thomas Boch [CDS]
 * 
 *****************************************************************************/

let Popup = (function() {
    
    
    // constructor
    function Popup(parentDiv, view) {
        this.domEl = $('<div class="aladin-popup-container"><div class="aladin-popup"><a class="aladin-closeBtn">&times;</a><div class="aladin-popupTitle"></div><div class="aladin-popupText"></div></div><div class="aladin-popup-arrow"></div></div>');
        this.domEl.appendTo(parentDiv);

        this.view = view;


        var self = this;
        // close popup
        this.domEl.find('.aladin-closeBtn').click(function() {self.hide();});
        
    };
    
    Popup.prototype.hide = function() {
        this.domEl.hide();

        this.view.mustClearCatalog=true;
        this.view.catalogForPopup.hide();
    };

    Popup.prototype.show = function() {
        this.domEl.show();
    };

    Popup.prototype.setTitle = function(title) {
        this.domEl.find('.aladin-popupTitle').html(title || '');
    };

    Popup.prototype.setText = function(text) {
        this.domEl.find('.aladin-popupText').html(text || '');
        this.w = this.domEl.outerWidth();
        this.h = this.domEl.outerHeight();
    };

    Popup.prototype.setSource = function(source) {
        // remove reference to popup for previous source
        if (this.source) {
            this.source.popup = null;
        }
        source.popup = this;
        this.source = source;
        this.setPosition(source.x, source.y);
    };

    Popup.prototype.setPosition = function(x, y) {
        var newX = x - this.w/2;
        var newY = y - this.h;
        if (this.source) {
            newY += this.source.catalog.sourceSize/2;
        }

        this.domEl[0].style.left = newX + 'px';
        this.domEl[0].style.top  = newY + 'px';
    };
    
    return Popup;
})();



/***/ }),

/***/ "./src/js/ProgressiveCat.js":
/*!**********************************!*\
  !*** ./src/js/ProgressiveCat.js ***!
  \**********************************/
/*! exports provided: ProgressiveCat */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ProgressiveCat", function() { return ProgressiveCat; });
/* harmony import */ var _Catalog_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Catalog.js */ "./src/js/Catalog.js");
/* harmony import */ var _Source_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Source.js */ "./src/js/Source.js");
/* harmony import */ var _Color_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./Color.js */ "./src/js/Color.js");
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File ProgressiveCat.js
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

 
 
 
 
 
 

// TODO: index sources according to their HEALPix ipix
// TODO : merge parsing with class Catalog
let ProgressiveCat = (function() {
    
    // TODO : test if CORS support. If no, need to pass through a proxy
    // currently, we suppose CORS is supported
    
    // constructor
    let ProgressiveCat = function(rootUrl, frameStr, maxOrder, options) {
        options = options || {};

        this.type = 'progressivecat';
        
        this.rootUrl = rootUrl; // TODO: method to sanitize rootURL (absolute, no duplicate slashes, remove end slash if existing)
        // fast fix for HTTPS support --> will work for all HiPS served by CDS
        if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isHttpsContext() && ( /u-strasbg.fr/i.test(this.rootUrl) || /unistra.fr/i.test(this.rootUrl)  ) ) {
            this.rootUrl = this.rootUrl.replace('http://', 'https://');
        }

        this.frameStr = frameStr;
        this.frame = _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_5__["CooFrameEnum"].fromString(frameStr) || _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_5__["CooFrameEnum"].J2000;
        this.maxOrder = maxOrder;
        this.isShowing = true; // TODO : inherit from catalogue

        this.name = options.name || "progressive-cat";
        this.color = options.color || _Color_js__WEBPACK_IMPORTED_MODULE_2__["Color"].getNextColor();
        this.shape = options.shape || "square";
        this.sourceSize = options.sourceSize || 6;
        this.selectSize = this.sourceSize + 2;
        this.selectionColor = '#00ff00'; // TODO: to be merged with Catalog

        // allows for filtering of sources
        this.filterFn = options.filter || undefined; // TODO: do the same for catalog


        this.onClick = options.onClick || undefined; // TODO: inherit from catalog

        

        // we cache the list of sources in each healpix tile. Key of the cache is norder+'-'+npix
        this.sourcesCache = new _Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].LRUCache(100);

        this.updateShape(options);




        this.maxOrderAllsky = 2;
        this.isReady = false;
    };

    // TODO: to be put higher in the class diagram, in a HiPS generic class
    ProgressiveCat.readProperties = function(rootUrl, successCallback, errorCallback) {
        if (! successCallback) {
            return;
        }

        var propertiesURL = rootUrl + '/properties';
        $.ajax({
            url: propertiesURL,
            method: 'GET',
            dataType: 'text',
            success: function(propertiesTxt) {
                var props = {};
                var lines = propertiesTxt.split('\n');
                for (var k=0; k<lines.length; k++) {
                    var line = lines[k];
                    var idx = line.indexOf('=');
                    var propName  = $.trim(line.substring(0, idx));
                    var propValue = $.trim(line.substring(idx + 1));
                    
                    props[propName] = propValue;
                }
    
                successCallback(props);
                
            },
            error: function(err) { // TODO : which parameters should we put in the error callback
                errorCallback && errorCallback(err);
            }
        });




        
    };

    function getFields(instance, xml) {
        var attributes = ["name", "ID", "ucd", "utype", "unit", "datatype", "arraysize", "width", "precision"];

        var fields = [];
        var k = 0;
        instance.keyRa = instance.keyDec = null;
        $(xml).find("FIELD").each(function() {
            var f = {};
            for (var i=0; i<attributes.length; i++) {
                var attribute = attributes[i];
                if ($(this).attr(attribute)) {
                    f[attribute] = $(this).attr(attribute);
                }
                
            }
            if ( ! f.ID) {
                f.ID = "col_" + k;
            }
            
            if (!instance.keyRa && f.ucd && (f.ucd.indexOf('pos.eq.ra')==0 || f.ucd.indexOf('POS_EQ_RA')==0)) {
                if (f.name) {
                    instance.keyRa = f.name;
                }
                else {
                    instance.keyRa = f.ID;
                }
            }
            if (!instance.keyDec && f.ucd && (f.ucd.indexOf('pos.eq.dec')==0 || f.ucd.indexOf('POS_EQ_DEC')==0)) {
                if (f.name) {
                    instance.keyDec = f.name;
                }
                else {
                    instance.keyDec = f.ID;
                }
            }
            
            fields.push(f);
            k++;
        });

        return fields;
    }

    function getSources(instance, csv, fields) {
        // TODO : find ra and dec key names (see in Catalog)
        if (!instance.keyRa || ! instance.keyDec) {
            return [];
        }
        var lines = csv.split('\n');
        var mesureKeys = [];
        for (var k=0; k<fields.length; k++) {
            if (fields[k].name) {
                mesureKeys.push(fields[k].name);
            }
            else {
                mesureKeys.push(fields[k].ID);
            }
        }
        

        var sources = [];
        var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_3__["Coo"]();
        var newSource;
        // start at i=1, as first line repeat the fields names
        for (var i=2; i<lines.length; i++) {
            var mesures = {};
            var data = lines[i].split('\t');
            if (data.length<mesureKeys.length) {
                continue;
            }
            for (var j=0; j<mesureKeys.length; j++) {
                mesures[mesureKeys[j]] = data[j];
            }
            var ra, dec;
            if (_Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(mesures[instance.keyRa]) && _Utils_js__WEBPACK_IMPORTED_MODULE_4__["Utils"].isNumber(mesures[instance.keyDec])) {
                ra = parseFloat(mesures[instance.keyRa]);
                dec = parseFloat(mesures[instance.keyDec]);
            }
            else {
                coo.parse(mesures[instance.keyRa] + " " + mesures[instance.keyDec]);
                ra = coo.lon;
                dec = coo.lat;
            }
            newSource = new _Source_js__WEBPACK_IMPORTED_MODULE_1__["Source"](ra, dec, mesures);
            sources.push(newSource);
            newSource.setCatalog(instance);
        }
        return sources;
    };

    //ProgressiveCat.prototype.updateShape = Catalog.prototype.updateShape;

    ProgressiveCat.prototype = {

        init: function(view) {
            var self = this;
            this.view = view;

            if (this.maxOrder && this.frameStr) {
                this._loadMetadata();
            }

            else {
                ProgressiveCat.readProperties(self.rootUrl,
                    function (properties) {
                        self.properties = properties;
                        self.maxOrder = self.properties['hips_order'];
                        self.frame = _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_5__["CooFrameEnum"].fromString(self.properties['hips_frame']);

                        self._loadMetadata();
                    }, function(err) {
                        console.log('Could not find properties for HiPS ' + self.rootUrl);
                    }
                );
            }
        },

        updateShape: _Catalog_js__WEBPACK_IMPORTED_MODULE_0__["Catalog"].prototype.updateShape,

        _loadMetadata: function() {
            var self = this;
            $.ajax({
                url: self.rootUrl + '/' + 'Metadata.xml',
                method: 'GET',
                success: function(xml) {
                    self.fields = getFields(self, xml);
                    self._loadAllskyNewMethod();
                },
                error: function(err) {
                    self._loadAllskyOldMethod();
                }
            });
        },

        _loadAllskyNewMethod: function() {
            var self = this;
            $.ajax({
                url: self.rootUrl + '/' + 'Norder1/Allsky.tsv',
                method: 'GET',
                success: function(tsv) {
                    self.order1Sources = getSources(self, tsv, self.fields);

                    if (self.order2Sources) {
                        self.isReady = true;
                        self._finishInitWhenReady();
                    }
                },
                error: function(err) {
                    console.log('Something went wrong: ' + err);
                }
            });

            $.ajax({
                url: self.rootUrl + '/' + 'Norder2/Allsky.tsv',
                method: 'GET',
                success: function(tsv) {
                    self.order2Sources = getSources(self, tsv, self.fields);

                    if (self.order1Sources) {
                        self.isReady = true;
                        self._finishInitWhenReady();
                    }
                },
                error: function(err) {
                    console.log('Something went wrong: ' + err);
                }
            });

        },

        _loadAllskyOldMethod: function() {
            this.maxOrderAllsky = 3;
            this._loadLevel2Sources();
            this._loadLevel3Sources();
        },

        _loadLevel2Sources: function() {
            var self = this;
            $.ajax({
                url: self.rootUrl + '/' + 'Norder2/Allsky.xml',
                method: 'GET',
                success: function(xml) {
                    self.fields = getFields(self, xml);
                    self.order2Sources = getSources(self, $(xml).find('CSV').text(), self.fields);
                    if (self.order3Sources) {
                        self.isReady = true;
                        self._finishInitWhenReady();
                    }
                },
                error: function(err) {
                    console.log('Something went wrong: ' + err);
                }
            });
        },

        _loadLevel3Sources: function() {
            var self = this;
            $.ajax({
                url: self.rootUrl + '/' + 'Norder3/Allsky.xml',
                method: 'GET',
                success: function(xml) {
                    self.order3Sources = getSources(self, $(xml).find('CSV').text(), self.fields);
                    if (self.order2Sources) {
                        self.isReady = true;
                        self._finishInitWhenReady();
                    }
                },
                error: function(err) {
                    console.log('Something went wrong: ' + err);
                }
            });
        },

        _finishInitWhenReady: function() {
            this.view.requestRedraw();
            this.loadNeededTiles();
        },

        draw: function(ctx, projection, frame, width, height, largestDim, zoomFactor) {
            if (! this.isShowing || ! this.isReady) {
                return;
            }
            this.drawSources(this.order1Sources, ctx, projection, frame, width, height, largestDim, zoomFactor);
            this.drawSources(this.order2Sources, ctx, projection, frame, width, height, largestDim, zoomFactor);
            this.drawSources(this.order3Sources, ctx, projection, frame, width, height, largestDim, zoomFactor);
            
            if (!this.tilesInView) {
                return;
            }
            var sources, key, t;
            for (var k=0; k<this.tilesInView.length; k++) {
                t = this.tilesInView[k];
                key = t[0] + '-' + t[1];
                sources = this.sourcesCache.get(key);
                if (sources) {
                    this.drawSources(sources, ctx, projection, frame, width, height, largestDim, zoomFactor);
                }
            }
            
            
            
        },
        drawSources: function(sources, ctx, projection, frame, width, height, largestDim, zoomFactor) {
            if (! sources) {
                return;
            }
            var s;
            for (var k=0, len = sources.length; k<len; k++) {
                s = sources[k];
                if (!this.filterFn || this.filterFn(s)) {
                    _Catalog_js__WEBPACK_IMPORTED_MODULE_0__["Catalog"].drawSource(this, s, ctx, projection, frame, width, height, largestDim, zoomFactor);
                }
            }
            for (var k=0, len = sources.length; k<len; k++) {
                s = sources[k];
                if (! s.isSelected) {
                    continue;
                }
                if (!this.filterFn || this.filterFn(s)) {
                    _Catalog_js__WEBPACK_IMPORTED_MODULE_0__["Catalog"].drawSourceSelection(this, s, ctx);
                }
            }
        },

        getSources: function() {
            var ret = [];
            if (this.order1Sources) {
                ret = ret.concat(this.order1Sources);
            }
            if (this.order2Sources) {
                ret = ret.concat(this.order2Sources);
            }
            if (this.order3Sources) {
                ret = ret.concat(this.order3Sources);
            }
            if (this.tilesInView) {
                var sources, key, t;
                for (var k=0; k<this.tilesInView.length; k++) {
                    t = this.tilesInView[k];
                    key = t[0] + '-' + t[1];
                    sources = this.sourcesCache.get(key);
                    if (sources) {
                        ret = ret.concat(sources);
                    }
                }
            }
            
            return ret;
        },


        
        deselectAll: function() {
            if (this.order1Sources) {
                for (var k=0; k<this.order1Sources.length; k++) {
                    this.order1Sources[k].deselect();
                }
            }

            if (this.order2Sources) {
                for (var k=0; k<this.order2Sources.length; k++) {
                    this.order2Sources[k].deselect();
                }
            }

            if (this.order3Sources) {
                for (var k=0; k<this.order3Sources.length; k++) {
                    this.order3Sources[k].deselect();
                }
            }
            var keys = this.sourcesCache.keys();
            for (key in keys) {
                if ( ! this.sourcesCache[key]) {
                    continue;
                }
                var sources = this.sourcesCache[key];
                for (var k=0; k<sources.length; k++) {
                    sources[k].deselect();
                }
            }
        },

        show: function() {
            if (this.isShowing) {
                return;
            }
            this.isShowing = true;
            this.loadNeededTiles();
            this.reportChange();
        },
        hide: function() {
            if (! this.isShowing) {
                return;
            }
            this.isShowing = false;
            this.reportChange();
        },
        reportChange: function() {
            this.view.requestRedraw();
        },
        
        getTileURL: function(norder, npix) {
            var dirIdx = Math.floor(npix/10000)*10000;
            return this.rootUrl + "/" + "Norder" + norder + "/Dir" + dirIdx + "/Npix" + npix + ".tsv";
        },
    
        loadNeededTiles: function() {
            if ( ! this.isShowing) {
                return;
            }
            this.tilesInView = [];
            
            var norder = this.view.realNorder;
            if (norder>this.maxOrder) {
                norder = this.maxOrder;
            }
            if (norder<=this.maxOrderAllsky) {
                return; // nothing to do, hurrayh !
            }
            var cells = this.view.getVisibleCells(norder, this.frame);
            var ipixList, ipix;
            for (var curOrder=3; curOrder<=norder; curOrder++) {
                ipixList = [];
                for (var k=0; k<cells.length; k++) {
                    ipix = Math.floor(cells[k].ipix / Math.pow(4, norder - curOrder));
                    if (ipixList.indexOf(ipix)<0) {
                        ipixList.push(ipix);
                    }
                }
                
                // load needed tiles
                for (var i=0; i<ipixList.length; i++) {
                    this.tilesInView.push([curOrder, ipixList[i]]);
                }
            }
            
            var t, key;
            var self = this;
            for (var k=0; k<this.tilesInView.length; k++) {
                t = this.tilesInView[k];
                key = t[0] + '-' + t[1]; // t[0] is norder, t[1] is ipix
                if (!this.sourcesCache.get(key)) {
                    (function(self, norder, ipix) { // wrapping function is needed to be able to retrieve norder and ipix in ajax success function
                        var key = norder + '-' + ipix;
                        $.ajax({
                            /*
                            url: Aladin.JSONP_PROXY,
                            data: {"url": self.getTileURL(norder, ipix)},
                            */
                            // ATTENTIOn : je passe en JSON direct, car je n'arrive pas a choper les 404 en JSONP
                            url: self.getTileURL(norder, ipix),
                            method: 'GET',
                            //dataType: 'jsonp',
                            success: function(tsv) {
                                self.sourcesCache.set(key, getSources(self, tsv, self.fields));
                                //self.view.requestRedraw();
                            },
                            error: function() {
                                // on suppose qu'il s'agit d'une erreur 404
                                self.sourcesCache.set(key, []);
                            }
                        });
                    })(this, t[0], t[1]);
                }
            }
        },

        reportChange: function() { // TODO: to be shared with Catalog
            this.view && this.view.requestRedraw();
        }
    

    }; // END OF .prototype functions
    
    
    return ProgressiveCat;
})();
    


/***/ }),

/***/ "./src/js/ProjectionEnum.js":
/*!**********************************!*\
  !*** ./src/js/ProjectionEnum.js ***!
  \**********************************/
/*! exports provided: ProjectionEnum */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ProjectionEnum", function() { return ProjectionEnum; });
/* harmony import */ var _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/astro/projection.js */ "./src/js/libs/astro/projection.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File CooFrameEnum
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

 

 let ProjectionEnum = {
    SIN: _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_SIN,
    AITOFF:  _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_AITOFF,
    MERCATOR:  _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_MERCATOR,
    ARC:  _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_ARC,
    TAN:  _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_TAN,
    MOL: _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_0__["Projection"].PROJ_MOLLWEIDE,
 };


/***/ }),

/***/ "./src/js/Sesame.js":
/*!**************************!*\
  !*** ./src/js/Sesame.js ***!
  \**************************/
/*! exports provided: Sesame */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Sesame", function() { return Sesame; });
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Sesame.js
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/



let Sesame = (function() {
    let Sesame = {};
    
    Sesame.cache = {};

    Sesame.SESAME_URL = "http://cds.u-strasbg.fr/cgi-bin/nph-sesame.jsonp";

    /** find RA, DEC for any target (object name or position)
     *  if successful, callback is called with an object {ra: <ra-value>, dec: <dec-value>}
     *  if not successful, errorCallback is called
     */
    Sesame.getTargetRADec = function(target, callback, errorCallback) {
        if (!callback) {
            return;
        }
        var isObjectName = /[a-zA-Z]/.test(target);

        // try to parse as a position
        if ( ! isObjectName) {
            var coo = new Coo();

            coo.parse(target);
            if (callback) {
                callback({ra: coo.lon, dec: coo.lat});
            }
        }
        // ask resolution by Sesame
        else {
            Sesame.resolve(target,
                   function(data) { // success callback
                       callback({ra:  data.Target.Resolver.jradeg,
                                 dec: data.Target.Resolver.jdedeg});
                   },

                   function(data) { // error callback
                       if (errorCallback) {
                           errorCallback();
                       }
                   }
           );
        }
    };
    
    Sesame.resolve = function(objectName, callbackFunctionSuccess, callbackFunctionError) {
        var sesameUrl = Sesame.SESAME_URL;
        if (_Utils_js__WEBPACK_IMPORTED_MODULE_0__["Utils"].isHttpsContext()) {
            sesameUrl = sesameUrl.replace('http://', 'https://')
        }
            

        $.ajax({
            url: sesameUrl ,
            data: {"object": objectName},
            method: 'GET',
            dataType: 'jsonp',
            success: function(data) {
                if (data.Target && data.Target.Resolver && data.Target.Resolver) {
                    callbackFunctionSuccess(data);
                }
                else {
                    callbackFunctionError(data);
                }
            },
            error: callbackFunctionError
            });
    };
    
    return Sesame;
})();



/***/ }),

/***/ "./src/js/Shaders.js":
/*!***************************!*\
  !*** ./src/js/Shaders.js ***!
  \***************************/
/*! exports provided: loadShaders */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "loadShaders", function() { return loadShaders; });
/* harmony import */ var _core_src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../core/src/shaders/catalogs/aitoff.vert */ "./src/core/src/shaders/catalogs/aitoff.vert");
/* harmony import */ var _core_src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0__);
/* harmony import */ var _core_src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../core/src/shaders/catalogs/mercator.vert */ "./src/core/src/shaders/catalogs/mercator.vert");
/* harmony import */ var _core_src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_1__);
/* harmony import */ var _core_src_shaders_catalogs_arc_vert__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../core/src/shaders/catalogs/arc.vert */ "./src/core/src/shaders/catalogs/arc.vert");
/* harmony import */ var _core_src_shaders_catalogs_arc_vert__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_arc_vert__WEBPACK_IMPORTED_MODULE_2__);
/* harmony import */ var _core_src_shaders_catalogs_tan_vert__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../core/src/shaders/catalogs/tan.vert */ "./src/core/src/shaders/catalogs/tan.vert");
/* harmony import */ var _core_src_shaders_catalogs_tan_vert__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_tan_vert__WEBPACK_IMPORTED_MODULE_3__);
/* harmony import */ var _core_src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ../core/src/shaders/catalogs/mollweide.vert */ "./src/core/src/shaders/catalogs/mollweide.vert");
/* harmony import */ var _core_src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4__);
/* harmony import */ var _core_src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ../core/src/shaders/catalogs/ortho.vert */ "./src/core/src/shaders/catalogs/ortho.vert");
/* harmony import */ var _core_src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_5___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_5__);
/* harmony import */ var _core_src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ../core/src/shaders/catalogs/ortho.frag */ "./src/core/src/shaders/catalogs/ortho.frag");
/* harmony import */ var _core_src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_6___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_6__);
/* harmony import */ var _core_src_shaders_catalogs_catalog_frag__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ../core/src/shaders/catalogs/catalog.frag */ "./src/core/src/shaders/catalogs/catalog.frag");
/* harmony import */ var _core_src_shaders_catalogs_catalog_frag__WEBPACK_IMPORTED_MODULE_7___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_catalogs_catalog_frag__WEBPACK_IMPORTED_MODULE_7__);
/* harmony import */ var _core_src_shaders_colormaps_colormap_vert__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ../core/src/shaders/colormaps/colormap.vert */ "./src/core/src/shaders/colormaps/colormap.vert");
/* harmony import */ var _core_src_shaders_colormaps_colormap_vert__WEBPACK_IMPORTED_MODULE_8___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_colormap_vert__WEBPACK_IMPORTED_MODULE_8__);
/* harmony import */ var _core_src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ../core/src/shaders/colormaps/blackwhite.frag */ "./src/core/src/shaders/colormaps/blackwhite.frag");
/* harmony import */ var _core_src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9__);
/* harmony import */ var _core_src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! ../core/src/shaders/colormaps/BluePastelRed.frag */ "./src/core/src/shaders/colormaps/BluePastelRed.frag");
/* harmony import */ var _core_src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_10___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_10__);
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_11__ = __webpack_require__(/*! ../core/src/shaders/colormaps/IDL_CB_BrBG.frag */ "./src/core/src/shaders/colormaps/IDL_CB_BrBG.frag");
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_11___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_11__);
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_12__ = __webpack_require__(/*! ../core/src/shaders/colormaps/IDL_CB_GnBu.frag */ "./src/core/src/shaders/colormaps/IDL_CB_GnBu.frag");
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_12___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_12__);
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_13__ = __webpack_require__(/*! ../core/src/shaders/colormaps/IDL_CB_YIGnBu.frag */ "./src/core/src/shaders/colormaps/IDL_CB_YIGnBu.frag");
/* harmony import */ var _core_src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_13___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_13__);
/* harmony import */ var _core_src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_14__ = __webpack_require__(/*! ../core/src/shaders/colormaps/red.frag */ "./src/core/src/shaders/colormaps/red.frag");
/* harmony import */ var _core_src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_14___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_14__);
/* harmony import */ var _core_src_shaders_grid_grid_vert__WEBPACK_IMPORTED_MODULE_15__ = __webpack_require__(/*! ../core/src/shaders/grid/grid.vert */ "./src/core/src/shaders/grid/grid.vert");
/* harmony import */ var _core_src_shaders_grid_grid_vert__WEBPACK_IMPORTED_MODULE_15___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_grid_vert__WEBPACK_IMPORTED_MODULE_15__);
/* harmony import */ var _core_src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_16__ = __webpack_require__(/*! ../core/src/shaders/grid/aitoff.frag */ "./src/core/src/shaders/grid/aitoff.frag");
/* harmony import */ var _core_src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_16___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_16__);
/* harmony import */ var _core_src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_17__ = __webpack_require__(/*! ../core/src/shaders/grid/mollweide.frag */ "./src/core/src/shaders/grid/mollweide.frag");
/* harmony import */ var _core_src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_17___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_17__);
/* harmony import */ var _core_src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_18__ = __webpack_require__(/*! ../core/src/shaders/grid/ortho.frag */ "./src/core/src/shaders/grid/ortho.frag");
/* harmony import */ var _core_src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_18___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_18__);
/* harmony import */ var _core_src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_19__ = __webpack_require__(/*! ../core/src/shaders/grid/mercator.frag */ "./src/core/src/shaders/grid/mercator.frag");
/* harmony import */ var _core_src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_19___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_19__);
/* harmony import */ var _core_src_shaders_grid_arc_frag__WEBPACK_IMPORTED_MODULE_20__ = __webpack_require__(/*! ../core/src/shaders/grid/arc.frag */ "./src/core/src/shaders/grid/arc.frag");
/* harmony import */ var _core_src_shaders_grid_arc_frag__WEBPACK_IMPORTED_MODULE_20___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_arc_frag__WEBPACK_IMPORTED_MODULE_20__);
/* harmony import */ var _core_src_shaders_grid_tan_frag__WEBPACK_IMPORTED_MODULE_21__ = __webpack_require__(/*! ../core/src/shaders/grid/tan.frag */ "./src/core/src/shaders/grid/tan.frag");
/* harmony import */ var _core_src_shaders_grid_tan_frag__WEBPACK_IMPORTED_MODULE_21___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_tan_frag__WEBPACK_IMPORTED_MODULE_21__);
/* harmony import */ var _core_src_shaders_grid_grid_cpu_vert__WEBPACK_IMPORTED_MODULE_22__ = __webpack_require__(/*! ../core/src/shaders/grid/grid_cpu.vert */ "./src/core/src/shaders/grid/grid_cpu.vert");
/* harmony import */ var _core_src_shaders_grid_grid_cpu_vert__WEBPACK_IMPORTED_MODULE_22___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_grid_cpu_vert__WEBPACK_IMPORTED_MODULE_22__);
/* harmony import */ var _core_src_shaders_grid_grid_cpu_frag__WEBPACK_IMPORTED_MODULE_23__ = __webpack_require__(/*! ../core/src/shaders/grid/grid_cpu.frag */ "./src/core/src/shaders/grid/grid_cpu.frag");
/* harmony import */ var _core_src_shaders_grid_grid_cpu_frag__WEBPACK_IMPORTED_MODULE_23___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_grid_grid_cpu_frag__WEBPACK_IMPORTED_MODULE_23__);
/* harmony import */ var _core_src_shaders_hips_raytracer_raytracer_vert__WEBPACK_IMPORTED_MODULE_24__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/raytracer.vert */ "./src/core/src/shaders/hips/raytracer/raytracer.vert");
/* harmony import */ var _core_src_shaders_hips_raytracer_raytracer_vert__WEBPACK_IMPORTED_MODULE_24___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_raytracer_vert__WEBPACK_IMPORTED_MODULE_24__);
/* harmony import */ var _core_src_shaders_hips_raytracer_color_frag__WEBPACK_IMPORTED_MODULE_25__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/color.frag */ "./src/core/src/shaders/hips/raytracer/color.frag");
/* harmony import */ var _core_src_shaders_hips_raytracer_color_frag__WEBPACK_IMPORTED_MODULE_25___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_color_frag__WEBPACK_IMPORTED_MODULE_25__);
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_26__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/grayscale_to_color.frag */ "./src/core/src/shaders/hips/raytracer/grayscale_to_color.frag");
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_26___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_26__);
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_27__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/grayscale_to_colormap.frag */ "./src/core/src/shaders/hips/raytracer/grayscale_to_colormap.frag");
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_27___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_27__);
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_28__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/grayscale_to_color_i.frag */ "./src/core/src/shaders/hips/raytracer/grayscale_to_color_i.frag");
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_28___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_28__);
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_29__ = __webpack_require__(/*! ../core/src/shaders/hips/raytracer/grayscale_to_colormap_i.frag */ "./src/core/src/shaders/hips/raytracer/grayscale_to_colormap_i.frag");
/* harmony import */ var _core_src_shaders_hips_raytracer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_29___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_raytracer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_29__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_30__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/ortho.vert */ "./src/core/src/shaders/hips/rasterizer/ortho.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_30___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_30__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_31__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/mercator.vert */ "./src/core/src/shaders/hips/rasterizer/mercator.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_31___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_31__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_32__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/aitoff.vert */ "./src/core/src/shaders/hips/rasterizer/aitoff.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_32___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_32__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_gnomonic_vert__WEBPACK_IMPORTED_MODULE_33__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/gnomonic.vert */ "./src/core/src/shaders/hips/rasterizer/gnomonic.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_gnomonic_vert__WEBPACK_IMPORTED_MODULE_33___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_gnomonic_vert__WEBPACK_IMPORTED_MODULE_33__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_arc_vert__WEBPACK_IMPORTED_MODULE_34__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/arc.vert */ "./src/core/src/shaders/hips/rasterizer/arc.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_arc_vert__WEBPACK_IMPORTED_MODULE_34___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_arc_vert__WEBPACK_IMPORTED_MODULE_34__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_35__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/mollweide.vert */ "./src/core/src/shaders/hips/rasterizer/mollweide.vert");
/* harmony import */ var _core_src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_35___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_35__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_color_frag__WEBPACK_IMPORTED_MODULE_36__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/color.frag */ "./src/core/src/shaders/hips/rasterizer/color.frag");
/* harmony import */ var _core_src_shaders_hips_rasterizer_color_frag__WEBPACK_IMPORTED_MODULE_36___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_color_frag__WEBPACK_IMPORTED_MODULE_36__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_37__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/grayscale_to_color.frag */ "./src/core/src/shaders/hips/rasterizer/grayscale_to_color.frag");
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_37___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_37__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_38__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/grayscale_to_colormap.frag */ "./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap.frag");
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_38___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_38__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_39__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/grayscale_to_color_i.frag */ "./src/core/src/shaders/hips/rasterizer/grayscale_to_color_i.frag");
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_39___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_39__);
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_40__ = __webpack_require__(/*! ../core/src/shaders/hips/rasterizer/grayscale_to_colormap_i.frag */ "./src/core/src/shaders/hips/rasterizer/grayscale_to_colormap_i.frag");
/* harmony import */ var _core_src_shaders_hips_rasterizer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_40___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_hips_rasterizer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_40__);
/* harmony import */ var _core_src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_41__ = __webpack_require__(/*! ../core/src/shaders/misc/text.vert */ "./src/core/src/shaders/misc/text.vert");
/* harmony import */ var _core_src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_41___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_41__);
/* harmony import */ var _core_src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_42__ = __webpack_require__(/*! ../core/src/shaders/misc/text.frag */ "./src/core/src/shaders/misc/text.frag");
/* harmony import */ var _core_src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_42___default = /*#__PURE__*/__webpack_require__.n(_core_src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_42__);
/* Import all the shaders here*/ 
// Catalog shaders









// Colormap shaders








// Grid shader










// HiPS shaders
// Raytracer







// Rasterizer











// Misc



let shaders = [
    // Catalog shaders
    {
        id: "CatalogAitoffVS",
        content: _core_src_shaders_catalogs_aitoff_vert__WEBPACK_IMPORTED_MODULE_0___default.a,
    },
    {
        id: "CatalogMercatVS",
        content: _core_src_shaders_catalogs_mercator_vert__WEBPACK_IMPORTED_MODULE_1___default.a,
    },
    {
        id: "CatalogArcVS",
        content: _core_src_shaders_catalogs_arc_vert__WEBPACK_IMPORTED_MODULE_2___default.a,
    },
    {
        id: "CatalogTanVS",
        content: _core_src_shaders_catalogs_tan_vert__WEBPACK_IMPORTED_MODULE_3___default.a,
    },
    {
        id: "CatalogMollVS",
        content: _core_src_shaders_catalogs_mollweide_vert__WEBPACK_IMPORTED_MODULE_4___default.a,
    },
    {
        id: "CatalogOrthoVS",
        content: _core_src_shaders_catalogs_ortho_vert__WEBPACK_IMPORTED_MODULE_5___default.a,
    },
    {
        id: "CatalogOrthoFS",
        content: _core_src_shaders_catalogs_ortho_frag__WEBPACK_IMPORTED_MODULE_6___default.a,
    },
    {
        id: "CatalogFS",
        content: _core_src_shaders_catalogs_catalog_frag__WEBPACK_IMPORTED_MODULE_7___default.a,    
    },
    // Colormap shaders
    {
        id: "ColormapVS",
        content: _core_src_shaders_colormaps_colormap_vert__WEBPACK_IMPORTED_MODULE_8___default.a,
    },
    {
        id: "ColormapBlackWhiteFS",
        content: _core_src_shaders_colormaps_blackwhite_frag__WEBPACK_IMPORTED_MODULE_9___default.a
    },
    {
        id: "ColormapBluePastelRedFS",
        content: _core_src_shaders_colormaps_BluePastelRed_frag__WEBPACK_IMPORTED_MODULE_10___default.a
    },
    {
        id: "ColormapIDL_CB_BrBGFS",
        content: _core_src_shaders_colormaps_IDL_CB_BrBG_frag__WEBPACK_IMPORTED_MODULE_11___default.a
    },
    {
        id: "ColormapIDL_CB_GnBuFS",
        content: _core_src_shaders_colormaps_IDL_CB_GnBu_frag__WEBPACK_IMPORTED_MODULE_12___default.a
    },
    {
        id: "ColormapIDL_CB_YIGnBuFS",
        content: _core_src_shaders_colormaps_IDL_CB_YIGnBu_frag__WEBPACK_IMPORTED_MODULE_13___default.a
    },
    {
        id: "ColormapRedTemperatureFS",
        content: _core_src_shaders_colormaps_red_frag__WEBPACK_IMPORTED_MODULE_14___default.a
    },
    // Grid shader
    {
        id: "GridVS",
        content: _core_src_shaders_grid_grid_vert__WEBPACK_IMPORTED_MODULE_15___default.a,
    },
    {
        id: "GridAitoffFS",
        content: _core_src_shaders_grid_aitoff_frag__WEBPACK_IMPORTED_MODULE_16___default.a,
    },
    {
        id: "GridMollFS",
        content: _core_src_shaders_grid_mollweide_frag__WEBPACK_IMPORTED_MODULE_17___default.a,
    },
    {
        id: "GridOrthoFS",
        content: _core_src_shaders_grid_ortho_frag__WEBPACK_IMPORTED_MODULE_18___default.a,
    },
    {
        id: "GridMercatorFS",
        content: _core_src_shaders_grid_mercator_frag__WEBPACK_IMPORTED_MODULE_19___default.a,
    },
    {
        id: "GridArcFS",
        content: _core_src_shaders_grid_arc_frag__WEBPACK_IMPORTED_MODULE_20___default.a,
    },
    {
        id: "GridTanFS",
        content: _core_src_shaders_grid_tan_frag__WEBPACK_IMPORTED_MODULE_21___default.a,
    },
    {
        id: "GridFS_CPU",
        content: _core_src_shaders_grid_grid_cpu_frag__WEBPACK_IMPORTED_MODULE_23___default.a,
    },
    {
        id: "GridVS_CPU",
        content: _core_src_shaders_grid_grid_cpu_vert__WEBPACK_IMPORTED_MODULE_22___default.a,
    },
    // HiPS shaders
    // Raytracer
    {
        id: "RayTracerVS",
        content: _core_src_shaders_hips_raytracer_raytracer_vert__WEBPACK_IMPORTED_MODULE_24___default.a,
    },
    {
        id: "RayTracerColorFS",
        content: _core_src_shaders_hips_raytracer_color_frag__WEBPACK_IMPORTED_MODULE_25___default.a,
    },
    {
        id: "RayTracerGrayscale2ColorFS",
        content: _core_src_shaders_hips_raytracer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_26___default.a,
    },
    {
        id: "RayTracerGrayscale2ColormapFS",
        content: _core_src_shaders_hips_raytracer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_27___default.a,
    },
    {
        id: "RayTracerGrayscale2ColorIntegerFS",
        content: _core_src_shaders_hips_raytracer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_28___default.a,
    },
    {
        id: "RayTracerGrayscale2ColormapIntegerFS",
        content: _core_src_shaders_hips_raytracer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_29___default.a,
    },
    /// Rasterizer
    {
        id: "RasterizerOrthoVS",
        content: _core_src_shaders_hips_rasterizer_ortho_vert__WEBPACK_IMPORTED_MODULE_30___default.a,
    },
    {
        id: "RasterizerMercatorVS",
        content: _core_src_shaders_hips_rasterizer_mercator_vert__WEBPACK_IMPORTED_MODULE_31___default.a,
    },
    {
        id: "RasterizerAitoffVS",
        content: _core_src_shaders_hips_rasterizer_aitoff_vert__WEBPACK_IMPORTED_MODULE_32___default.a,
    },
    {
        id: "RasterizerArcVS",
        content: _core_src_shaders_hips_rasterizer_arc_vert__WEBPACK_IMPORTED_MODULE_34___default.a,
    },
    {
        id: "RasterizerGnomonicVS",
        content: _core_src_shaders_hips_rasterizer_gnomonic_vert__WEBPACK_IMPORTED_MODULE_33___default.a,
    },
    {
        id: "RasterizerMollVS",
        content: _core_src_shaders_hips_rasterizer_mollweide_vert__WEBPACK_IMPORTED_MODULE_35___default.a,
    },
    {
        id: "RasterizerColorFS",
        content: _core_src_shaders_hips_rasterizer_color_frag__WEBPACK_IMPORTED_MODULE_36___default.a,
    },
    {
        id: "RasterizerGrayscale2ColorFS",
        content: _core_src_shaders_hips_rasterizer_grayscale_to_color_frag__WEBPACK_IMPORTED_MODULE_37___default.a,
    },
    {
        id: "RasterizerGrayscale2ColormapFS",
        content: _core_src_shaders_hips_rasterizer_grayscale_to_colormap_frag__WEBPACK_IMPORTED_MODULE_38___default.a,
    },
    {
        id: "RasterizerGrayscale2ColorIntegerFS",
        content: _core_src_shaders_hips_rasterizer_grayscale_to_color_i_frag__WEBPACK_IMPORTED_MODULE_39___default.a,
    },
    {
        id: "RasterizerGrayscale2ColormapIntegerFS",
        content: _core_src_shaders_hips_rasterizer_grayscale_to_colormap_i_frag__WEBPACK_IMPORTED_MODULE_40___default.a,
    },
    // Misc
    {
        id: "TextVS",
        content: _core_src_shaders_misc_text_vert__WEBPACK_IMPORTED_MODULE_41___default.a,
    },
    {
        id: "TextFS",
        content: _core_src_shaders_misc_text_frag__WEBPACK_IMPORTED_MODULE_42___default.a,
    },
];

function loadShaders() {
    return shaders;
}


/***/ }),

/***/ "./src/js/SimbadPointer.js":
/*!*********************************!*\
  !*** ./src/js/SimbadPointer.js ***!
  \*********************************/
/*! no static exports found */
/***/ (function(module, exports) {

// Copyright 2018 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File SimbadPointer.js
 *
 * The SIMBAD pointer will query Simbad for a given position and radius and
 * return information on the object with 
 *  
 * Author: Thomas Boch [CDS]
 * 
 *****************************************************************************/

SimbadPointer = (function() {
    
    
    SimbadPointer = {};

    SimbadPointer.MIRRORS = ['https://alasky.u-strasbg.fr/cgi/simbad-flat/simbad-quick.py', 'https://alaskybis.u-strasbg.fr/cgi/simbad-flat/simbad-quick.py']; // list of base URL for Simbad pointer service

    
    SimbadPointer.query = function(ra, dec, radiusDegrees, aladinInstance) {
        var coo = new Coo(ra, dec, 7);
        var params = {Ident: coo.format('s/'), SR: radiusDegrees}
        var successCallback = function(result) {
            aladinInstance.view.setCursor('pointer');

            var regexp = /(.*?)\/(.*?)\((.*?),(.*?)\)/g;
            var match = regexp.exec(result);
            if (match) {
                var objCoo = new Coo();
                objCoo.parse(match[1]);
                var objName = match[2];
                var title = '<div class="aladin-sp-title"><a target="_blank" href="http://simbad.u-strasbg.fr/simbad/sim-id?Ident=' + encodeURIComponent(objName) + '">' + objName + '</a></div>';
                var content = '<div class="aladin-sp-content">';
                content += '<em>Type: </em>' + match[4] + '<br>';
                var magnitude = match[3];
                if (Utils.isNumber(magnitude)) {
                    content += '<em>Mag: </em>' + magnitude + '<br>';
                }
                content += '<br><a target="_blank" href="http://cdsportal.u-strasbg.fr/?target=' + encodeURIComponent(objName) + '">Query in CDS portal</a>';
                content += '</div>';
                aladinInstance.showPopup(objCoo.lon, objCoo.lat, title, content);
            }
            else {
                aladinInstance.hidePopup();
            }
        };
        var failureCallback = function() {
            aladinInstance.view.setCursor('pointer');
            aladinInstance.hidePopup();
        };
        Utils.loadFromMirrors(SimbadPointer.MIRRORS, {data: params, onSuccess: successCallback, onFailure: failureCallback, timeout: 5});

    };

    return SimbadPointer;
})();
    


/***/ }),

/***/ "./src/js/Source.js":
/*!**************************!*\
  !*** ./src/js/Source.js ***!
  \**************************/
/*! exports provided: Source */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Source", function() { return Source; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Source
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let Source = (function() {
    // constructor
    let Source = function(ra, dec, data, options) {
    	this.ra = ra;
    	this.dec = dec;
    	this.data = data;
    	this.catalog = null;
    	
        this.marker = (options && options.marker) || false;
        if (this.marker) {
            this.popupTitle = (options && options.popupTitle) ? options.popupTitle : '';
            this.popupDesc = (options && options.popupDesc) ? options.popupDesc : '';
            this.useMarkerDefaultIcon = (options && options.useMarkerDefaultIcon!==undefined) ? options.useMarkerDefaultIcon : true;
        }

    	this.isShowing = true;
    	this.isSelected = false;
    };
    
    Source.prototype.setCatalog = function(catalog) {
        this.catalog = catalog;
    };
    
    Source.prototype.show = function() {
        if (this.isShowing) {
            return;
        }
        this.isShowing = true;
        if (this.catalog) {
            this.catalog.reportChange();
        }
    };
    
    Source.prototype.hide = function() {
        if (! this.isShowing) {
            return;
        }
        this.isShowing = false;
        if (this.catalog) {
            this.catalog.reportChange();
        }
    };
    
    Source.prototype.select = function() {
        if (this.isSelected) {
            return;
        }
        this.isSelected = true;
        if (this.catalog) {
            this.catalog.reportChange();
        }
    };
    
    Source.prototype.deselect = function() {
        if (! this.isSelected) {
            return;
        }
        this.isSelected = false;
        if (this.catalog) {
            this.catalog.reportChange();
        }
    };

    // function called when a source is clicked. Called by the View object
    Source.prototype.actionClicked = function() {
        if (this.catalog && this.catalog.onClick) {
            var view = this.catalog.view;
            if (this.catalog.onClick=='showTable') {
                view.aladin.measurementTable.showMeasurement(this);
                this.select();
            }
            else if (this.catalog.onClick=='showPopup') {
                view.popup.setTitle('<br><br>');
                var m = '<div class="aladin-marker-measurement">';
                m += '<table>';
                for (var key in this.data) {
                    m += '<tr><td>' + key + '</td><td>' + this.data[key] + '</td></tr>';
                }
                m += '</table>';
                m += '</div>';
                view.popup.setText(m);
                view.popup.setSource(this);
                view.popup.show();
            }
            else if (typeof this.catalog.onClick === 'function') {
                this.catalog.onClick(this);
                view.lastClickedObject = this;
            }

        }
    };

    
    Source.prototype.actionOtherObjectClicked = function() {
        if (this.catalog && this.catalog.onClick) {
            this.deselect();
        }
    };
    
    return Source;
})();


/***/ }),

/***/ "./src/js/Tile.js":
/*!************************!*\
  !*** ./src/js/Tile.js ***!
  \************************/
/*! exports provided: Tile */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Tile", function() { return Tile; });
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File Tile
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

let Tile = (function() {
    // constructor
	function Tile(img, url) {
		this.img = img;
		this.url = url;
	};
	
	// check whether the image corresponding to the tile is loaded and ready to be displayed
	//
	// source : http://www.sajithmr.me/javascript-check-an-image-is-loaded-or-not
	Tile.isImageOk = function(img) {
		if (img.allSkyTexture) {
			return true;
		}
		
        if (!img.src) {
            return false;
        }

	    // During the onload event, IE correctly identifies any images that
	    // weren’t downloaded as not complete. Others should too. Gecko-based
	    // browsers act like NS4 in that they report this incorrectly.
	    if (!img.complete) {
	        return false;
	    }

	    // However, they do have two very useful properties: naturalWidth and
	    // naturalHeight. These give the true size of the image. If it failed
	    // to load, either of these should be zero.

	    if (typeof img.naturalWidth != "undefined" && img.naturalWidth == 0) {
	        return false;
	    }

        // TODO: Add a method checking if the tile has been copied to the GPU texture
        

	    // No other way of checking: assume it’s ok.
	    return true;
	};
	

	return Tile;
})();


/***/ }),

/***/ "./src/js/TileBuffer.js":
/*!******************************!*\
  !*** ./src/js/TileBuffer.js ***!
  \******************************/
/*! exports provided: TileBuffer */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "TileBuffer", function() { return TileBuffer; });
/* harmony import */ var _Tile_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Tile.js */ "./src/js/Tile.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File TileBuffer
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/

 

let TileBuffer = (function() {
	var NB_MAX_TILES = 800; // buffer size
	
	// constructor
	function TileBuffer() {
		this.pointer = 0;
		this.tilesMap = {};
		this.tilesArray = new Array(NB_MAX_TILES);

		for (var i=0; i<NB_MAX_TILES; i++) {
			this.tilesArray[i] = new _Tile_js__WEBPACK_IMPORTED_MODULE_0__["Tile"](new Image(), null);
		}
	};
	
	TileBuffer.prototype.addTile = function(url) {
	    // return null if already in buffer
        if (this.getTile(url)) {
            return null;
        }

        // delete existing tile
        var curTile = this.tilesArray[this.pointer];
        if (curTile.url != null) {
            curTile.img.src = null;
            delete this.tilesMap[curTile.url];
        }

        this.tilesArray[this.pointer].url = url;
        this.tilesMap[url] = this.tilesArray[this.pointer];

        this.pointer++;
        if (this.pointer>=NB_MAX_TILES) {
            this.pointer = 0;
        }

        return this.tilesMap[url];
	};
	
	TileBuffer.prototype.getTile = function(url) {
        return this.tilesMap[url];
	};
	
	return TileBuffer;
})();


/***/ }),

/***/ "./src/js/URLBuilder.js":
/*!******************************!*\
  !*** ./src/js/URLBuilder.js ***!
  \******************************/
/*! exports provided: URLBuilder */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "URLBuilder", function() { return URLBuilder; });
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//



/******************************************************************************
 * Aladin Lite project
 * 
 * File URLBuilder
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/


let URLBuilder = (function() {    

    let URLBuilder = {
        buildSimbadCSURL: function(target, radiusDegrees) {
            if (target && (typeof target  === "object")) {
                if ('ra' in target && 'dec' in target) {
                    var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_0__["Coo"](target.ra, target.dec, 7);
                    target = coo.format('s');
                }
            }
            return 'https://alasky.unistra.fr/cgi/simbad-flat/simbad-cs.py?target=' + encodeURIComponent(target) + '&SR=' + radiusDegrees + '&format=votable&SRUNIT=deg&SORTBY=nbref';
        },

        buildNEDPositionCSURL: function(ra, dec, radiusDegrees) {
                return 'https://ned.ipac.caltech.edu/cgi-bin/nph-objsearch?search_type=Near+Position+Search&of=xml_main&RA=' + ra + '&DEC=' + dec + '&SR=' + radiusDegrees;
        },

        buildNEDObjectCSURL: function(object, radiusDegrees) {
                return 'https://ned.ipac.caltech.edu/cgi-bin/nph-objsearch?search_type=Near+Name+Search&radius=' + (60 * radiusDegrees) + '&of=xml_main&objname=' + object;
        },

        buildVizieRCSURL: function(vizCatId, target, radiusDegrees, options) {
            console.log(target)
            if (target && (typeof target  === "object")) {
                if ('ra' in target && 'dec' in target) {
                    var coo = new _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_0__["Coo"](target.ra, target.dec, 7);

                    target = coo.format('s');
                }
            }

            var maxNbSources = 1e5;
            if (options && options.hasOwnProperty('limit') && _Utils_js__WEBPACK_IMPORTED_MODULE_1__["Utils"].isNumber(options.limit)) {
                maxNbSources = parseInt(options.limit);
            }
            return 'https://vizier.unistra.fr/viz-bin/votable?-source=' + vizCatId + '&-c=' + encodeURIComponent(target) + '&-out.max=' + maxNbSources + '&-c.rd=' + radiusDegrees;
        },

        buildSkyBotCSURL: function(ra, dec, radius, epoch, queryOptions) {
            var url = 'http://vo.imcce.fr/webservices/skybot/skybotconesearch_query.php?-from=AladinLite';
            url += '&RA=' + encodeURIComponent(ra);
            url += '&DEC=' + encodeURIComponent(dec);
            url += '&SR=' + encodeURIComponent(radius);
            url += '&EPOCH=' + encodeURIComponent(epoch);

            if (queryOptions) {
                for (var key in queryOptions) {
                    if (queryOptions.hasOwnProperty(key)) {
                            url += '&' + key + '=' + encodeURIComponent(queryOptions[key]);
                    }
                }
            }

            return url;
        }
    

    };

    return URLBuilder;
    
})();



/***/ }),

/***/ "./src/js/Utils.js":
/*!*************************!*\
  !*** ./src/js/Utils.js ***!
  \*************************/
/*! exports provided: Utils */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Utils", function() { return Utils; });
/* harmony import */ var _Aladin_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Aladin.js */ "./src/js/Aladin.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//




/******************************************************************************
 * Aladin Lite project
 * 
 * File Utils
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/



let Utils = {};

Utils.cssScale = undefined;
// adding relMouseCoords to HTMLCanvasElement prototype (see http://stackoverflow.com/questions/55677/how-do-i-get-the-coordinates-of-a-mouse-click-on-a-canvas-element ) 
function relMouseCoords(event) {
    var totalOffsetX = 0;
    var totalOffsetY = 0;
    var canvasX = 0;
    var canvasY = 0;
    var currentElement = this;
   
    if (event.offsetX) {
        return {x: event.offsetX, y:event.offsetY};
    } 
    else {
        if (!Utils.cssScale) {
            var st = window.getComputedStyle(document.body, null);
            var tr = st.getPropertyValue("-webkit-transform") ||
                    st.getPropertyValue("-moz-transform") ||
                    st.getPropertyValue("-ms-transform") ||
                    st.getPropertyValue("-o-transform") ||
                    st.getPropertyValue("transform");
            var matrixRegex = /matrix\((-?\d*\.?\d+),\s*0,\s*0,\s*(-?\d*\.?\d+),\s*0,\s*0\)/;
            var matches = tr.match(matrixRegex);
            if (matches) {
                Utils.cssScale = parseFloat(matches[1]);
            }
            else {
                Utils.cssScale = 1;
            }
        }
        var e = event;
        var canvas = e.target;
        // http://www.jacklmoore.com/notes/mouse-position/
        var target = e.target || e.srcElement;
        var style = target.currentStyle || window.getComputedStyle(target, null);
        var borderLeftWidth = parseInt(style['borderLeftWidth'], 10);
        var borderTopWidth = parseInt(style['borderTopWidth'], 10);
        var rect = target.getBoundingClientRect();

        var clientX = e.clientX;
        var clientY = e.clientY;
        if (e.clientX) {
            clientX = e.clientX;
            clientY = e.clientY;
        }
        else {
            clientX = e.originalEvent.changedTouches[0].clientX;
            clientY = e.originalEvent.changedTouches[0].clientY;
        }

        var offsetX = clientX - borderLeftWidth - rect.left;
        var offsetY = clientY - borderTopWidth - rect.top

        return {x: parseInt(offsetX/Utils.cssScale), y: parseInt(offsetY/Utils.cssScale)};
    }
}
HTMLCanvasElement.prototype.relMouseCoords = relMouseCoords;



//Function.prototype.bind polyfill from 
//https://developer.mozilla.org/en/JavaScript/Reference/Global_Objects/Function/bind
if (!Function.prototype.bind) {
    Function.prototype.bind = function (obj) {
        // closest thing possible to the ECMAScript 5 internal IsCallable function
        if (typeof this !== 'function') {
            throw new TypeError('Function.prototype.bind - what is trying to be bound is not callable');
        }

        var slice = [].slice,
        args = slice.call(arguments, 1),
        self = this,
        nop = function () { },
        bound = function () {
            return self.apply(this instanceof nop ? this : (obj || {}),
                    args.concat(slice.call(arguments)));
        };

        bound.prototype = this.prototype;

        return bound;
    };
}








//$ = $ || jQuery;

/* source : http://stackoverflow.com/a/8764051 */
$.urlParam = function(name, queryString){
    if (queryString===undefined) {
        queryString = location.search;
    }
	return decodeURIComponent((new RegExp('[?|&]' + name + '=' + '([^&;]+?)(&|#|;|$)').exec(queryString)||[,""])[1].replace(/\+/g, '%20'))||null;
};

/* source: http://stackoverflow.com/a/1830844 */
Utils.isNumber = function(n) {
  return !isNaN(parseFloat(n)) && isFinite(n);
};

Utils.isInt = function(n) {
    return Utils.isNumber(n) && Math.floor(n)==n;
};

/* a debounce function, used to prevent multiple calls to the same function if less than delay milliseconds have passed */
Utils.debounce = function(fn, delay) {
    var timer = null;
    return function () {
      var context = this, args = arguments;
      clearTimeout(timer);
      timer = setTimeout(function () {
        fn.apply(context, args);
      }, delay);
    };
};

/* return a throttled function, to rate limit the number of calls (by default, one call every 250 milliseconds) */
Utils.throttle = function(fn, threshhold, scope) {
  threshhold || (threshhold = 250);
  var last,
      deferTimer;
  return function () {
    var context = scope || this;

    var now = +new Date,
        args = arguments;
    if (last && now < last + threshhold) {
      // hold on to it
      clearTimeout(deferTimer);
      deferTimer = setTimeout(function () {
        last = now;
        fn.apply(context, args);
      }, threshhold);
    } else {
      last = now;
      fn.apply(context, args);
    }
  };
}


/* A LRU cache, inspired by https://gist.github.com/devinus/409353#file-gistfile1-js */
// TODO : utiliser le LRU cache pour les tuiles images
Utils.LRUCache = function (maxsize) {
    this._keys = [];
    this._items = {};
    this._expires = {};
    this._size = 0;
    this._maxsize = maxsize || 1024;
};
   
Utils.LRUCache.prototype = {
        set: function (key, value) {
            var keys = this._keys,
                items = this._items,
                expires = this._expires,
                size = this._size,
                maxsize = this._maxsize;

            if (size >= maxsize) { // remove oldest element when no more room
                keys.sort(function (a, b) {
                    if (expires[a] > expires[b]) return -1;
                    if (expires[a] < expires[b]) return 1;
                    return 0;
                });

                size--;
                delete expires[keys[size]];
                delete items[keys[size]];
            }

            keys[size] = key;
            items[key] = value;
            expires[key] = Date.now();
            size++;

            this._keys = keys;
            this._items = items;
            this._expires = expires;
            this._size = size;
        },

        get: function (key) {
            var item = this._items[key];
            if (item) this._expires[key] = Date.now();
            return item;
        },
        
        keys: function() {
            return this._keys;
        }
};

////////////////////////////////////////////////////////////////////////////:

/**
  Make an AJAX call, given a list of potential mirrors
  First successful call will result in options.onSuccess being called back
  If all calls fail, onFailure is called back at the end

  This method assumes the URL are CORS-compatible, no proxy will be used
 */
Utils.loadFromMirrors = function(urls, options) {
    var data    = options && options.data || null;
    var method = options && options.method || 'GET';
    var dataType = options && options.dataType || null;
    var timeout = options && options.timeout || 20;

    var onSuccess = options && options.onSuccess || null;
    var onFailure = options && options.onFailure || null;

    if (urls.length === 0) {
        (typeof onFailure === 'function') && onFailure();
    }
    else {
        var ajaxOptions = {
            url: urls[0],
            data: data
        }
        if (dataType) {
            ajaxOptions.dataType = dataType;
        }

        $.ajax(ajaxOptions)
        .done(function(data) {
            (typeof onSuccess === 'function') && onSuccess(data);
        })
        .fail(function() {
             Utils.loadFromMirrors(urls.slice(1), options);
        });
    }
} 

// return the jquery ajax object configured with the requested parameters
// by default, we use the proxy (safer, as we don't know if the remote server supports CORS)
Utils.getAjaxObject = function(url, method, dataType, useProxy) {
        if (useProxy!==false) {
            useProxy = true;
        }

        if (useProxy===true) {
            var urlToRequest = _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].JSONP_PROXY + '?url=' + encodeURIComponent(url);
        }
        else {
            urlToRequest = url;
        }
        method = method || 'GET';
        dataType = dataType || null;

        return $.ajax({
            url: urlToRequest,
            method: method,
            dataType: dataType
        }); 
};

// return true if script is executed in a HTTPS context
// return false otherwise
Utils.isHttpsContext = function() {
    return ( window.location.protocol === 'https:' );
};

// generate an absolute URL from a relative URL
// example: getAbsoluteURL('foo/bar/toto') return http://cds.unistra.fr/AL/foo/bar/toto if executed from page http://cds.unistra.fr/AL/
Utils.getAbsoluteURL = function(url) {
    var a = document.createElement('a');
    a.href = url;

    return a.href;
};

// generate a valid v4 UUID
Utils.uuidv4 = function() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}



/***/ }),

/***/ "./src/js/View.js":
/*!************************!*\
  !*** ./src/js/View.js ***!
  \************************/
/*! exports provided: View */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "View", function() { return View; });
/* harmony import */ var _Aladin_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./Aladin.js */ "./src/js/Aladin.js");
/* harmony import */ var _Popup_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./Popup.js */ "./src/js/Popup.js");
/* harmony import */ var _HealpixGrid_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./HealpixGrid.js */ "./src/js/HealpixGrid.js");
/* harmony import */ var _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./HpxImageSurvey.js */ "./src/js/HpxImageSurvey.js");
/* harmony import */ var _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./ProjectionEnum.js */ "./src/js/ProjectionEnum.js");
/* harmony import */ var _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./libs/astro/projection.js */ "./src/js/libs/astro/projection.js");
/* harmony import */ var _libs_astro_coo_js__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./libs/astro/coo.js */ "./src/js/libs/astro/coo.js");
/* harmony import */ var _AladinUtils_js__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./AladinUtils.js */ "./src/js/AladinUtils.js");
/* harmony import */ var _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ./libs/healpix.js */ "./src/js/libs/healpix.js");
/* harmony import */ var _HealpixCache_js__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ./HealpixCache.js */ "./src/js/HealpixCache.js");
/* harmony import */ var _Utils_js__WEBPACK_IMPORTED_MODULE_10__ = __webpack_require__(/*! ./Utils.js */ "./src/js/Utils.js");
/* harmony import */ var _SimbadPointer_js__WEBPACK_IMPORTED_MODULE_11__ = __webpack_require__(/*! ./SimbadPointer.js */ "./src/js/SimbadPointer.js");
/* harmony import */ var _SimbadPointer_js__WEBPACK_IMPORTED_MODULE_11___default = /*#__PURE__*/__webpack_require__.n(_SimbadPointer_js__WEBPACK_IMPORTED_MODULE_11__);
/* harmony import */ var _TileBuffer_js__WEBPACK_IMPORTED_MODULE_12__ = __webpack_require__(/*! ./TileBuffer.js */ "./src/js/TileBuffer.js");
/* harmony import */ var _Downloader_js__WEBPACK_IMPORTED_MODULE_13__ = __webpack_require__(/*! ./Downloader.js */ "./src/js/Downloader.js");
/* harmony import */ var _libs_Stats_js__WEBPACK_IMPORTED_MODULE_14__ = __webpack_require__(/*! ./libs/Stats.js */ "./src/js/libs/Stats.js");
/* harmony import */ var _ColorMap_js__WEBPACK_IMPORTED_MODULE_15__ = __webpack_require__(/*! ./ColorMap.js */ "./src/js/ColorMap.js");
/* harmony import */ var _Footprint_js__WEBPACK_IMPORTED_MODULE_16__ = __webpack_require__(/*! ./Footprint.js */ "./src/js/Footprint.js");
/* harmony import */ var _Circle_js__WEBPACK_IMPORTED_MODULE_17__ = __webpack_require__(/*! ./Circle.js */ "./src/js/Circle.js");
/* harmony import */ var _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__ = __webpack_require__(/*! ./CooFrameEnum.js */ "./src/js/CooFrameEnum.js");
/* harmony import */ var _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__ = __webpack_require__(/*! ./CooConversion.js */ "./src/js/CooConversion.js");
/* harmony import */ var _libs_RequestAnimationFrame_js__WEBPACK_IMPORTED_MODULE_20__ = __webpack_require__(/*! ./libs/RequestAnimationFrame.js */ "./src/js/libs/RequestAnimationFrame.js");
/* harmony import */ var _Shaders_js__WEBPACK_IMPORTED_MODULE_21__ = __webpack_require__(/*! ./Shaders.js */ "./src/js/Shaders.js");
/* harmony import */ var _core_img_kernel_png__WEBPACK_IMPORTED_MODULE_22__ = __webpack_require__(/*! ../core/img/kernel.png */ "./src/core/img/kernel.png");
/* harmony import */ var _ImageSurveyLayer_js__WEBPACK_IMPORTED_MODULE_23__ = __webpack_require__(/*! ./ImageSurveyLayer.js */ "./src/js/ImageSurveyLayer.js");
// Copyright 2013 - UDS/CNRS
// The Aladin Lite program is distributed under the terms
// of the GNU General Public License version 3.
//
// This file is part of Aladin Lite.
//
//    Aladin Lite is free software: you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation, version 3 of the License.
//
//    Aladin Lite is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.
//
//    The GNU General Public License is available in COPYING file
//    along with Aladin Lite.
//




/******************************************************************************
 * Aladin Lite project
 * 
 * File View.js
 * 
 * Author: Thomas Boch[CDS]
 * 
 *****************************************************************************/
























// Import kernel image



let View = (function() {

    /** Constructor */
    function View (aladin, location, fovDiv, cooFrame, zoom) {
            this.aladin = aladin;
            // Add a reference to the WebGL API
            //this.webglAPI = aladin.webglAPI;
            this.options = aladin.options;
            this.aladinDiv = this.aladin.aladinDiv;
            this.popup = new _Popup_js__WEBPACK_IMPORTED_MODULE_1__["Popup"](this.aladinDiv, this);

            this.createCanvases();
            // Init the WebGL context
            // At this point, the view has been created so the image canvas too
            let shaders = Object(_Shaders_js__WEBPACK_IMPORTED_MODULE_21__["loadShaders"])();
            //console.log(shaders);
        
            // Start our Rust application. You can find `WebClient` in `src/lib.rs`
            let resources = {
                'kernel': _core_img_kernel_png__WEBPACK_IMPORTED_MODULE_22__["default"],
            };
            this.aladin.webglAPI = new _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.WebClient(shaders, resources);
            this.aladin.webglAPI.resize(500, 400);
            //this.aladin.webglAPI.setHiPS(aladin.survey);

            this.location = location;
            this.fovDiv = fovDiv;
            this.mustClearCatalog = true;
            this.mustRedrawReticle = true;
            this.imageSurveysToSet = [];
            this.mode = View.PAN;
            
            this.minFOV = this.maxFOV = null; // by default, no restriction
            this.fov_limit = 180.0;
            
            this.healpixGrid = new _HealpixGrid_js__WEBPACK_IMPORTED_MODULE_2__["HealpixGrid"](this.imageCanvas);

            
            var lon, lat;
            lon = lat = 0;
            
            this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].SIN;
            this.projection = new _libs_astro_projection_js__WEBPACK_IMPORTED_MODULE_5__["Projection"](lon, lat);
            this.projection.setProjection(this.projectionMethod);
            //this.zoomLevel = 0;
            // Prev time of the last frame
            this.prev = 0;
            //this.zoomFactor = this.computeZoomFactor(this.zoomLevel);
            this.zoomFactor = this.aladin.webglAPI.getClipZoomFactor();
    
            this.viewCenter = {lon: lon, lat: lat}; // position of center of view

            if (cooFrame) {
                this.cooFrame = cooFrame;
            } else {
                this.cooFrame = _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].GAL;
            }
            if (cooFrame.system === _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.GAL) {
                console.log()
                const GAL = _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.GALCooSys();
                this.aladin.webglAPI.setCooSystem(GAL);
            } else {
                const ICRSJ2000 = _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.ICRSJ2000CooSys();
                this.aladin.webglAPI.setCooSystem(ICRSJ2000);
            }

            if (zoom) {
                this.setZoom(zoom);
            }
            
            // current reference image survey displayed
            this.imageSurveys = new Map();
            // current catalogs displayed
            this.catalogs = [];
            // a dedicated catalog for the popup
            var c = document.createElement('canvas');
            c.width = c.height = 24;
            var ctx= c.getContext('2d');
            ctx.lineWidth = 6.0;
            ctx.beginPath();
            ctx.strokeStyle = '#eee';
            ctx.arc(12, 12, 8, 0, 2*Math.PI, true);
            ctx.stroke();
            ctx.lineWidth = 3.0;
            ctx.beginPath();
            ctx.strokeStyle = '#c38';
            ctx.arc(12, 12, 8, 0, 2*Math.PI, true);
            ctx.stroke();
            this.catalogForPopup = A.catalog({shape: c, sourceSize: 24});
            //this.catalogForPopup = A.catalog({sourceSize: 18, shape: 'circle', color: '#c38'});
            this.catalogForPopup.hide();
            this.catalogForPopup.setView(this);
            // overlays (footprints for instance)
            this.overlays = [];
            // MOCs
            this.mocs = [];
            // reference to all overlay layers (= catalogs + overlays + mocs)
            this.allOverlayLayers = []
            
    
            
            this.tileBuffer = new _TileBuffer_js__WEBPACK_IMPORTED_MODULE_12__["TileBuffer"](); // tile buffer is shared across different image surveys
            this.fixLayoutDimensions();
            
            this.firstHiPS = true;
            this.curNorder = 1;
            this.realNorder = 1;
            this.curOverlayNorder = 1;
            
            // some variables for mouse handling
            this.dragging = false;
            this.dragx = null;
            this.dragy = null;
            this.needRedraw = true;

            // zoom pinching
            this.pinchZoomParameters = {
                isPinching: false, // true if a pinch zoom is ongoing
                initialFov: undefined,
                initialDistance: undefined
            };
    
            this.downloader = new _Downloader_js__WEBPACK_IMPORTED_MODULE_13__["Downloader"](this); // the downloader object is shared across all HpxImageSurveys
            this.flagForceRedraw = false;
    
            this.fadingLatestUpdate = null;
            
            this.dateRequestRedraw = null;
            
            this.showGrid = false; // coordinates grid
            
            init(this);
            

            // listen to window resize and reshape canvases
            this.resizeTimer = null;
            var self = this;
            $(window).resize(function() {
                clearTimeout(self.resizeTimer);
                self.resizeTimer = setTimeout(function() {self.fixLayoutDimensions(self)}, 100);
            });


            // in some contexts (Jupyter notebook for instance), the parent div changes little time after Aladin Lite creation
            // this results in canvas dimension to be incorrect.
            // The following line tries to fix this issue
            setTimeout(function() {
                var computedWidth = $(self.aladinDiv).width();
                var computedHeight = $(self.aladinDiv).height();

                if (self.width!==computedWidth || self.height===computedHeight) {
                    self.fixLayoutDimensions();
                    // As the WebGL backend has been resized correctly by
                    // the previous call, we can get the zoom factor from it
                    
                    self.updateZoomState(); // needed to force recomputation of displayed FoV
                }
           }, 1000);

        };
    
    // different available modes
    View.PAN = 0;
    View.SELECT = 1;
    View.TOOL_SIMBAD_POINTER = 2;
        
    
    // TODO: should be put as an option at layer level    
    View.DRAW_SOURCES_WHILE_DRAGGING = true;
    View.DRAW_MOCS_WHILE_DRAGGING = true;

    View.CALLBACKS_THROTTLE_TIME_MS = 100; // minimum time between two consecutive callback calls

    
    // (re)create needed canvases
    View.prototype.createCanvases = function() {
        var a = $(this.aladinDiv);
        //a.find('.aladin-webglCanvas').remove();
        a.find('.aladin-imageCanvas').remove();
        a.find('.aladin-catalogCanvas').remove();
        a.find('.aladin-reticleCanvas').remove();
        a.find('.aladin-gridCanvas').remove();

        // canvas to draw the images
        //this.webglCanvas = $("<canvas class='aladin-webglCanvas'></canvas>").appendTo(this.aladinDiv)[0];
        // canvas to draw the overlays
        this.imageCanvas = $("<canvas class='aladin-imageCanvas'></canvas>").appendTo(this.aladinDiv)[0];
        // canvas to draw the grid
        this.gridCanvas = $("<canvas class='aladin-gridCanvas'></canvas>").appendTo(this.aladinDiv)[0];
        // canvas to draw the catalogs
        this.catalogCanvas = $("<canvas class='aladin-catalogCanvas'></canvas>").appendTo(this.aladinDiv)[0];
        // canvas to draw the reticle
        this.reticleCanvas = $("<canvas class='aladin-reticleCanvas'></canvas>").appendTo(this.aladinDiv)[0];

    };
    
    // called at startup and when window is resized
    // The WebGL backend is resized
    View.prototype.fixLayoutDimensions = function() {
        _Utils_js__WEBPACK_IMPORTED_MODULE_10__["Utils"].cssScale = undefined;
        
        var computedWidth = $(this.aladinDiv).width();
        var computedHeight = $(this.aladinDiv).height();

        this.width = Math.max(computedWidth, 1);
        this.height = Math.max(computedHeight, 1); // this prevents many problems when div size is equal to 0
        
        
        this.cx = this.width/2;
        this.cy = this.height/2;
        
        this.largestDim = Math.max(this.width, this.height);
        this.smallestDim = Math.min(this.width, this.height);
        this.ratio = this.largestDim/this.smallestDim;

        
        this.mouseMoveIncrement = 160/this.largestDim;

        // reinitialize 2D context
        this.imageCtx = this.imageCanvas.getContext("webgl2");
        this.aladin.webglAPI.resize(this.width, this.height);
        
        this.catalogCtx = this.catalogCanvas.getContext("2d");
        this.reticleCtx = this.reticleCanvas.getContext("2d");
        this.gridCtx = this.gridCanvas.getContext("2d");

        this.imageCtx.canvas.width = this.width;
        this.catalogCtx.canvas.width = this.width;
        this.reticleCtx.canvas.width = this.width;
        this.gridCtx.canvas.width = this.width;
        
        this.imageCtx.canvas.height = this.height;
        this.catalogCtx.canvas.height = this.height;
        this.reticleCtx.canvas.height = this.height;
        this.gridCtx.canvas.height = this.height;

        pixelateCanvasContext(this.imageCtx, this.aladin.options.pixelateCanvas);

        // change logo
        if (!this.logoDiv) {
            this.logoDiv = $(this.aladinDiv).find('.aladin-logo')[0];
        }
        if (this.width>800) {
            $(this.logoDiv).removeClass('aladin-logo-small');
            $(this.logoDiv).addClass('aladin-logo-large');
            $(this.logoDiv).css('width', '90px');
        }
        else {
            $(this.logoDiv).addClass('aladin-logo-small');
            $(this.logoDiv).removeClass('aladin-logo-large');
            $(this.logoDiv).css('width', '32px');
        }

        
        this.computeNorder();
        //this.requestRedraw();
    };

    var pixelateCanvasContext = function(ctx, pixelateFlag) {
        var enableSmoothing = ! pixelateFlag;
        ctx.imageSmoothingEnabled = enableSmoothing;
        ctx.webkitImageSmoothingEnabled = enableSmoothing;
        ctx.mozImageSmoothingEnabled = enableSmoothing;
        ctx.msImageSmoothingEnabled = enableSmoothing;
        ctx.oImageSmoothingEnabled = enableSmoothing;
    }
    

    View.prototype.setMode = function(mode) {
        this.mode = mode;
        if (this.mode==View.SELECT) {
            this.setCursor('crosshair');
        }
        else if (this.mode==View.TOOL_SIMBAD_POINTER) {
            this.popup.hide();
            this.reticleCanvas.style.cursor = '';
            $(this.reticleCanvas).addClass('aladin-sp-cursor');
        }
        else {
            this.setCursor('default');
        }
    };
    
    View.prototype.setCursor = function(cursor) {
        if (this.reticleCanvas.style.cursor==cursor) {
            return;
        }
        if (this.mode==View.TOOL_SIMBAD_POINTER) {
            return;
        }
        this.reticleCanvas.style.cursor = cursor;
    };

    
    
    /**
     * return dataURL string corresponding to the current view
     */
    View.prototype.getCanvasDataURL = function(imgType, width, height) {
        imgType = imgType || "image/png"; 
        var c = document.createElement('canvas');
        width = width || this.width;
        height = height || this.height;
        c.width = width;
        c.height = height;
        var ctx = c.getContext('2d');
        ctx.drawImage(this.imageCanvas, 0, 0, c.width, c.height);
        ctx.drawImage(this.catalogCanvas, 0, 0, c.width, c.height);
        ctx.drawImage(this.reticleCanvas, 0, 0, c.width, c.height);
        ctx.drawImage(this.gridCanvas, 0, 0, c.width, c.height);

        return c.toDataURL(imgType);
        //return c.toDataURL("image/jpeg", 0.01); // setting quality only works for JPEG (?)
    };


    /**
     * Compute the FoV in degrees of the view and update mouseMoveIncrement
     * 
     * @param view
     * @returns FoV (array of 2 elements : width and height) in degrees
     */
/*   function computeFov(view) {
        var fov = doComputeFov(view, view.zoomFactor);
        
        
        view.mouseMoveIncrement = fov/view.imageCanvas.width;
            
        return fov;
    }

    function doComputeFov(view, zoomFactor) {
        // if zoom factor < 1, we view 180°
        var fov;
        if (view.zoomFactor<1) {
            fov = 180.0;
            //fov = 360;
        }
        else {
            // TODO : fov sur les 2 dimensions !!
            // to compute FoV, we first retrieve 2 points at coordinates (0, view.cy) and (width-1, view.cy)
            var xy1 = AladinUtils.viewToXy(0, view.cy, view.width, view.height, view.largestDim, zoomFactor);
            var lonlat1 = view.projection.unproject(xy1.x, xy1.y);
            
            var xy2 = AladinUtils.viewToXy(view.imageCanvas.width-1, view.cy, view.width, view.height, view.largestDim, zoomFactor);
            var lonlat2 = view.projection.unproject(xy2.x, xy2.y);
            
            
            fov = new Coo(lonlat1.ra, lonlat1.dec).distance(new Coo(lonlat2.ra, lonlat2.dec));
        }

        fov = Math.min(180.0, fov);
        
        return fov;
    }
    */
    function updateFovDiv(view) {
        if (isNaN(view.fov)) {
            view.fovDiv.html("FoV:");
            return;
        }
        // update FoV value
        var fovStr;
        if (view.fov>1) {
            fovStr = Math.round(view.fov*100)/100 + "°";
        }
        else if (view.fov*60>1) {
            fovStr = Math.round(view.fov*60*100)/100 + "'";
        }
        else {
            fovStr = Math.round(view.fov*3600*100)/100 + '"';
        }
        view.fovDiv.html("FoV: " + fovStr);
    }
    
    
    var createListeners = function(view) {
        var hasTouchEvents = false;
        if ('ontouchstart' in window) {
            hasTouchEvents = true;
        }
        
        // various listeners
        let onDblClick = function(e) {
            var xymouse = view.imageCanvas.relMouseCoords(e);

            //var xy = AladinUtils.viewToXy(xymouse.x, xymouse.y, view.width, view.height, view.largestDim, view.zoomFactor);
            try {
                var lonlat = view.aladin.webglAPI.screenToWorld(xymouse.x, xymouse.y);
            }
            catch(err) {
                return;
            }
            var radec;
            /*if (view.aladin.webglAPI.cooSystem() === Aladin.wasmLibs.webgl.GALCooSys()) {
                radec = view.aladin.webglAPI.Gal2J2000(lonlat[0], lonlat[1]);
            } else {*/
                radec = lonlat;
            //}
            //var radec = view.aladin.webglAPI.;
            // convert to J2000 if needed
            /*if (view.cooFrame.system==CooFrameEnum.SYSTEMS.GAL) {
                radec = CooConversion.GalacticToJ2000([lonlat.ra, lonlat.dec]);
            }
            else {
                radec = lonlat;
            }*/

            view.pointTo(radec[0], radec[1], {forceAnimation: true});
        };
        if (! hasTouchEvents) {
            $(view.reticleCanvas).dblclick(onDblClick);
        }
        
        
        $(view.reticleCanvas).bind("mousedown touchstart", function(e) {
            // zoom pinching
            if (e.type==='touchstart' && e.originalEvent && e.originalEvent.targetTouches && e.originalEvent.targetTouches.length==2) {
                view.dragging = false;

                view.pinchZoomParameters.isPinching = true;
                //var fov = view.aladin.getFov();
                //view.pinchZoomParameters.initialFov = Math.max(fov[0], fov[1]);
                var fov = view.aladin.webglAPI.getFieldOfView();
                view.pinchZoomParameters.initialFov = fov;
                view.pinchZoomParameters.initialDistance = Math.sqrt(Math.pow(e.originalEvent.targetTouches[0].clientX - e.originalEvent.targetTouches[1].clientX, 2) + Math.pow(e.originalEvent.targetTouches[0].clientY - e.originalEvent.targetTouches[1].clientY, 2));

                return;
            }

            var xymouse = view.imageCanvas.relMouseCoords(e);
            if (e.originalEvent && e.originalEvent.targetTouches) {
                view.dragx = e.originalEvent.targetTouches[0].clientX;
                view.dragy = e.originalEvent.targetTouches[0].clientY;
            }
            else {
                /*
                view.dragx = e.clientX;
                view.dragy = e.clientY;
                */
                view.dragx = xymouse.x;
                view.dragy = xymouse.y;
            }


            view.dragging = true;
            if (view.mode==View.PAN) {
                view.setCursor('move');
            }
            else if (view.mode==View.SELECT) {
                view.selectStartCoo = {x: view.dragx, y: view.dragy};
            }
            view.aladin.webglAPI.pressLeftMouseButton();
            return false; // to disable text selection
        });

        //$(view.reticleCanvas).bind("mouseup mouseout touchend", function(e) {
        $(view.reticleCanvas).bind("click mouseout touchend", function(e) { // reacting on 'click' rather on 'mouseup' is more reliable when panning the view
            if (e.type==='touchend' && view.pinchZoomParameters.isPinching) {
                view.pinchZoomParameters.isPinching = false;
                view.pinchZoomParameters.initialFov = view.pinchZoomParameters.initialDistance = undefined;
    
                return;
            }


            var wasDragging = view.realDragging === true;
            var selectionHasEnded = view.mode===View.SELECT && view.dragging;

            if (view.dragging) { // if we were dragging, reset to default cursor
                view.setCursor('default');
                view.dragging = false;

                if (wasDragging) {
                    view.realDragging = false;
                
                    // call positionChanged one last time after dragging, with dragging: false
                    var posChangedFn = view.aladin.callbacksByEventName['positionChanged'];
                    if (typeof posChangedFn === 'function') {
                        var pos = view.aladin.pix2world(view.width/2, view.height/2);
                        if (pos !== undefined) {
                            posChangedFn({ra: pos[0], dec: pos[1], dragging: false});
                        }
                    }
                }
            } // end of "if (view.dragging) ... "

            if (selectionHasEnded) {
                view.aladin.fire('selectend', 
                                 view.getObjectsInBBox(view.selectStartCoo.x, view.selectStartCoo.y,
                                                       view.dragx-view.selectStartCoo.x, view.dragy-view.selectStartCoo.y));    

                view.mustRedrawReticle = true; // pour effacer selection bounding box
                view.requestRedraw();

                return;
            }



            view.mustClearCatalog = true;
            view.mustRedrawReticle = true; // pour effacer selection bounding box
            view.dragx = view.dragy = null;



            if (e.type==="mouseout" || e.type==="touchend") {
                view.requestRedraw(true);
                updateLocation(view, view.width/2, view.height/2, true);


                if (e.type==="mouseout") {
                    if (view.mode===View.TOOL_SIMBAD_POINTER) {
                        view.setMode(View.PAN);
                    }

                    return;
                }
            }

            var xymouse = view.imageCanvas.relMouseCoords(e);

            if (view.mode==View.TOOL_SIMBAD_POINTER) {
                var radec = view.aladin.pix2world(xymouse.x, xymouse.y);

                view.setMode(View.PAN);
                view.setCursor('wait');

                _SimbadPointer_js__WEBPACK_IMPORTED_MODULE_11__["SimbadPointer"].query(radec[0], radec[1], Math.min(1, 15 * view.fov / view.largestDim), view.aladin);

                return; // when in TOOL_SIMBAD_POINTER mode, we do not call the listeners
            }

            // popup to show ?
            var objs = view.closestObjects(xymouse.x, xymouse.y, 5);
            if (! wasDragging && objs) {
                var o = objs[0];

                // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
                if (o instanceof _Footprint_js__WEBPACK_IMPORTED_MODULE_16__["Footprint"] || o instanceof _Circle_js__WEBPACK_IMPORTED_MODULE_17__["Circle"]) {
                    o.dispatchClickEvent();
                }

                // display marker
                else if (o.marker) {
                    // could be factorized in Source.actionClicked
                    view.popup.setTitle(o.popupTitle);
                    view.popup.setText(o.popupDesc);
                    view.popup.setSource(o);
                    view.popup.show();
                }
                // show measurements
                else {
                    if (view.lastClickedObject) {
                        view.lastClickedObject.actionOtherObjectClicked && view.lastClickedObject.actionOtherObjectClicked();
                    }
                    o.actionClicked();
                }
                view.lastClickedObject = o;
                var objClickedFunction = view.aladin.callbacksByEventName['objectClicked'];
                (typeof objClickedFunction === 'function') && objClickedFunction(o);
            }
            else {
                if (view.lastClickedObject && ! wasDragging) {
                    view.aladin.measurementTable.hide();
                    view.popup.hide();

                    if (view.lastClickedObject instanceof _Footprint_js__WEBPACK_IMPORTED_MODULE_16__["Footprint"]) {
                        //view.lastClickedObject.deselect();
                    }
                    else {
                        view.lastClickedObject.actionOtherObjectClicked();
                    }

                    view.lastClickedObject = null;
                    var objClickedFunction = view.aladin.callbacksByEventName['objectClicked'];
                    (typeof objClickedFunction === 'function') && objClickedFunction(null);
                }
            }

            // call listener of 'click' event
            var onClickFunction = view.aladin.callbacksByEventName['click'];
            if (typeof onClickFunction === 'function') {
                var pos = view.aladin.pix2world(xymouse.x, xymouse.y);
                if (pos !== undefined) {
                    onClickFunction({ra: pos[0], dec: pos[1], x: xymouse.x, y: xymouse.y, isDragging: wasDragging});
                }
            }


            // TODO : remplacer par mecanisme de listeners
            // on avertit les catalogues progressifs
            view.refreshProgressiveCats();

            view.requestRedraw(true);
            view.aladin.webglAPI.releaseLeftButtonMouse();
        });
        var lastHoveredObject; // save last object hovered by mouse
        var lastMouseMovePos = null;
        let webglAPI = view.aladin.webglAPI;
        $(view.reticleCanvas).bind("mousemove touchmove", function(e) {
            e.preventDefault();

            if (e.type==='touchmove' && view.pinchZoomParameters.isPinching && e.originalEvent && e.originalEvent.touches && e.originalEvent.touches.length==2) {
                var dist = Math.sqrt(Math.pow(e.originalEvent.touches[0].clientX - e.originalEvent.touches[1].clientX, 2) + Math.pow(e.originalEvent.touches[0].clientY - e.originalEvent.touches[1].clientY, 2));
                view.setZoom(view.pinchZoomParameters.initialFov * view.pinchZoomParameters.initialDistance / dist);

                return;
            }

            var xymouse = view.imageCanvas.relMouseCoords(e);
            if (!view.dragging || hasTouchEvents) {
                // update location box
                updateLocation(view, xymouse.x, xymouse.y);
                // call listener of 'mouseMove' event
                var onMouseMoveFunction = view.aladin.callbacksByEventName['mouseMove'];
                if (typeof onMouseMoveFunction === 'function') {
                    var pos = view.aladin.pix2world(xymouse.x, xymouse.y);
                    if (pos !== undefined) {
                        onMouseMoveFunction({ra: pos[0], dec: pos[1], x: xymouse.x, y: xymouse.y});
                    }
                    // send null ra and dec when we go out of the "sky"
                    else if (lastMouseMovePos != null) {
                        onMouseMoveFunction({ra: null, dec: null, x: xymouse.x, y: xymouse.y});
                    }
                    lastMouseMovePos = pos;
                }


                if (!view.dragging && ! view.mode==View.SELECT) {
                    // objects under the mouse ?
                    var closest = view.closestObjects(xymouse.x, xymouse.y, 5);
                    if (closest) {
                        view.setCursor('pointer');
                        var objHoveredFunction = view.aladin.callbacksByEventName['objectHovered'];
                        if (typeof objHoveredFunction === 'function' && closest[0]!=lastHoveredObject) {
                            var ret = objHoveredFunction(closest[0]);
                        }
                        lastHoveredObject = closest[0];
        
                    }
                    else {
                        view.setCursor('default');
                        var objHoveredFunction = view.aladin.callbacksByEventName['objectHovered'];
                        if (typeof objHoveredFunction === 'function' && lastHoveredObject) {
                            lastHoveredObject = null;
                            // call callback function to notify we left the hovered object
                            var ret = objHoveredFunction(null);
                        }
                    }
                }
                if (!hasTouchEvents) {
                    return;
                }
            }

            if (! view.dragging) {
                return;
            }

            //var xoffset, yoffset;
            var s1, s2;
            if (e.originalEvent && e.originalEvent.targetTouches) {
                /*xoffset = e.originalEvent.targetTouches[0].clientX-view.dragx;
                yoffset = e.originalEvent.targetTouches[0].clientY-view.dragy;
                var xy1 = AladinUtils.viewToXy(e.originalEvent.targetTouches[0].clientX, e.originalEvent.targetTouches[0].clientY, view.width, view.height, view.largestDim, view.zoomFactor);
                var xy2 = AladinUtils.viewToXy(view.dragx, view.dragy, view.width, view.height, view.largestDim, view.zoomFactor);

                pos1 = view.projection.unproject(xy1.x, xy1.y);
                pos2 = view.projection.unproject(xy2.x, xy2.y);*/
                s1 = {x: view.dragx, y: view.dragy};
                s2 = {x: e.originalEvent.targetTouches[0].clientX, y: e.originalEvent.targetTouches[0].clientY};
            }
            else {
                /*
                xoffset = e.clientX-view.dragx;
                yoffset = e.clientY-view.dragy;

                xoffset = xymouse.x-view.dragx;
                yoffset = xymouse.y-view.dragy;
                var xy1 = AladinUtils.viewToXy(xymouse.x, xymouse.y, view.width, view.height, view.largestDim, view.zoomFactor);
                var xy2 = AladinUtils.viewToXy(view.dragx, view.dragy, view.width, view.height, view.largestDim, view.zoomFactor);
                */
                //pos1 = view.projection.unproject(xy1.x, xy1.y);
                //pos2 = view.projection.unproject(xy2.x, xy2.y);
                //console.log(view.dragx, view.dragy)
                //console.log(xymouse)

                /*pos1 = webglAPI.screenToWorld(view.dragx, view.dragy);
                pos2 = webglAPI.screenToWorld(xymouse.x, xymouse.y);

                if (pos2 == undefined)  {
                    return;
                }*/
                s1 = {x: view.dragx, y: view.dragy};
                s2 = {x: xymouse.x, y: xymouse.y};
            }
            
            // TODO : faut il faire ce test ??
//            var distSquared = xoffset*xoffset+yoffset*yoffset;
//            if (distSquared<3) {
//                return;
//            }
            if (e.originalEvent && e.originalEvent.targetTouches) {
                view.dragx = e.originalEvent.targetTouches[0].clientX;
                view.dragy = e.originalEvent.targetTouches[0].clientY;
            }
            else {
                view.dragx = xymouse.x;
                view.dragy = xymouse.y;
                /*
                view.dragx = e.clientX;
                view.dragy = e.clientY;
                */
            }
            
            if (view.mode==View.SELECT) {
                  view.requestRedraw();
                  return;
            }

            //view.viewCenter.lon += xoffset*view.mouseMoveIncrement/Math.cos(view.viewCenter.lat*Math.PI/180.0);
            /*
            view.viewCenter.lon += xoffset*view.mouseMoveIncrement;
            view.viewCenter.lat += yoffset*view.mouseMoveIncrement;
            */
            
            //view.viewCenter.lon = pos2.ra -  pos1.ra;
            //view.viewCenter.lat = pos2.dec - pos1.dec;
            //view.viewCenter.lon = pos2.ra;
            //view.viewCenter.lon = pos2.ra;

            
            // can not go beyond poles
            if (view.viewCenter.lat>90) {
                view.viewCenter.lat = 90;
            }
            else if (view.viewCenter.lat < -90) {
                view.viewCenter.lat = -90;
            }
            
            // limit lon to [0, 360]
            if (view.viewCenter.lon < 0) {
                view.viewCenter.lon = 360 + view.viewCenter.lon;
            }
            else if (view.viewCenter.lon > 360) {
                view.viewCenter.lon = view.viewCenter.lon % 360;
            }
            view.realDragging = true;

            //webglAPI.goFromTo(pos1[0], pos1[1], pos2[0], pos2[1]);
            webglAPI.goFromTo(s1.x, s1.y, s2.x, s2.y);
            //webglAPI.setCenter(pos2[0], pos2[1]);
            let viewCenter = webglAPI.getCenter();
            view.viewCenter.lon = viewCenter[0];
            view.viewCenter.lat = viewCenter[1];


            //console.log(view.viewCenter);

            view.requestRedraw();
        }); //// endof mousemove ////
        
        // disable text selection on IE
        $(view.aladinDiv).onselectstart = function () { return false; }

        $(view.reticleCanvas).on('wheel', function(event) {
            event.preventDefault();
            event.stopPropagation();
            //var level = view.zoomLevel;

            var delta = event.deltaY;
            // this seems to happen in context of Jupyter notebook --> we have to invert the direction of scroll
            // hope this won't trigger some side effects ...
            if (event.hasOwnProperty('originalEvent')) {
                delta = -event.originalEvent.deltaY;
            } 
            /*if (delta>0) {
                level += 1;
                //zoom
            }
            else {
                level -= 1;
                //unzoom
            }*/
            // The value of the field of view is determined
            // inside the backend
            view.aladin.webglAPI.registerWheelEvent(delta);
            view.updateZoomState();

            if (! view.debounceProgCatOnZoom) {
                var self = view;
                view.debounceProgCatOnZoom = _Utils_js__WEBPACK_IMPORTED_MODULE_10__["Utils"].debounce(function() {self.refreshProgressiveCats();}, 300);
            }
            view.debounceProgCatOnZoom();
            //view.setZoomLevel(level);
            //view.refreshProgressiveCats();
            return false;
        });

    };
    
    var init = function(view) {
        var stats = new _libs_Stats_js__WEBPACK_IMPORTED_MODULE_14__["Stats"]();
        stats.domElement.style.top = '50px';
        if ($('#aladin-statsDiv').length>0) {
            $('#aladin-statsDiv')[0].appendChild( stats.domElement );
        }
        
        view.stats = stats;

        createListeners(view);

        view.executeCallbacksThrottled = _Utils_js__WEBPACK_IMPORTED_MODULE_10__["Utils"].throttle(
            function() {
                var pos = view.aladin.pix2world(view.width/2, view.height/2);
                var fov = view.fov;
                if (pos===undefined || fov===undefined) {
                    return;
                }

                var ra = pos[0];
                var dec = pos[1];
                // trigger callback only if position has changed !
                if (ra!==this.ra || dec!==this.dec) {
                    var posChangedFn = view.aladin.callbacksByEventName['positionChanged'];
                    (typeof posChangedFn === 'function') && posChangedFn({ra: ra, dec: dec, dragging: true});
    
                    // finally, save ra and dec value
                    this.ra = ra;
                    this.dec = dec;
                }

                // trigger callback only if FoV (zoom) has changed !
                if (fov!==this.old_fov) {
                    var fovChangedFn = view.aladin.callbacksByEventName['zoomChanged'];
                    (typeof fovChangedFn === 'function') && fovChangedFn(fov);
    
                    // finally, save fov value
                    this.old_fov = fov;
                }

            },
            View.CALLBACKS_THROTTLE_TIME_MS);


        view.displayHpxGrid = false;
        view.displaySurvey = true;
        view.displayCatalog = false;
        view.displayReticle = true;

        // initial draw
        //view.fov = computeFov(view);
        //updateFovDiv(view);
        //view.redraw();
    };

    function updateLocation(view, x, y, isViewCenterPosition) {
        if (!view.projection) {
            return;
        }
        //var xy = AladinUtils.viewToXy(x, y, view.width, view.height, view.largestDim, view.zoomFactor);

        var lonlat;
        try {
            lonlat = view.aladin.webglAPI.screenToWorld(x, y);
            //lonlat = view.projection.unproject(xy.x, xy.y);
        } catch(err) {
        }
        
        if (lonlat) {
            // Convert it to galactic
            if (view.aladin.webglAPI.cooSystem() === _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.GALCooSys()) {
                lonlat = view.aladin.webglAPI.J20002Gal(lonlat[0], lonlat[1]);
            }

            view.location.update(lonlat[0], lonlat[1], view.cooFrame, isViewCenterPosition);
        }
    }
    
    View.prototype.requestRedrawAtDate = function(date) {
        this.dateRequestDraw = date;
    };

    /**
     * Return the color of the lowest intensity pixel 
     * in teh current color map of the current background image HiPS
     */
    View.prototype.getBackgroundColor = function() {
        var white = 'rgb(255, 255, 255)';
        var black = 'rgb(0, 0, 0)';

        if (! this.imageSurvey) {
            return black;
        }

        var cm = this.imageSurvey.getColorMap();
        if (!cm) {
            return black;
        }
        if (cm.mapName == 'native' || cm.mapName == 'grayscale') {
            return cm.reversed ? white : black;
        }

        var idx = cm.reversed ? 255 : 0;
        var r = _ColorMap_js__WEBPACK_IMPORTED_MODULE_15__["ColorMap"].MAPS[cm.mapName].r[idx];
        var g = _ColorMap_js__WEBPACK_IMPORTED_MODULE_15__["ColorMap"].MAPS[cm.mapName].g[idx];
        var b = _ColorMap_js__WEBPACK_IMPORTED_MODULE_15__["ColorMap"].MAPS[cm.mapName].b[idx];

        return 'rgb(' + r + ',' + g + ',' + b + ')';
    };

    View.prototype.getViewParams = function() {
        var resolution = this.width > this.height ? this.fov / this.width : this.fov / this.height;
        return {
            fov: [this.width * resolution, this.height * resolution],   
            width: this.width,   
            height: this.height   
        };
    };

    /**
     * redraw the whole view
     */
    View.prototype.redraw = function() {
        var saveNeedRedraw = this.needRedraw;
        Object(_libs_RequestAnimationFrame_js__WEBPACK_IMPORTED_MODULE_20__["requestAnimFrame"])(this.redraw.bind(this));
        var now = Date.now();
        var dt = now - this.prev;

        this.ready = this.aladin.webglAPI.isReady();
        if (this.imageSurveysToSet !== null && (this.firstHiPS || this.ready)) {
            console.log("surveyes", this.imageSurveysToSet)
            this.aladin.webglAPI.setImageSurveys(this.imageSurveysToSet);

            this.imageSurveysToSet = null;
            this.firstHiPS = false;
        }

        this.aladin.webglAPI.update(dt, this.needRedraw);
        // This is called at each frame
        // Better way is to give this function
        // to Rust so that the backend executes it
        // only when necessary, i.e. during the zoom
        // animation
        updateFovDiv(this);
        // check whether a catalog has been parsed and
        // is ready to be plot
        let catReady = this.aladin.webglAPI.isCatalogLoaded();
        if (catReady) {
            var callbackFn = this.aladin.callbacksByEventName['catalogReady'];
            (typeof callbackFn === 'function') && callbackFn();
        }
        this.aladin.webglAPI.render(this.needRedraw);

        var imageCtx = this.imageCtx;
        //////// 1. Draw images ////////
        /*if (imageCtx.start2D) {
            imageCtx.start2D();
        }*/
        //// clear canvas ////
        // TODO : do not need to clear if fov small enough ?
        /*imageCtx.clearRect(0, 0, this.imageCanvas.width, this.imageCanvas.height);
        ////////////////////////
    
        var bkgdColor = this.getBackgroundColor();    
        // fill with background of the same color than the first color map value (lowest intensity)
        if (this.projectionMethod==ProjectionEnum.SIN) {
            if (this.fov>=60) {
                imageCtx.fillStyle = bkgdColor;
                imageCtx.beginPath();
                var maxCxCy = this.cx>this.cy ? this.cx : this.cy;
                imageCtx.arc(this.cx, this.cy, maxCxCy * this.zoomFactor, 0, 2*Math.PI, true);
                imageCtx.fill();
            }
            // pour eviter les losanges blancs qui apparaissent quand les tuiles sont en attente de chargement
            else {
                imageCtx.fillStyle = bkgdColor;
                imageCtx.fillRect(0, 0, this.imageCanvas.width, this.imageCanvas.height);
            }
        }
        else if (this.projectionMethod==ProjectionEnum.AITOFF) {
            if (imageCtx.ellipse) {
                imageCtx.fillStyle = bkgdColor;
                imageCtx.beginPath();
                imageCtx.ellipse(this.cx, this.cy, 2.828*this.cx*this.zoomFactor, this.cx*this.zoomFactor*1.414, 0, 0, 2*Math.PI);
                imageCtx.fill();
            }
        }*/
        /*if (imageCtx.finish2D) {
            imageCtx.finish2D();
        }*/

        
        this.projection.setCenter(this.viewCenter.lon, this.viewCenter.lat);
        // do we have to redo that every time? Probably not
        //this.projection.setProjection(this.projectionMethod);


        // ************* Draw allsky tiles (low resolution) *****************

        var cornersXYViewMapHighres = null;
        // Pour traitement des DEFORMATIONS --> TEMPORAIRE, draw deviendra la methode utilisee systematiquement

        /*if (this.imageSurvey && this.imageSurvey.isReady && this.displaySurvey) {
                if (this.aladin.reduceDeformations==null) {
                    this.imageSurvey.draw(imageCtx, this, !this.dragging, this.curNorder);
                }

                else {
                    this.imageSurvey.draw(imageCtx, this, this.aladin.reduceDeformations, this.curNorder);
                }
        }*/
        /*
        else {
            var cornersXYViewMapAllsky = this.getVisibleCells(3);
            var cornersXYViewMapHighres = null;
            if (this.curNorder>=3) {
                if (this.curNorder==3) {
                    cornersXYViewMapHighres = cornersXYViewMapAllsky;
                }
                else {
                    cornersXYViewMapHighres = this.getVisibleCells(this.curNorder);
                }
            }

            // redraw image survey
            if (this.imageSurvey && this.imageSurvey.isReady && this.displaySurvey) {
                // TODO : a t on besoin de dessiner le allsky si norder>=3 ?
                // TODO refactoring : should be a method of HpxImageSurvey
                this.imageSurvey.redrawAllsky(imageCtx, cornersXYViewMapAllsky, this.fov, this.curNorder);
                if (this.curNorder>=3) {
                    this.imageSurvey.redrawHighres(imageCtx, cornersXYViewMapHighres, this.curNorder);
                }
            }
        }
        */
        

        // redraw overlay image survey
        // TODO : does not work if different frames 
        // TODO: use HpxImageSurvey.draw method !!
        if (this.overlayImageSurvey && this.overlayImageSurvey.isReady) {
            /*imageCtx.globalAlpha = this.overlayImageSurvey.getAlpha();

            if (this.aladin.reduceDeformations==null) {
                this.overlayImageSurvey.draw(imageCtx, this, !this.dragging, this.curOverlayNorder);
            }

            else {
                this.overlayImageSurvey.draw(imageCtx, this, this.aladin.reduceDeformations, this.curOverlayNorder);
            }*/
            /*
            if (this.fov>50) {
                this.overlayImageSurvey.redrawAllsky(imageCtx, cornersXYViewMapAllsky, this.fov, this.curOverlayNorder);
            }
            if (this.curOverlayNorder>=3) {
                var norderOverlay = Math.min(this.curOverlayNorder, this.overlayImageSurvey.maxOrder);
                if ( cornersXYViewMapHighres==null || norderOverlay != this.curNorder ) {
                    cornersXYViewMapHighres = this.getVisibleCells(norderOverlay);
                }
                this.overlayImageSurvey.redrawHighres(imageCtx, cornersXYViewMapHighres, norderOverlay);
            }
            */

           //imageCtx.globalAlpha = 1.0;

        }
        
        // redraw HEALPix grid
        if( this.displayHpxGrid) {
            var cornersXYViewMapAllsky = this.getVisibleCells(3);
            var cornersXYViewMapHighres = null;
            if (this.curNorder>=3) {
                if (this.curNorder==3) {
                    cornersXYViewMapHighres = cornersXYViewMapAllsky;
                }
                else {
                    cornersXYViewMapHighres = this.getVisibleCells(this.curNorder);
                }
            }
            this.gridCtx.clearRect(0, 0, this.imageCanvas.width, this.imageCanvas.height);
            if (cornersXYViewMapHighres && this.curNorder>3) {
                this.healpixGrid.redraw(this.gridCtx, cornersXYViewMapHighres, this.fov, this.curNorder);
            }
            else {
                this.healpixGrid.redraw(this.gridCtx, cornersXYViewMapAllsky, this.fov, 3);
            }
        }
        
        // redraw coordinates grid
        /*if (this.showGrid) {
            if (this.cooGrid==null) {
                this.cooGrid = new CooGrid();
            }
            
            this.cooGrid.redraw(this.gridCtx, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor, this.fov);
        }*/
         


        ///*
        ////// 2. Draw catalogues////////
        var catalogCtx = this.catalogCtx;

        var catalogCanvasCleared = false;
        if (this.mustClearCatalog) {
            catalogCtx.clearRect(0, 0, this.width, this.height);
            catalogCanvasCleared = true;
            this.mustClearCatalog = false;
        }
        if (this.catalogs && this.catalogs.length>0 && this.displayCatalog && (! this.dragging  || View.DRAW_SOURCES_WHILE_DRAGGING)) {
              // TODO : do not clear every time
            //// clear canvas ////
            if (! catalogCanvasCleared) {
                catalogCtx.clearRect(0, 0, this.width, this.height);
                catalogCanvasCleared = true;
            }
            for (var i=0; i<this.catalogs.length; i++) {
                var cat = this.catalogs[i];
                //console.log( this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor);
                cat.draw(catalogCtx, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor);
            }
        }
        // draw popup catalog
        if (this.catalogForPopup.isShowing && this.catalogForPopup.sources.length>0) {
            if (! catalogCanvasCleared) {
                catalogCtx.clearRect(0, 0, this.width, this.height);
                catalogCanvasCleared = true;
            }
            this.catalogForPopup.draw(catalogCtx, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor);
        }

        ////// 3. Draw overlays////////
        var overlayCtx = this.catalogCtx;
        if (this.overlays && this.overlays.length>0 && (! this.dragging  || View.DRAW_SOURCES_WHILE_DRAGGING)) {
            if (! catalogCanvasCleared) {
                catalogCtx.clearRect(0, 0, this.width, this.height);
                catalogCanvasCleared = true;
            }
            for (var i=0; i<this.overlays.length; i++) {
                this.overlays[i].draw(overlayCtx, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor);
            }
        }
        

        // draw MOCs
        var mocCtx = this.catalogCtx;
        if (this.mocs && this.mocs.length>0 && (! this.dragging  || View.DRAW_MOCS_WHILE_DRAGGING)) {
            if (! catalogCanvasCleared) {
                catalogCtx.clearRect(0, 0, this.width, this.height);
                catalogCanvasCleared = true;
            }
            for (var i=0; i<this.mocs.length; i++) {
                this.mocs[i].draw(mocCtx, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor, this.fov);
            }
        }

        //*/
        if (this.mode==View.SELECT) {
            mustRedrawReticle = true;
        }
        
        ////// 4. Draw reticle ///////
        // TODO: reticle should be placed in a static DIV, no need to waste a canvas
        var reticleCtx = this.reticleCtx;
        if (this.mustRedrawReticle || this.mode==View.SELECT) {
            reticleCtx.clearRect(0, 0, this.width, this.height);
        }
        if (this.displayReticle) {
            
            if (! this.reticleCache) {
                // build reticle image
                var c = document.createElement('canvas');
                var s = this.options.reticleSize;
                c.width = s;
                c.height = s;
                var ctx = c.getContext('2d');
                ctx.lineWidth = 2;
                ctx.strokeStyle = this.options.reticleColor;
                ctx.beginPath();
                ctx.moveTo(s/2, s/2+(s/2-1));
                ctx.lineTo(s/2, s/2+2);
                ctx.moveTo(s/2, s/2-(s/2-1));
                ctx.lineTo(s/2, s/2-2);
                
                ctx.moveTo(s/2+(s/2-1), s/2);
                ctx.lineTo(s/2+2,  s/2);
                ctx.moveTo(s/2-(s/2-1), s/2);
                ctx.lineTo(s/2-2,  s/2);
                
                ctx.stroke();
                
                this.reticleCache = c;
            }
                
            reticleCtx.drawImage(this.reticleCache, this.width/2 - this.reticleCache.width/2, this.height/2 - this.reticleCache.height/2);
            
            
            this.mustRedrawReticle = false;
        }
        /*
        ////// 5. Draw all-sky ring /////
        if (this.projectionMethod==ProjectionEnum.SIN && this.fov>=60 && this.aladin.options['showAllskyRing'] === true) {
                    imageCtx.strokeStyle = this.aladin.options['allskyRingColor'];
                    var ringWidth = this.aladin.options['allskyRingWidth'];
                    imageCtx.lineWidth = ringWidth;
                    imageCtx.beginPath();
                    var maxCxCy = this.cx>this.cy ? this.cx : this.cy;
                    imageCtx.arc(this.cx, this.cy, (maxCxCy-(ringWidth/2.0)+1) * this.zoomFactor, 0, 2*Math.PI, true);
                    imageCtx.stroke();
        }

        
        // draw selection box
        if (this.mode==View.SELECT && this.dragging) {
            reticleCtx.fillStyle = "rgba(100, 240, 110, 0.25)";
            var w = this.dragx - this.selectStartCoo.x;
            var h =  this.dragy - this.selectStartCoo.y;
            
            reticleCtx.fillRect(this.selectStartCoo.x, this.selectStartCoo.y, w, h);
        }
        */
        
         // TODO : is this the right way?
         if (saveNeedRedraw==this.needRedraw) {
             this.needRedraw = false;
         }


        // objects lookup
        if (!this.dragging) {
            this.updateObjectsLookup();
        } 

        // execute 'positionChanged' and 'zoomChanged' callbacks
        //this.executeCallbacksThrottled();
        this.prev = now;

    };

    /*View.prototype.drawGridLabels = function (text) {
        //let ctx = this.imageCanvas.getContext("webgl2");
        //var c = document.getElementById("myCanvas");
        //var ctx = c.getContext("2d");
        
        this.reticleCtx.font = "30px Verdana";
        this.reticleCtx.fillText(text, 200, 50);
        //ctx.font = "30px Verdana";
    }*/

    View.prototype.forceRedraw = function() {
        this.flagForceRedraw = true;
    };
    
    View.prototype.refreshProgressiveCats = function() {
        if (! this.catalogs) {
            return;
        }

        for (var i=0; i<this.catalogs.length; i++) {
            if (this.catalogs[i].type=='progressivecat') {
                this.catalogs[i].loadNeededTiles();
            }
        }
    };

    View.prototype.getVisiblePixList = function(norder, frameSurvey) {
        var nside = Math.pow(2, norder);

        var pixList;
        var npix = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].nside2Npix(nside);
        if (this.fov>80) {
            pixList = [];
            for (var ipix=0; ipix<npix; ipix++) {
                pixList.push(ipix);
            }
        }
        else {
            var hpxIdx = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"](nside);
            hpxIdx.init();
            var spatialVector = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["SpatialVector"]();
            // if frame != frame image survey, we need to convert to survey frame system
            //var xy = AladinUtils.viewToXy(this.cx, this.cy, this.width, this.height, this.largestDim, this.zoomFactor);
            //var radec = this.projection.unproject(xy.x, xy.y);
            let pos_world = this.aladin.webglAPI.screenToWorld(this.cx, this.cy);
            let radec = {
                ra: pos_world[0],
                dec: pos_world[1]
            };
            var lonlat = [];
            /*if (frameSurvey && frameSurvey.system != this.cooFrame.system) {
                if (frameSurvey.system==CooFrameEnum.SYSTEMS.J2000) {
                    lonlat = CooConversion.GalacticToJ2000([radec.ra, radec.dec]);
                }
                else if (frameSurvey.system==CooFrameEnum.SYSTEMS.GAL) {
                    lonlat = CooConversion.J2000ToGalactic([radec.ra, radec.dec]);
                }
            }
            else {
                lonlat = [radec.ra, radec.dec];
            }*/
            lonlat = [radec.ra, radec.dec];
            spatialVector.set(lonlat[0], lonlat[1]);

            var radius = this.fov*0.5*this.ratio;
            // we need to extend the radius
            if (this.fov>60) {
                radius *= 1.6;
            }
            else if (this.fov>12) {
                radius *=1.45;
            }
            else {
                radius *= 1.1;
            }



            pixList = hpxIdx.queryDisc(spatialVector, radius*Math.PI/180.0, true, true);
            // add central pixel at index 0
            var polar = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].utils.radecToPolar(lonlat[0], lonlat[1]);
            var ipixCenter = hpxIdx.ang2pix_nest(polar.theta, polar.phi);
            pixList.unshift(ipixCenter);

        }

        return pixList;
    };
    
    View.prototype.setAngleRotation = function(theta) {

    }

    // TODO: optimize this method !!
    View.prototype.getVisibleCells = function(norder, frameSurvey) {
        if (! frameSurvey && this.imageSurvey) {
            frameSurvey = this.imageSurvey.cooFrame;
        }
        var cells = []; // array to be returned
        var cornersXY = [];
        var spVec = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["SpatialVector"]();
        var nside = Math.pow(2, norder); // TODO : to be modified
        var npix = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].nside2Npix(nside);
        var ipixCenter = null;
        
        // build list of pixels
        // TODO: pixList can be obtained from getVisiblePixList
        var pixList;
        if (this.fov>80) {
            pixList = [];
            for (var ipix=0; ipix<npix; ipix++) {
                pixList.push(ipix);
            }
        }
        else {
            var hpxIdx = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"](nside);
            hpxIdx.init();
            var spatialVector = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["SpatialVector"]();
            // if frame != frame image survey, we need to convert to survey frame system
            var xy = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_7__["AladinUtils"].viewToXy(this.cx, this.cy, this.width, this.height, this.largestDim, this.zoomFactor);
            //var radec = this.projection.unproject(xy.x, xy.y);
            var radec = this.aladin.webglAPI.screenToWorld(this.cx, this.cy);
            var radec = {
                ra: radec[0],
                dec: radec[1],
            };
            var lonlat = [];
            if (frameSurvey && frameSurvey.system != this.cooFrame.system) {
                if (frameSurvey.system==_CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.J2000) {
                    lonlat = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].GalacticToJ2000([radec.ra, radec.dec]); 
                }
                else if (frameSurvey.system==_CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.GAL) {
                    lonlat = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].J2000ToGalactic([radec.ra, radec.dec]);
                }
            }
            else {
                lonlat = [radec.ra, radec.dec];
            }
            spatialVector.set(lonlat[0], lonlat[1]);

            var radius = this.fov*0.5*this.ratio;
            // we need to extend the radius
            if (this.fov>60) {
                radius *= 1.6;
            }
            else if (this.fov>12) {
                radius *=1.45;
            }
            else {
                radius *= 1.1;
            }
            
            
                
            pixList = hpxIdx.queryDisc(spatialVector, radius*Math.PI/180.0, true, true);
            // add central pixel at index 0
            var polar = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].utils.radecToPolar(lonlat[0], lonlat[1]);
            ipixCenter = hpxIdx.ang2pix_nest(polar.theta, polar.phi);
            pixList.unshift(ipixCenter);
        }
        
        
        var ipix;
        var lon, lat;
        var corners;
        for (var ipixIdx=0, len=pixList.length; ipixIdx<len; ipixIdx++) {
            ipix = pixList[ipixIdx];
            if (ipix==ipixCenter && ipixIdx>0) { 
                continue;
            }
            var cornersXYView = [];
            corners = _HealpixCache_js__WEBPACK_IMPORTED_MODULE_9__["HealpixCache"].corners_nest(ipix, nside);

            for (var k=0; k<4; k++) {
                spVec.setXYZ(corners[k].x, corners[k].y, corners[k].z);
                
                // need for frame transformation ?
                if (frameSurvey && frameSurvey.system != this.cooFrame.system) {
                    if (frameSurvey.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.J2000) {
                        var radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].J2000ToGalactic([spVec.ra(), spVec.dec()]); 
                        lon = radec[0];
                        lat = radec[1];
                    }
                    else if (frameSurvey.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.GAL) {
                        var radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].GalacticToJ2000([spVec.ra(), spVec.dec()]); 
                        lon = radec[0];
                        lat = radec[1];
                    }
                }
                else {
                    lon = spVec.ra();
                    lat = spVec.dec();
                }
                
                //cornersXY[k] = this.projection.project(lon, lat);
                cornersXY[k] = this.aladin.webglAPI.worldToScreen(lon, lat);
            }


            if (cornersXY[0] == null ||  cornersXY[1] == null  ||  cornersXY[2] == null ||  cornersXY[3] == null ) {
                continue;
            }



            for (var k=0; k<4; k++) {
                //cornersXYView[k] = AladinUtils.xyToView(cornersXY[k].X, cornersXY[k].Y, this.width, this.height, this.largestDim, this.zoomFactor);
                cornersXYView[k] = {
                    vx: cornersXY[k][0],
                    vy: cornersXY[k][1],
                };
            }

            var indulge = 10;
            // detect pixels outside view. Could be improved !
            // we minimize here the number of cells returned
            if( cornersXYView[0].vx<0 && cornersXYView[1].vx<0 && cornersXYView[2].vx<0 &&cornersXYView[3].vx<0) {
                continue;
            }
            if( cornersXYView[0].vy<0 && cornersXYView[1].vy<0 && cornersXYView[2].vy<0 &&cornersXYView[3].vy<0) {
                continue;
            }
            if( cornersXYView[0].vx>=this.width && cornersXYView[1].vx>=this.width && cornersXYView[2].vx>=this.width &&cornersXYView[3].vx>=this.width) {
                continue;
            }
            if( cornersXYView[0].vy>=this.height && cornersXYView[1].vy>=this.height && cornersXYView[2].vy>=this.height &&cornersXYView[3].vy>=this.height) {
                continue;
            }


            // check if pixel is visible
//            if (this.fov<160) { // don't bother checking if fov is large enough
//                if ( ! AladinUtils.isHpxPixVisible(cornersXYView, this.width, this.height) ) {
//                    continue;
//                }
//            }
            // check if we have a pixel at the edge of the view in allsky projections
            if (this.projection.PROJECTION!=_ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].SIN && this.projection.PROJECTION!=_ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].TAN) {
               /* console.log(this.largestDim);
                var xdiff = cornersXYView[0].vx-cornersXYView[2].vx;
                var ydiff = cornersXYView[0].vy-cornersXYView[2].vy;
                var distDiag = Math.sqrt(xdiff*xdiff + ydiff*ydiff);
                if (distDiag>this.largestDim/5) {
                    continue;
                }
                xdiff = cornersXYView[1].vx-cornersXYView[3].vx;
                ydiff = cornersXYView[1].vy-cornersXYView[3].vy;
                distDiag = Math.sqrt(xdiff*xdiff + ydiff*ydiff);
                if (distDiag>this.largestDim/5) {
                    continue;
                }*/

                // New faster approach: when a vertex from a cell gets to the other side of the projection
                // its vertices order change from counter-clockwise to clockwise!
                // So if the vertices describing a cell are given in clockwise order
                // we know it crosses the projection, so we do not plot them!
                if (!_AladinUtils_js__WEBPACK_IMPORTED_MODULE_7__["AladinUtils"].counterClockwiseTriangle(cornersXYView[0].vx, cornersXYView[0].vy, cornersXYView[1].vx, cornersXYView[1].vy, cornersXYView[2].vx, cornersXYView[2].vy) ||
                    !_AladinUtils_js__WEBPACK_IMPORTED_MODULE_7__["AladinUtils"].counterClockwiseTriangle(cornersXYView[0].vx, cornersXYView[0].vy, cornersXYView[2].vx, cornersXYView[2].vy, cornersXYView[3].vx, cornersXYView[3].vy)) {
                    continue;
                }
            }
            
            cornersXYView.ipix = ipix;
            cells.push(cornersXYView);
        }
        
        return cells;
    };

    // get position in view for a given HEALPix cell
    View.prototype.getPositionsInView = function(ipix, norder) {
        var cornersXY = [];
        var lon, lat;
        var spVec = new _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["SpatialVector"]();
        var nside = Math.pow(2, norder); // TODO : to be modified
        
        
        var cornersXYView = [];  // will be returned
        var corners = _HealpixCache_js__WEBPACK_IMPORTED_MODULE_9__["HealpixCache"].corners_nest(ipix, nside);

        for (var k=0; k<4; k++) {
            spVec.setXYZ(corners[k].x, corners[k].y, corners[k].z);
                
            // need for frame transformation ?
            if (this.imageSurvey && this.imageSurvey.cooFrame.system != this.cooFrame.system) {
                if (this.imageSurvey.cooFrame.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.J2000) {
                    var radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].J2000ToGalactic([spVec.ra(), spVec.dec()]); 
                    lon = radec[0];
                    lat = radec[1];
                }
                else if (this.imageSurvey.cooFrame.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.GAL) {
                    var radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].GalacticToJ2000([spVec.ra(), spVec.dec()]); 
                    lon = radec[0];
                    lat = radec[1];
                }
            }
            else {
                lon = spVec.ra();
                lat = spVec.dec();
            }
            //cornersXY[k] = this.projection.project(lon, lat);
            let xy = this.aladin.webglAPI.worldToScreen(lon, lat);
            cornersXYView[k] = {
                vx: xy.x,
                vy: xy.y
            };
        }
        
        if (cornersXYView[0] == null ||  cornersXYView[1] == null  ||  cornersXYView[2] == null ||  cornersXYView[3] == null ) {
            return null;
        }
        /*if (cornersXY[0] == null ||  cornersXY[1] == null  ||  cornersXY[2] == null ||  cornersXY[3] == null ) {
            return null;
        }*/
        /*for (var k=0; k<4; k++) {
            cornersXYView[k] = AladinUtils.xyToView(cornersXY[k].X, cornersXY[k].Y, this.width, this.height, this.largestDim, this.zoomFactor);
        }*/

        return cornersXYView;
    };
    
    
    /*View.prototype.computeZoomFactor = function(level) {
        if (level>0) {
            return AladinUtils.getZoomFactorForAngle(180.0/Math.pow(1.35, level), this.projectionMethod);
        }
        else {
            return 1 + 0.1*level;
        }
    };*/
    /*View.prototype.computeZoomLevelFromFOV = function() {
        if (level>0) {
            return AladinUtils.getZoomFactorForAngle(180/Math.pow(1.15, level), this.projectionMethod);
        }
        else {
            return 1 + 0.1*level;
        }
    };*/
    
    // Called for touchmove events
    View.prototype.setZoom = function(fovDegrees) {
        if (fovDegrees<0) {
            return;
        }
        // Erase the field of view state of the backend by
        this.aladin.webglAPI.setFieldOfView(fovDegrees);
        //var zoomLevel = Math.log(180/fovDegrees)/Math.log(1.15);
        //this.setZoomLevel(zoomLevel);
        this.updateZoomState();
    };

    View.prototype.increaseZoom = function() {
        for (let i = 0; i < 5; i++) {
            this.aladin.webglAPI.registerWheelEvent(0.01);
        }
    }
    View.prototype.decreaseZoom = function() {
        for (let i = 0; i < 5; i++) {
            this.aladin.webglAPI.registerWheelEvent(-0.01);
        }
    }
    View.prototype.setShowGrid = function(showGrid) {
        this.showGrid = showGrid;
        if (showGrid) {
            this.aladin.webglAPI.enableGrid();
        } else {
            this.aladin.webglAPI.disableGrid();
        }
        this.requestRedraw();
    };

    //View.prototype.setZoom = function(level) {
    View.prototype.updateZoomState = function() {
        /*let zoom = {"action": undefined};

        if (this.zoomLevel > level) {
            console.log("unzoom")
            zoom["action"] = "unzoom";
        } else if (this.zoomLevel < level) {
            zoom["action"] = "zoom";
        }*/

        /*if (this.minFOV || this.maxFOV) {
            var newFov = doComputeFov(this, this.computeZoomFactor(Math.max(-2, level)));
            if (this.maxFOV && newFov>this.maxFOV  ||  this.minFOV && newFov<this.minFOV)  {
                return;
            }
        }*/

        /*if (this.projectionMethod==ProjectionEnum.SIN) {
            //this.zoomLevel = Math.max(-2, level); // TODO : canvas freezes in firefox when max level is small
            this.zoomLevel = Math.max(-7, level); // TODO : canvas freezes in firefox when max level is small
        } else {
            this.zoomLevel = Math.max(-7, level); // TODO : canvas freezes in firefox when max level is small
        }*/
        //this.zoomLevel = Math.max(-7, level);
        
        /// Old
        /*this.zoomFactor = this.computeZoomFactor(this.zoomLevel);
        this.fov = computeFov(this);

        if (this.zoomFactor >= 1.0) {
            this.aladin.webglAPI.setFieldOfView(this.fov);
        } else {
            //console.log("FOV, ", this.fov / this.zoomFactor);

            // zoom factor
            this.aladin.webglAPI.setFieldOfView(this.fov / this.zoomFactor);
        }*/
        this.zoomFactor = this.aladin.webglAPI.getClipZoomFactor();
        this.fov = this.aladin.webglAPI.getFieldOfView();

        // TODO: event/listener should be better
        //updateFovDiv(this);
        
        this.computeNorder();
        
        this.forceRedraw();
        //this.requestRedraw();
        // on avertit les catalogues progressifs

    };
    
    /**
     * compute and set the norder corresponding to the current view resolution
     */
    View.prototype.computeNorder = function() {
        var resolution = this.fov / this.largestDim; // in degree/pixel
        var tileSize = 512; // TODO : read info from HpxImageSurvey.tileSize
        var nside = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].calculateNSide(3600*tileSize*resolution); // 512 = size of a "tile" image
        var norder = Math.log(nside)/Math.log(2);
        norder = Math.max(norder, 1);
        this.realNorder = norder;

            
        // here, we force norder to 3 (otherwise, the display is "blurry" for too long when zooming in)
        if (this.fov<=50 && norder<=2) {
            norder = 3;
        }
           

        // that happens if we do not wish to display tiles coming from Allsky.[jpg|png]
        if (this.imageSurvey && norder<=2 && this.imageSurvey.minOrder>2) {
            norder = this.imageSurvey.minOrder;
        }

        var overlayNorder  = norder;
        if (this.imageSurvey && norder>this.imageSurvey.maxOrder) {
            norder = this.imageSurvey.maxOrder;
        }
        if (this.overlayImageSurvey && overlayNorder>this.overlayImageSurvey.maxOrder) {
            overlayNorder = this.overlayImageSurvey.maxOrder;
        }
        // should never happen, as calculateNSide will return something <=HealpixIndex.ORDER_MAX
        if (norder>_libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].ORDER_MAX) {
            norder = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].ORDER_MAX;
        }
        if (overlayNorder>_libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].ORDER_MAX) {
            overlayNorder = _libs_healpix_js__WEBPACK_IMPORTED_MODULE_8__["HealpixIndex"].ORDER_MAX;
        }
            
        this.curNorder = norder;
        this.curOverlayNorder = overlayNorder;
    };
    
    View.prototype.untaintCanvases = function() {
        this.createCanvases();
        createListeners(this);
        this.fixLayoutDimensions();
    };
    
    View.prototype.setOverlayImageSurvey = async function(idOrUrl) {
        /*if (! overlayImageSurvey) {
            this.overlayImageSurvey = null;
            this.requestRedraw();
            return;
        }
        
        // reset canvas to "untaint" canvas if needed
        // we test if the previous base image layer was using CORS or not
        if ($.support.cors && this.overlayImageSurvey && ! this.overlayImageSurvey.useCors) {
            this.untaintCanvases();
        }
        
        var newOverlayImageSurvey;
        if (typeof overlayImageSurvey == "string") {
            newOverlayImageSurvey = HpxImageSurvey.getSurveyFromId(overlayImageSurvey);
            if ( ! newOverlayImageSurvey) {
                newOverlayImageSurvey = HpxImageSurvey.getSurveyFromId(HpxImageSurvey.DEFAULT_SURVEY_ID);
            }
        }
        else {
            newOverlayImageSurvey = overlayImageSurvey;
        }
        newOverlayImageSurvey.isReady = false;
        this.overlayImageSurvey = newOverlayImageSurvey;
        
        var self = this;
        newOverlayImageSurvey.init(this, function() {
            //self.imageSurvey = newImageSurvey;
            self.computeNorder();
            newOverlayImageSurvey.isReady = true;
            self.requestRedraw();
            self.updateObjectsLookup();
            
            if (callback) {
                callback();
            }
        });*/
        if (!idOrUrl) {
            return;
        }

        let overlaySurvey = await new _HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_3__["HpxImageSurvey"](idOrUrl);
        this.aladin.webglAPI.setOverlayHiPS(overlaySurvey);
    };

    View.prototype.setUnknownSurveyIfNeeded = function() {
        if (unknownSurveyId) {
            this.setImageSurvey(unknownSurveyId);
            unknownSurveyId = undefined;
        }
    }

    /*View.prototype.addImageSurveyLayer = function(layer) {
        if (!(layer instanceof ImageSurveyLayer)) {
            throw "Except an ImageSurveyLayer object";
        }

        let surveys = [];
        for (let survey of layer.getSurveys()) {
            surveys.push(survey);
        }
        console.log("set layer: ", layer);
        this.aladin.webglAPI.addImageSurveyLayer(layer.name, surveys);
    };*/

    var unknownSurveyId = undefined;
    // @param imageSurvey : HpxImageSurvey object or image survey identifier
    View.prototype.addImageSurvey = function(survey, layer) {
        // We wait for the HpxImageSurvey to complete
        // Register to the view
        const url = survey.properties.url;
        survey.layer = layer;

        this.imageSurveys.get(layer).set(url, survey);
        // Then we send the current surveys to the backend
        this.setHiPS();
    };

    View.prototype.setImageSurvey = function(survey, layer) {
        const url = survey.properties.url;
        survey.layer = layer;
        
        this.imageSurveys.set(layer, new Map());
        this.imageSurveys.get(layer).set(url, survey);
        // Then we send the current surveys to the backend
        this.setHiPS();
    };

    View.prototype.setImageSurveysLayer = function(surveys, layer) {
        this.imageSurveys.set(layer, new Map());

        surveys.forEach(survey => {
            const url = survey.properties.url;
            survey.layer = layer;
            
            this.imageSurveys.get(layer).set(url, survey);
        });

        // Then we send the current surveys to the backend
        this.setHiPS();
    };

    View.prototype.removeImageSurveysLayer = function (layer) {
        this.imageSurveys.delete(layer);

        this.setHiPS();
    };

    View.prototype.moveImageSurveysLayerForward = function(layer) {
        this.aladin.webglAPI.moveImageSurveysLayerForward(layer);
    }

    View.prototype.setHiPS = function() {
        let surveys = [];
        for (let layer of this.imageSurveys.values()) {
            for (let survey of layer.values()) {
                surveys.push(survey);
            }
        }

        this.imageSurveysToSet = surveys;

        //this.aladin.webglAPI.setImageSurveys(surveys);
    };

    View.prototype.requestRedraw = function() {
        this.needRedraw = true;
    };
    
    View.prototype.changeProjection = function(projectionName) {
        switch (projectionName) {
            case "aitoff":
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].AITOFF;
                break;
            case "tan":
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].TAN;
                break;
            case "arc":
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].ARC;
                break;
            case "mercator":
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].MERCATOR;
                break;
            case "mollweide":
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].MOL;
                break;
            case "sinus":
            default:
                this.projectionMethod = _ProjectionEnum_js__WEBPACK_IMPORTED_MODULE_4__["ProjectionEnum"].SIN;
        }
        // Change the projection here
        this.projection.setProjection(this.projectionMethod);
        this.aladin.webglAPI.setProjection(projectionName);

        this.requestRedraw();
    };

    View.prototype.changeFrame = function(cooFrame) {
        var oldCooFrame = this.cooFrame;
        this.cooFrame = cooFrame;
        // recompute viewCenter
        console.log("change frame")
        if (this.cooFrame.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.GAL && this.cooFrame.system != oldCooFrame.system) {
            var lb = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].J2000ToGalactic([this.viewCenter.lon, this.viewCenter.lat]);
            this.viewCenter.lon = lb[0];
            this.viewCenter.lat = lb[1]; 

            const GAL = _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.GALCooSys();
            this.aladin.webglAPI.setCooSystem(GAL);
        }
        else if (this.cooFrame.system == _CooFrameEnum_js__WEBPACK_IMPORTED_MODULE_18__["CooFrameEnum"].SYSTEMS.J2000 && this.cooFrame.system != oldCooFrame.system) {
            var radec = _CooConversion_js__WEBPACK_IMPORTED_MODULE_19__["CooConversion"].GalacticToJ2000([this.viewCenter.lon, this.viewCenter.lat]);
            this.viewCenter.lon = radec[0];
            this.viewCenter.lat = radec[1];

            const ICRSJ2000 = _Aladin_js__WEBPACK_IMPORTED_MODULE_0__["Aladin"].wasmLibs.webgl.ICRSJ2000CooSys();
            this.aladin.webglAPI.setCooSystem(ICRSJ2000);
        }

        this.location.update(this.viewCenter.lon, this.viewCenter.lat, this.cooFrame, true);

        this.requestRedraw();
    };

    View.prototype.showHealpixGrid = function(show) {
        // Clear the grid ctx when not showing it
        if (!show) {
            this.gridCtx.clearRect(0, 0, this.imageCanvas.width, this.imageCanvas.height);
        }
        this.displayHpxGrid = show;
        this.requestRedraw();
    };
    
    View.prototype.showSurvey = function(show) {
        this.displaySurvey = show;

        this.requestRedraw();
    };
    
    View.prototype.showCatalog = function(show) {
        this.displayCatalog = show;

        if (!this.displayCatalog) {
            this.mustClearCatalog = true;
        }
        this.requestRedraw();
    };
    
    View.prototype.showReticle = function(show) {
        this.displayReticle = show;

        this.mustRedrawReticle = true;
        this.requestRedraw();
    };

    View.prototype.pointTo = function(ra, dec, options) {
        options = options || {};
        ra = parseFloat(ra);
        dec = parseFloat(dec);

        if (isNaN(ra) || isNaN(dec)) {
            return;
        }
        //if (this.cooFrame.system==CooFrameEnum.SYSTEMS.J2000) {
            this.viewCenter.lon = ra;
            this.viewCenter.lat = dec;
        //}
        /*else if (this.cooFrame.system==CooFrameEnum.SYSTEMS.GAL) {
            var lb = CooConversion.J2000ToGalactic([ra, dec]);
            this.viewCenter.lon = lb[0];
            this.viewCenter.lat = lb[1];
        }*/
        this.location.update(this.viewCenter.lon, this.viewCenter.lat, this.cooFrame, true);

        if (options && options.forceAnimation === false) {
            this.aladin.webglAPI.setCenter(this.viewCenter.lon, this.viewCenter.lat);
        } else if (options && options.forceAnimation === true) {
            this.aladin.webglAPI.moveToLocation(this.viewCenter.lon, this.viewCenter.lat)
        } else {
            if (this.fov > 30.0) {
                this.aladin.webglAPI.moveToLocation(this.viewCenter.lon, this.viewCenter.lat);
            } else {
                this.aladin.webglAPI.setCenter(this.viewCenter.lon, this.viewCenter.lat);
            }
        }
        
        this.forceRedraw();
        this.requestRedraw();

        var self = this;
        setTimeout(function() {self.refreshProgressiveCats();}, 1000);
    };
    View.prototype.makeUniqLayerName = function(name) {
        if (! this.layerNameExists(name)) {
            return name;
        }
        for (var k=1;;++k) {
            var newName = name + '_' + k;
            if ( ! this.layerNameExists(newName)) {
                return newName;
            }
        }
    };
    View.prototype.layerNameExists = function(name) {
        var c = this.allOverlayLayers;
        for (var k=0; k<c.length; k++) {
            if (name==c[k].name) {
                return true;
            }
        }
        return false;
    };

    View.prototype.removeLayers = function() {
        this.catalogs = [];
        this.overlays = [];
        this.mocs = [];
        this.allOverlayLayers = [];
        this.requestRedraw();
    };

    View.prototype.addCatalog = function(catalog) {
        catalog.name = this.makeUniqLayerName(catalog.name);
        this.allOverlayLayers.push(catalog);
        this.catalogs.push(catalog);
        if (catalog.type=='catalog') {
            catalog.setView(this);
        }
        else if (catalog.type=='progressivecat') {
            catalog.init(this);
        }
    };
    View.prototype.addOverlay = function(overlay) {
        overlay.name = this.makeUniqLayerName(overlay.name);
        this.overlays.push(overlay);
        this.allOverlayLayers.push(overlay);
        overlay.setView(this);
    };
    
    View.prototype.addMOC = function(moc) {
        moc.name = this.makeUniqLayerName(moc.name);
        this.mocs.push(moc);
        this.allOverlayLayers.push(moc);
        moc.setView(this);
    };
    
    View.prototype.getObjectsInBBox = function(x, y, w, h) {
        if (w<0) {
            x = x+w;
            w = -w;
        }
        if (h<0) {
            y = y+h;
            h = -h;
        }
        var objList = [];
        var cat, sources, s;
        if (this.catalogs) {
            for (var k=0; k<this.catalogs.length; k++) {
                cat = this.catalogs[k];
                if (!cat.isShowing) {
                    continue;
                }
                sources = cat.getSources();
                for (var l=0; l<sources.length; l++) {
                    s = sources[l];
                    if (!s.isShowing || !s.x || !s.y) {
                        continue;
                    }
                    if (s.x>=x && s.x<=x+w && s.y>=y && s.y<=y+h) {
                        objList.push(s);
                    }
                }
            }
        }
        return objList;
        
    };

    // update objLookup, lookup table 
    View.prototype.updateObjectsLookup = function() {
        this.objLookup = [];

        var cat, sources, s, xRounded, yRounded;
        if (this.catalogs) {
            for (var k=0; k<this.catalogs.length; k++) {
                cat = this.catalogs[k];
                if (!cat.isShowing) {
                    continue;
                }
                sources = cat.getSources();
                for (var l=0; l<sources.length; l++) {
                    s = sources[l];
                    if (!s.isShowing || !s.x || !s.y) {
                        continue;
                    }

                    xRounded = Math.round(s.x);
                    yRounded = Math.round(s.y);

                    if (typeof this.objLookup[xRounded] === 'undefined') {
                        this.objLookup[xRounded] = [];
                    }
                    if (typeof this.objLookup[xRounded][yRounded] === 'undefined') {
                        this.objLookup[xRounded][yRounded] = [];
                    }
                    this.objLookup[xRounded][yRounded].push(s);
                }       
            }           
        }     
    };

    // return closest object within a radius of maxRadius pixels. maxRadius is an integer
    View.prototype.closestObjects = function(x, y, maxRadius) {

        // footprint selection code adapted from Fabrizio Giordano dev. from Serco for ESA/ESDC
        var overlay;
        var canvas=this.catalogCanvas;
        var ctx = canvas.getContext("2d");
        // this makes footprint selection easier as the catch-zone is larger
        ctx.lineWidth = 6;

        if (this.overlays) {
            for (var k=0; k<this.overlays.length; k++) {
                overlay = this.overlays[k];
                for (var i=0; i<overlay.overlays.length;i++){

                    // test polygons first
                    var footprint = overlay.overlays[i];
                    var pointXY = [];
                    for(var j=0;j<footprint.polygons.length;j++){

                        /*var xy = AladinUtils.radecToViewXy(footprint.polygons[j][0], footprint.polygons[j][1],
                                this.projection,
                                this.cooFrame,
                                this.width, this.height,
                                this.largestDim,
                                this.zoomFactor);*/
                        var xy = _AladinUtils_js__WEBPACK_IMPORTED_MODULE_7__["AladinUtils"].radecToViewXy(footprint.polygons[j][0], footprint.polygons[j][1], this);
                        if (! xy) {
                            continue;
                        }
                        pointXY.push({
                            x: xy[0],
                            y: xy[1]
                        });
                    }
                    for(var l=0; l<pointXY.length-1;l++){

                        ctx.beginPath();                        // new segment
                        ctx.moveTo(pointXY[l].x, pointXY[l].y);     // start is current point
                        ctx.lineTo(pointXY[l+1].x, pointXY[l+1].y); // end point is next
                        if (ctx.isPointInStroke(x, y)) {        // x,y is on line?
                            closest = footprint;
                            return [closest];
                        }
                    }
                }

                // test Circles
                for (var i=0; i<overlay.overlay_items.length; i++) {
                    if (overlay.overlay_items[i] instanceof _Circle_js__WEBPACK_IMPORTED_MODULE_17__["Circle"]) {
                        overlay.overlay_items[i].draw(ctx, this, this.projection, this.cooFrame, this.width, this.height, this.largestDim, this.zoomFactor, true);

                        if (ctx.isPointInStroke(x, y)) {
                            closest = overlay.overlay_items[i];
                            return [closest];
                        }
                    }
                }
            }
        }






        if (!this.objLookup) {
            return null;
        }
        var closest, dist;
        for (var r=0; r<=maxRadius; r++) {
            closest = dist = null;
            for (var dx=-maxRadius; dx<=maxRadius; dx++) {
                if (! this.objLookup[x+dx]) {
                    continue;
                }
                for (var dy=-maxRadius; dy<=maxRadius; dy++) {
                    if (this.objLookup[x+dx][y+dy]) {
                        var d = dx*dx + dy*dy;
                        if (!closest) {
                            closest = this.objLookup[x+dx][y+dy];
                            dist = d;
                        }
                        else if (d<dist) {
                            dist = d;
                            closest = this.objLookup[x+dx][y+dy];
                        }
                    }
                }
            }
            if (closest) {
                return closest;
            }
        }
        return null;
    };
    
    return View;
})();


/***/ }),

/***/ "./src/js/libs/RequestAnimationFrame.js":
/*!**********************************************!*\
  !*** ./src/js/libs/RequestAnimationFrame.js ***!
  \**********************************************/
/*! exports provided: requestAnimFrame */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "requestAnimFrame", function() { return requestAnimFrame; });
// requestAnimationFrame() shim by Paul Irish
// http://paulirish.com/2011/requestanimationframe-for-smart-animating/
/*export let requestAnimFrame = (function() {
	return  window.requestAnimationFrame       || 
			window.webkitRequestAnimationFrame || 
			window.mozRequestAnimationFrame    || 
			window.oRequestAnimationFrame      || 
			window.msRequestAnimationFrame     || 
			function( callback, element){
				window.setTimeout(callback, 1000 / 60);
			};
})();
*/

let requestAnimFrame = (function() {
	return  window.requestAnimationFrame       || 
			window.webkitRequestAnimationFrame || 
			window.mozRequestAnimationFrame    || 
			window.oRequestAnimationFrame      || 
			window.msRequestAnimationFrame;
})();


/***/ }),

/***/ "./src/js/libs/Stats.js":
/*!******************************!*\
  !*** ./src/js/libs/Stats.js ***!
  \******************************/
/*! exports provided: Stats */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Stats", function() { return Stats; });
// stats.js r6 - http://github.com/mrdoob/stats.js
let Stats=function(){function s(a,g,d){var f,c,e;for(c=0;c<30;c++)for(f=0;f<73;f++)e=(f+c*74)*4,a[e]=a[e+4],a[e+1]=a[e+5],a[e+2]=a[e+6];for(c=0;c<30;c++)e=(73+c*74)*4,c<g?(a[e]=b[d].bg.r,a[e+1]=b[d].bg.g,a[e+2]=b[d].bg.b):(a[e]=b[d].fg.r,a[e+1]=b[d].fg.g,a[e+2]=b[d].fg.b)}var r=0,t=2,g,u=0,j=(new Date).getTime(),F=j,v=j,l=0,w=1E3,x=0,k,d,a,m,y,n=0,z=1E3,A=0,f,c,o,B,p=0,C=1E3,D=0,h,i,q,E,b={fps:{bg:{r:16,g:16,b:48},fg:{r:0,g:255,b:255}},ms:{bg:{r:16,g:48,b:16},fg:{r:0,g:255,b:0}},mb:{bg:{r:48,g:16,
b:26},fg:{r:255,g:0,b:128}}};g=document.createElement("div");g.style.cursor="pointer";g.style.width="80px";g.style.opacity="0.9";g.style.zIndex="10001";g.addEventListener("click",function(){r++;r==t&&(r=0);k.style.display="none";f.style.display="none";h.style.display="none";switch(r){case 0:k.style.display="block";break;case 1:f.style.display="block";break;case 2:h.style.display="block"}},!1);k=document.createElement("div");k.style.backgroundColor="rgb("+Math.floor(b.fps.bg.r/2)+","+Math.floor(b.fps.bg.g/
2)+","+Math.floor(b.fps.bg.b/2)+")";k.style.padding="2px 0px 3px 0px";g.appendChild(k);d=document.createElement("div");d.style.fontFamily="Helvetica, Arial, sans-serif";d.style.textAlign="left";d.style.fontSize="9px";d.style.color="rgb("+b.fps.fg.r+","+b.fps.fg.g+","+b.fps.fg.b+")";d.style.margin="0px 0px 1px 3px";d.innerHTML='<span style="font-weight:bold">FPS</span>';k.appendChild(d);a=document.createElement("canvas");a.width=74;a.height=30;a.style.display="block";a.style.marginLeft="3px";k.appendChild(a);
m=a.getContext("2d");m.fillStyle="rgb("+b.fps.bg.r+","+b.fps.bg.g+","+b.fps.bg.b+")";m.fillRect(0,0,a.width,a.height);y=m.getImageData(0,0,a.width,a.height);f=document.createElement("div");f.style.backgroundColor="rgb("+Math.floor(b.ms.bg.r/2)+","+Math.floor(b.ms.bg.g/2)+","+Math.floor(b.ms.bg.b/2)+")";f.style.padding="2px 0px 3px 0px";f.style.display="none";g.appendChild(f);c=document.createElement("div");c.style.fontFamily="Helvetica, Arial, sans-serif";c.style.textAlign="left";c.style.fontSize=
"9px";c.style.color="rgb("+b.ms.fg.r+","+b.ms.fg.g+","+b.ms.fg.b+")";c.style.margin="0px 0px 1px 3px";c.innerHTML='<span style="font-weight:bold">MS</span>';f.appendChild(c);a=document.createElement("canvas");a.width=74;a.height=30;a.style.display="block";a.style.marginLeft="3px";f.appendChild(a);o=a.getContext("2d");o.fillStyle="rgb("+b.ms.bg.r+","+b.ms.bg.g+","+b.ms.bg.b+")";o.fillRect(0,0,a.width,a.height);B=o.getImageData(0,0,a.width,a.height);try{performance&&performance.memory&&performance.memory.totalJSHeapSize&&
(t=3)}catch(G){}h=document.createElement("div");h.style.backgroundColor="rgb("+Math.floor(b.mb.bg.r/2)+","+Math.floor(b.mb.bg.g/2)+","+Math.floor(b.mb.bg.b/2)+")";h.style.padding="2px 0px 3px 0px";h.style.display="none";g.appendChild(h);i=document.createElement("div");i.style.fontFamily="Helvetica, Arial, sans-serif";i.style.textAlign="left";i.style.fontSize="9px";i.style.color="rgb("+b.mb.fg.r+","+b.mb.fg.g+","+b.mb.fg.b+")";i.style.margin="0px 0px 1px 3px";i.innerHTML='<span style="font-weight:bold">MB</span>';
h.appendChild(i);a=document.createElement("canvas");a.width=74;a.height=30;a.style.display="block";a.style.marginLeft="3px";h.appendChild(a);q=a.getContext("2d");q.fillStyle="#301010";q.fillRect(0,0,a.width,a.height);E=q.getImageData(0,0,a.width,a.height);return{domElement:g,update:function(){u++;j=(new Date).getTime();n=j-F;z=Math.min(z,n);A=Math.max(A,n);s(B.data,Math.min(30,30-n/200*30),"ms");c.innerHTML='<span style="font-weight:bold">'+n+" MS</span> ("+z+"-"+A+")";o.putImageData(B,0,0);F=j;if(j>
v+1E3){l=Math.round(u*1E3/(j-v));w=Math.min(w,l);x=Math.max(x,l);s(y.data,Math.min(30,30-l/100*30),"fps");d.innerHTML='<span style="font-weight:bold">'+l+" FPS</span> ("+w+"-"+x+")";m.putImageData(y,0,0);if(t==3)p=performance.memory.usedJSHeapSize*9.54E-7,C=Math.min(C,p),D=Math.max(D,p),s(E.data,Math.min(30,30-p/2),"mb"),i.innerHTML='<span style="font-weight:bold">'+Math.round(p)+" MB</span> ("+Math.round(C)+"-"+Math.round(D)+")",q.putImageData(E,0,0);v=j;u=0}}}};



/***/ }),

/***/ "./src/js/libs/astro/astroMath.js":
/*!****************************************!*\
  !*** ./src/js/libs/astro/astroMath.js ***!
  \****************************************/
/*! exports provided: AstroMath */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "AstroMath", function() { return AstroMath; });
//=================================
//            AstroMath
//=================================

// Class AstroMath having 'static' methods
let AstroMath = function() {};

// Constant for conversion Degrees => Radians (rad = deg*AstroMath.D2R)
AstroMath.D2R = Math.PI/180.0;
// Constant for conversion Radians => Degrees (deg = rad*AstroMath.R2D)
AstroMath.R2D = 180.0/Math.PI;
/**
 * Function sign
 * @param x value for checking the sign
 * @return -1, 0, +1 respectively if x < 0, = 0, > 0
 */
AstroMath.sign = function(x) { return x > 0 ? 1 : (x < 0 ? -1 : 0 ); };

/**
 * Function cosd(degrees)
 * @param x angle in degrees
 * @returns the cosine of the angle
 */
AstroMath.cosd = function(x) {
	if (x % 90 == 0) {
		var i = Math.abs(Math.floor(x / 90 + 0.5)) % 4;
		switch (i) {
			case 0:	return 1;
			case 1:	return 0;
			case 2:	return -1;
			case 3:	return 0;
		}
	}
	return Math.cos(x*AstroMath.D2R);
};

/**
 * Function sind(degrees)
 * @param x angle in degrees
 * @returns the sine of the angle
 */
AstroMath.sind = function(x) {
	if (x % 90 === 0) {
		var i = Math.abs(Math.floor(x / 90 - 0.5)) % 4;
		switch (i) {
			case 0:	return 1;
			case 1:	return 0;
			case 2:	return -1;
			case 3:	return 0;
		}
	}

	return Math.sin(x*AstroMath.D2R);
};

/**
 * Function tand(degrees)
 * @param x angle in degrees
 * @returns the tangent of the angle
 */
AstroMath.tand = function(x) {
	var resid;

	resid = x % 360;
	if (resid == 0 || Math.abs(resid) == 180) {
		return 0;
	} else if (resid == 45 || resid == 225) {
		return 1;
	} else if (resid == -135 || resid == -315) {
		return -1
	}

	return Math.tan(x * AstroMath.D2R);
};

/**
 * Function asin(degrees)
 * @param sine value [0,1]
 * @return the angle in degrees
 */
AstroMath.asind = function(x) { return Math.asin(x)*AstroMath.R2D; };

/**
 * Function acos(degrees)
 * @param cosine value [0,1]
 * @return the angle in degrees
 */
AstroMath.acosd = function(x) { return Math.acos(x)*AstroMath.R2D; };

/**
 * Function atan(degrees)
 * @param tangent value
 * @return the angle in degrees
 */
AstroMath.atand = function(x) { return Math.atan(x)*AstroMath.R2D; };

/**
 * Function atan2(y,x)
 * @param y y component of the vector
 * @param x x component of the vector
 * @return the angle in radians
 */
AstroMath.atan2 = function(y,x) {
	if (y != 0.0) {
		var sgny = AstroMath.sign(y);
		if (x != 0.0) {
			var phi = Math.atan(Math.abs(y/x));
			if (x > 0.0) return phi*sgny;
			else if (x < 0) return (Math.PI-phi)*sgny;
		} else return (Math.PI/2)*sgny;
	} else {
		return x > 0.0 ? 0.0 : (x < 0 ? Math.PI : 0.0/0.0);
	}
}  

/**
 * Function atan2d(y,x)
 * @param y y component of the vector
 * @param x x component of the vector
 * @return the angle in degrees
 */
AstroMath.atan2d = function(y,x) {
	return AstroMath.atan2(y,x)*AstroMath.R2D;
}

/*=========================================================================*/
/**
 * Computation of hyperbolic cosine
 * @param x argument
 */
AstroMath.cosh = function(x) {
	return (Math.exp(x)+Math.exp(-x))/2;
}

/**
 * Computation of hyperbolic sine
 * @param x argument
 */
AstroMath.sinh = function(x) {
	return (Math.exp(x)-Math.exp(-x))/2;
}

/**
 * Computation of hyperbolic tangent
 * @param x argument
 */
AstroMath.tanh = function(x) {
	return (Math.exp(x)-Math.exp(-x))/(Math.exp(x)+Math.exp(-x));
}

/**
 * Computation of Arg cosh
 * @param x argument in degrees. Must be in the range [ 1, +infinity ]
 */
AstroMath.acosh = function(x) {
	return(Math.log(x+Math.sqrt(x*x-1.0)));
}

/**
 * Computation of Arg sinh
 * @param x argument in degrees
 */
AstroMath.asinh = function(x) {
	return(Math.log(x+Math.sqrt(x*x+1.0)));
}

/**
 * Computation of Arg tanh
 * @param x argument in degrees. Must be in the range ] -1, +1 [
 */
AstroMath.atanh = function(x) {
	return(0.5*Math.log((1.0+x)/(1.0-x)));
}

//=============================================================================
//      Special Functions using trigonometry
//=============================================================================
/**
 * Computation of sin(x)/x
 *	@param x in degrees.
 * For small arguments x <= 0.001, use approximation 
 */
AstroMath.sinc = function(x) {
	var ax = Math.abs(x);
	var y;

	if (ax <= 0.001) {
		ax *= ax;
		y = 1 - ax*(1.0-ax/20.0)/6.0;
	} else {
		y = Math.sin(ax)/ax;
	}

	return y;
}

/**
 * Computes asin(x)/x
 * @param x in degrees.
 * For small arguments x <= 0.001, use an approximation
 */
AstroMath.asinc = function(x) {
	var ax = Math.abs(x);
	var y;

	if (ax <= 0.001) {
		ax *= ax; 
		y = 1 + ax*(6.0 + ax*(9.0/20.0))/6.0;
	} else {
		y = Math.asin(ax)/ax;	// ???? radians ???
	}

	return (y);
}


//=============================================================================
/**
 * Computes the hypotenuse of x and y
 * @param x value
 * @param y value
 * @return sqrt(x*x+y*y)
 */
AstroMath.hypot = function(x,y) {
	return Math.sqrt(x*x+y*y);
}

/** Generate the rotation matrix from the Euler angles
 * @param z	Euler angle
 * @param theta	Euler angle
 * @param zeta	Euler angles
 * @return R [3][3]		the rotation matrix
 * The rotation matrix is defined by:<pre>
 *    R =      R_z(-z)      *        R_y(theta)     *     R_z(-zeta)
 *   |cos.z -sin.z  0|   |cos.the  0 -sin.the|   |cos.zet -sin.zet 0|
 * = |sin.z  cos.z  0| x |   0     1     0   | x |sin.zet  cos.zet 0|
 *   |   0      0   1|   |sin.the  0  cos.the|   |   0        0    1|
 * </pre>
 */
AstroMath.eulerMatrix = function(z, theta, zeta) {
	var R = new Array(3);
	R[0] = new Array(3);
	R[1] = new Array(3);
	R[2] = new Array(3);
	var cosdZ = AstroMath.cosd(z);
	var sindZ = AstroMath.sind(z);
	var cosdTheta = AstroMath.cosd(theta);
	var w = AstroMath.sind(theta) ;
	var cosdZeta = AstroMath.cosd(zeta);
	var sindZeta = AstroMath.sind(zeta);

	R[0][0] = cosdZeta*cosdTheta*cosdZ - sindZeta*sindZ;
	R[0][1] = -sindZeta*cosdTheta*cosdZ - cosdZeta*sindZ;
	R[0][2] = -w*cosdZ;

	R[1][0] = cosdZeta*cosdTheta*sindZ + sindZeta*cosdZ;
	R[1][1] = -sindZeta*cosdTheta*sindZ + cosdZeta*cosdZ;
	R[1][2] = -w*sindZ;

	R[2][0] = -w*cosdZeta;
	R[2][1] = -w*cosdZ;
	R[2][2] = cosdTheta;
	return R ;
};


AstroMath.displayMatrix = function(m) {
	// Number of rows
	var nbrows = m.length;
	// Max column count
	var nbcols = 0
	for (var i=0; i<nbrows; i++) {
		if (m[i].length > nbcols) nbcols = m[i].length;
	}
	var str = '<table>\n';
	for (var i=0; i<nbrows; i++) {
		str += '<tr>';
		for (var j=0; j<nbrows; j++) {
			str += '<td>';
			if (i < m[i].length)
				str += (m[i][j]).toString();
			str += '</td>';
		}
		str += '</td>\n';
	}
	str += '</table>\n';

	return str;
}


/***/ }),

/***/ "./src/js/libs/astro/coo.js":
/*!**********************************!*\
  !*** ./src/js/libs/astro/coo.js ***!
  \**********************************/
/*! exports provided: Coo */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Coo", function() { return Coo; });
/* harmony import */ var _astroMath_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./astroMath.js */ "./src/js/libs/astro/astroMath.js");
//=================================
// Class Coo
//=================================



/**
 * Constructor
 * @param longitude longitude (decimal degrees)
 * @param latitude latitude (decimal degrees)
 * @param prec precision
 * (8: 1/1000th sec, 7: 1/100th sec, 6: 1/10th sec, 5: sec, 4: 1/10th min, 3: min, 2: 1/10th deg, 1: deg
 */
let Coo = function(longitude, latitude, prec) {
	this.lon = longitude;
	this.lat = latitude;
	this.prec = prec;
	this.frame = null;

	this.computeDirCos();
};

Coo.factor = [ 3600.0, 60.0, 1.0 ];
Coo.prototype = {
	setFrame: function(astroframe) {
		this.frame = astroframe;
	},
	computeDirCos: function() {
		var coslat = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(this.lat);

		this.x = coslat*_astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(this.lon);
		this.y = coslat*_astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(this.lon);
		this.z = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(this.lat);	
	}, 
	computeLonLat: function() {
		var r2 = this.x*this.x+this.y*this.y;
		this.lon = 0.0;
		if (r2 == 0.0) {
			// In case of poles
			if (this.z == 0.0) {
				this.lon = 0.0/0.0;
				this.lat = 0.0/0.0;
			} else {
				this.lat = (this.z > 0.0) ? 90.0 : -90.0;
			}
		} else {
			this.lon = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].atan2d(this.y, this.x);
			this.lat = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].atan2d(this.z, Math.sqrt(r2));
			if (this.lon < 0) this.lon += 360.0;
		}
	},

  /**
    * Squared distance between 2 points (= 4.sin<sup>2</sup>(r/2))
    * @param  pos      another position on the sphere
    * @return ||pos-this||<sup>2</sup> = 4.sin<sup>2</sup>(r/2)
   **/
   dist2: function(pos) {
//    	if ((this.x==0)&&(this.y==0)&&(this.z==0)) return(0./0.);
//    	if ((pos.x==0)&&(pos.y==0)&&(pos.z==0)) return(0./0.);
	var w = pos.x - this.x;
	var r2 = w * w;
	w = pos.y - this.y; r2 += w * w;
	w = pos.z - this.z; r2 += w * w;
	return r2;
   },

   /**
    * Distance between 2 points on the sphere.
    * @param  pos another position on the sphere
    * @return distance in degrees in range [0, 180]
   **/
    distance: function(pos) {
      // Take care of NaN:
    	if ((pos.x==0)&&(pos.y==0)&&(pos.z==0)) return(0./0.);
    	if ((this.x==0)&&(this.y==0)&&(this.z==0)) return(0./0.);
      return (2. * _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].asind(0.5 * Math.sqrt(this.dist2(pos))));
    },

   /**
    * Transform the position into another frame.
    * @param new_frame	The frame of the resulting position.
   **/
   convertTo: function(new_frame) {
		// Verify first if frames identical -- then nothing to do !
		if (this.frame.equals(new_frame)) {
	    		return;
		}

		// Move via ICRS
		this.frame.toICRS(this.coo);	// Position now in ICRS
		new_frame.fromICRS(this.coo);	// Position now in new_frame
		this.frame = new_frame;
		this.lon = this.lat = 0./0.;	// Actual angles not recomputed
   },

    /**
     * Rotate a coordinate (apply a rotation to the position).
     * @param R [3][3] Rotation Matrix
     */
    rotate: function(R) {
      var X, Y, Z;
		if (R == Umatrix3) return;
		X = R[0][0]*this.x + R[0][1]*this.y + R[0][2]*this.z;
		Y = R[1][0]*this.x + R[1][1]*this.y + R[1][2]*this.z;
		Z = R[2][0]*this.x + R[2][1]*this.y + R[2][2]*this.z;
    	// this.set(X, Y, Z); Not necessary to compute positions each time.
		this.x = X; this.y = Y; this.z = Z;
		this.lon = this.lat = 0./0.;
    },

    /**
     * Rotate a coordinate (apply a rotation to the position) in reverse direction.
     * The method is the inverse of rotate.
     * @param R [3][3] Rotation Matrix
     */
    rotate_1: function(R) {
      var X, Y, Z;
      if (R == Umatrix3) return;
		X = R[0][0]*this.x + R[1][0]*this.y + R[2][0]*this.z;
		Y = R[0][1]*this.x + R[1][1]*this.y + R[2][1]*this.z;
		Z = R[0][2]*this.x + R[1][2]*this.y + R[2][2]*this.z;
    	// this.set(X, Y, Z); Not necessary to compute positions each time.
		this.x = X; this.y = Y; this.z = Z;
		this.lon = this.lat = 0./0.;
    },


    /**
     * Test equality of Coo.
     * @param coo Second coordinate to compare with
     * @return  True if the two coordinates are equal
     */
    equals: function(coo) {
		return this.x == coo.x && this.y == coo.y && this.z == coo.z;
    },

	/**
	 * parse a coordinate string. The coordinates can be in decimal or sexagesimal
	 * @param str string to parse
	 * @return true if the parsing succeded, false otherwise
	 */
	parse: function(str) {
		var p = str.indexOf('+');
		if (p < 0) p = str.indexOf('-');
		if (p < 0) p = str.indexOf(' ');
		if (p < 0) {
			this.lon = 0.0/0.0;
			this.lat = 0.0/0.0;
			this.prec = 0;
			return false;
		}
		var strlon = str.substring(0,p);
		var strlat = str.substring(p);
	
		this.lon = this.parseLon(strlon);	// sets the precision parameter
		this.lat = this.parseLat(strlat);	// sets the precision parameter
		return true;
	},

	parseLon: function(str) {
		var str = str.trim();
        str = str.replace(/:/g, ' ');

		if (str.indexOf(' ') < 0) {
			// The longitude is a integer or decimal number
			var p = str.indexOf('.');
			this.prec = p < 0 ? 0 : str.length - p - 1;
			return parseFloat(str);
		} else {
			var stok = new Tokenizer(str,' ');
			var i = 0;
			var l = 0;
			var pr = 0;
			while (stok.hasMore()) {
				var tok = stok.nextToken();
				var dec = tok.indexOf('.');
				l += parseFloat(tok)*Coo.factor[i];
//				pr = dec < 0 ? 1 : 2;
				switch (i) {
					case 0: pr = dec < 0 ? 1 : 2; break;
					case 1: pr = dec < 0 ? 3 : 4; break;
					case 2: pr = dec < 0 ? 5 : 4+tok.length-dec;
					default: break;
				}
				i++;
			}
			this.prec = pr;
			return l*15/3600.0;	
		}
	},
			
	parseLat: function(str) {
		var str = str.trim();
        str = str.replace(/:/g, ' ');

		var sign;
		if (str.charAt(0) == '-') {
			sign = -1;
			str = str.substring(1);
		} else if (str.charAt(0) == '-') {
			sign = 1;
			str = str.substring(1);
		} else {
			// No sign specified
			sign = 1;
		}
		if (str.indexOf(' ') < 0) {
			// The longitude is a integer or decimal number
			var p = str.indexOf('.');
			this.prec = p < 0 ? 0 : str.length - p - 1;
			return parseFloat(str)*sign;
		} else {
			var stok = new Tokenizer(str,' ');
			var i = 0;
			var l = 0;
			var pr = 0;
			while (stok.hasMore()) {
				var tok = stok.nextToken();
				var dec = tok.indexOf('.');
				l += parseFloat(tok)*Coo.factor[i];
				switch (i) {
					case 0: pr = dec < 0 ? 1 : 2; break;
					case 1: pr = dec < 0 ? 3 : 4; break;
					case 2: pr = dec < 0 ? 5 : 4+tok.length-dec;
					default: break;
				}
				i++;
			}
			this.prec = pr;
			return l*sign/3600.0;	
		}
	},

	/**
	 * Format coordinates according to the options
	 * @param options 'd': decimal, 's': sexagésimal, '/': space separated, '2': return [ra,dec] in an array
	 * @return the formatted coordinates
	 */
	format: function(options) {
		if (isNaN(this.lon)) this.computeLonLat();
		var strlon = "", strlat = "";
		if (options.indexOf('d') >= 0) {
			// decimal display
			strlon = Numbers.format(this.lon, this.prec);
			strlat = Numbers.format(this.lat, this.prec);
		} else {
			// sexagesimal display
			var hlon = this.lon/15.0;
			var strlon = Numbers.toSexagesimal(hlon, this.prec+1, false);
			var strlat = Numbers.toSexagesimal(this.lat, this.prec, false);
		}
		if (this.lat > 0) strlat = '+'+strlat;

		if (options.indexOf('/') >= 0) {
			return strlon+' '+strlat;
		} else if (options.indexOf('2') >= 0) {
			return [strlon, strlat];
		}
		return strlon+strlat;
	}
		
}

/**
 * Distance between 2 points on the sphere.
 * @param coo1 firs	var coslat = AstroMath.cosd(this.lat);

	this.x = coslat*AstroMath.cosd(this.lon);
	this.y = coslat*AstroMath.sind(this.lon);
	this.z = AstroMath.sind(this.lat);
t coordinates point
 * @param coo2 second coordinates point
 * @return distance in degrees in range [0, 180]
**/
/*
Coo.distance = function(Coo coo1, Coo coo2) {
	return Coo.distance(coo1.lon, coo1.lat, coo2.lon, coo2.lat);
}
*/
/**
 * Distance between 2 points on the sphere.
 * @param lon1 longitude of first point in degrees
 * @param lat1 latitude of first point in degrees
 * @param lon2 longitude of second point in degrees
 * @param lat2 latitude of second point in degrees
 * @return distance in degrees in range [0, 180]
**/
/*
Coo.distance = function(lon1, lat1, lon2, lat2) {
	var c1 = AstroMath.cosd(lat1);
	var c2 = AstroMath.cosd(lat2);

	var w, r2;
	w = c1 * AstroMath.cosd(lon1) - c2 * AstroMath.cosd(lon2);
	r2 = w*w;
	w = c1 * AstroMath.sind(lon1) - c2 * AstroMath.sind(lon2);
	r2 += w*w;
	w = AstroMath.sind(lat1) - AstroMath.sind(lat2);
	r2 += w*w;

	return 2. * AstroMath.asind(0.5 * Math.sqrt(r2));
}


//===================================
// Class Tokenizer (similar to Java)
//===================================

/**
 * Constructor
 * @param str String to tokenize
 * @param sep token separator char
 */
function Tokenizer(str, sep) {
	this.string = Strings.trim(str, sep);
	this.sep = sep;
	this.pos = 0;
}

Tokenizer.prototype = {
	/**
	 * Check if the string has more tokens
	 * @return true if a token remains (read with nextToken())
	 */
	hasMore: function() {
		return this.pos < this.string.length;
	},

	/**
	 * Returns the next token (as long as hasMore() is true)
	 * @return the token string
	 */
	nextToken: function() {
		// skip all the separator chars
		var p0 = this.pos;
		while (p0 < this.string.length && this.string.charAt(p0) == this.sep) p0++;
		var p1 = p0;
		// get the token
		while (p1 < this.string.length && this.string.charAt(p1) != this.sep) p1++;
		this.pos = p1;
		return this.string.substring(p0, p1);
	},
}

//================================
// Class Strings (static methods)
//================================
function Strings() {}

/**
 * Removes a given char at the beginning and the end of a string
 * @param str string to trim
 * @param c char to remove
 * @return the trimmed string
 */

Strings.trim = function(str, c) {
	var p0=0, p1=str.length-1;
	while (p0 < str.length && str.charAt(p0) == c) p0++;
	if (p0 == str.length) return "";
	while (p1 > p0 && str.charAt(p1) == c) p1--;
	return str.substring(p0, p1+1);
}

//================================
// Class Numbers (static methods)
//================================
function Numbers() {}
//                0  1   2    3     4      5       6        7         8          9
Numbers.pow10 = [ 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
//      10           11            12             13              14
	10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000 ];
//                 0    1     2      3       4        5         6          7
Numbers.rndval = [ 0.5, 0.05, 0.005, 0.0005, 0.00005, 0.000005, 0.0000005, 0.00000005,
//      8            9             10             11              12
	0.000000005, 0.0000000005, 0.00000000005, 0.000000000005, 0.0000000000005,
//      13                14
	0.00000000000005, 0.00000000000005 ];
/**
 * Format a integer or decimal number, adjusting the value with 'prec' decimal digits
 * @param num number (integer or decimal)
 * @param prec precision (= number of decimal digit to keep or append)
 * @return a string with the formatted number
 */
Numbers.format = function(num, prec) {
		if (prec <= 0) {
			// Return an integer number
			return (Math.round(num)).toString();
		}
		var str = num.toString();
		var p = str.indexOf('.');
		var nbdec = p >= 0 ? str.length-p-1 : 0;
		if (prec >= nbdec) {
			if (p < 0) str += '.';
			for (var i=0; i<prec-nbdec; i++)
				str += '0';
			return str;
		}
		// HERE: prec > 0 and prec < nbdec
		str = (num+Numbers.rndval[prec]).toString();
		return str.substr(0, p+prec+1);
}


/**
 * Convert a decimal coordinate into sexagesimal string, according to the given precision<br>
 * 8: 1/1000th sec, 7: 1/100th sec, 6: 1/10th sec, 5: sec, 4: 1/10th min, 3: min, 2: 1/10th deg, 1: deg
 * @param num number (integer or decimal)
 * @param prec precision (= number of decimal digit to keep or append)
 * @param plus if true, the '+' sign is displayed
 * @return a string with the formatted sexagesimal number
 */
Numbers.toSexagesimal = function(num, prec, plus) {
	var resu = "";
	var sign = num < 0 ? '-' : (plus ? '+' : '');
	var n = Math.abs(num);

	switch (prec) {
		case 1:	// deg
			var n1 = Math.round(n);
			return sign+n1.toString();
		case 2:	// deg.d
			return sign+Numbers.format(n, 1);
		case 3:	// deg min
			var n1 = Math.floor(n);
			var n2 = Math.round((n-n1)*60);
			return sign+n1+' '+n2;
		case 4:	// deg min.d
			var n1 = Math.floor(n);
			var n2 = (n-n1)*60;
			return sign+n1+' '+Numbers.format(n2, 1);
		case 5:	// deg min sec
			var n1 = Math.floor(n);	// d
			var n2 = (n-n1)*60;		// M.d
			var n3 = Math.floor(n2);// M
			var n4 = Math.round((n2-n3)*60);	// S
			return sign+n1+' '+n3+' '+n4;
		case 6:	// deg min sec.d
		case 7:	// deg min sec.dd
		case 8:	// deg min sec.ddd
			var n1 = Math.floor(n);	// d
			if (n1<10) n1 = '0' + n1;
			var n2 = (n-n1)*60;		// M.d
			var n3 = Math.floor(n2);// M
			if (n3<10) n3 = '0' + n3;
			var n4 = (n2-n3)*60;		// S.ddd
			return sign+n1+' '+n3+' '+Numbers.format(n4, prec-5);
		default:
			return sign+Numbers.format(n, 1);
	}
}


/***/ }),

/***/ "./src/js/libs/astro/projection.js":
/*!*****************************************!*\
  !*** ./src/js/libs/astro/projection.js ***!
  \*****************************************/
/*! exports provided: Projection */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Projection", function() { return Projection; });
/* harmony import */ var _astroMath_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./astroMath.js */ "./src/js/libs/astro/astroMath.js");



let Projection = function(lon0, lat0) {
	this.PROJECTION = Projection.PROJ_TAN;
	this.ROT = this.tr_oR(lon0, lat0);

    this.longitudeIsReversed = false;
}

//var ROT;
//var PROJECTION = Projection.PROJ_TAN;	// Default projection


Projection.PROJ_TAN = 1;	/* Gnomonic projection*/
Projection.PROJ_TAN2 = 2;	/* Stereographic projection*/
Projection.PROJ_STG = 2;	
Projection.PROJ_SIN = 3;	/* Orthographic		*/
Projection.PROJ_SIN2 = 4;	/* Equal-area 		*/
Projection.PROJ_ZEA = 4;	/* Zenithal Equal-area 	*/
Projection.PROJ_ARC = 5;	/* For Schmidt plates	*/
Projection.PROJ_SCHMIDT = 5;	/* For Schmidt plates	*/
Projection.PROJ_AITOFF = 6;	/* Aitoff Projection	*/
Projection.PROJ_AIT = 6;	/* Aitoff Projection	*/
Projection.PROJ_GLS = 7;	/* Global Sin (Sanson)	*/
Projection.PROJ_MERCATOR = 8;
Projection.PROJ_MER = 8;	
Projection.PROJ_LAM = 9;	/* Lambert Projection	*/
Projection.PROJ_LAMBERT = 9;	
Projection.PROJ_TSC = 10;	/* Tangent Sph. Cube	*/
Projection.PROJ_QSC = 11;	/* QuadCube Sph. Cube	*/
Projection.PROJ_MOLLWEIDE = 12;
Projection.PROJ_LIST = [
	"Mercator",Projection.PROJ_MERCATOR,
	"Gnomonic",Projection.PROJ_TAN,
	"Stereographic",Projection.PROJ_TAN2,
	"Orthographic",Projection.PROJ_SIN,
	"Zenithal",Projection.PROJ_ZEA,
	"Schmidt",Projection.PROJ_SCHMIDT,
	"Aitoff",Projection.PROJ_AITOFF,
	"Lambert",Projection.PROJ_LAMBERT,
//	"Tangential",Projection.PROJ_TSC,
//	"Quadrilaterized",Projection.PROJ_QSC,
];
Projection.PROJ_NAME = [
	'-', 'Gnomonic', 'Stereographic', 'Orthographic', 'Equal-area', 'Schmidt plates',
	'Aitoff', 'Global sin', 'Mercator', 'Lambert'
];

Projection.prototype = { 
	
	/** Set the center of the projection
	 * 
	 * (ajout T. Boch, 19/02/2013)
	 * 
	 * */
	setCenter: function(lon0, lat0) {
		this.ROT = this.tr_oR(lon0, lat0);
	},

    /** Reverse the longitude
      * If set to true, longitudes will increase from left to right
      *
      * */
    reverseLongitude: function(b) {
        this.longitudeIsReversed = b;
    },
	
	/**
	 * Set the projection to use
	 * p = projection code
	 */
	setProjection: function(p) {
		this.PROJECTION = p;
	},


	/**
	 * Computes the projection of 1 point : ra,dec => X,Y
	 * alpha, delta = longitude, lattitude
	 */
	project: function(alpha, delta) {
        var u1 = this.tr_ou(alpha, delta);	// u1[3]
		var u2 = this.tr_uu(u1, this.ROT);	// u2[3]
		var P = this.tr_up(this.PROJECTION, u2);	// P[2] = [X,Y]
		if (P == null) {
			return null;
		}

		if( this.longitudeIsReversed) {
            return { X: P[0], Y: -P[1] };
        }
        else {
		    return { X: -P[0], Y: -P[1] };
        }
        //return { X: -P[0], Y: -P[1] };
	},

	/**
	 * Computes the coordinates from a projection point : X,Y => ra,dec
	 * return o = [ ra, dec ]
	 */
	unproject: function(X,Y) {
		if ( ! this.longitudeIsReversed) {
            X = -X;
        }
		Y = -Y;
		var u1 = this.tr_pu(this.PROJECTION, X, Y);	// u1[3]
		var u2 = this.tr_uu1(u1, this.ROT);	// u2[3]
		var o = this.tr_uo(u2);	// o[2]

/*
		if (this.longitudeIsReversed) {
            return { ra: 360-o[0], dec: o[1] };
        }
        else {
		    return { ra: o[0], dec: o[1] };
        }
*/
        return { ra: o[0], dec: o[1] };
	},

	/**
	 * Compute projections from unit vector
	 * The center of the projection correspond to u = [1, 0, 0)
	 * proj = projection system (integer code like _PROJ_MERCATOR_
	 * u[3] = unit vector
	 * return: an array [x,y] or null
	 */
	tr_up: function(proj, u) {
		var x = u[0]; var y = u[1]; var z = u[2];
		var r, den;
		var pp;
		var X,Y;

		r = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].hypot(x,y);			// r = cos b
		if (r == 0.0 && z == 0.0) return null;

		switch(proj) {
			default:
				pp = null;
				break;

			case Projection.PROJ_AITOFF:
				den = Math.sqrt(r*(r+x)/2.0); 		// cos b . cos l/2
				X = Math.sqrt(2.0*r*(r-x));
				den = Math.sqrt((1.0 + den)/2.0); 
				X = X / den;
				Y = z / den;
				if (y < 0.0) X = -X;
				pp = [ X, Y];
				break;

			case Projection.PROJ_GLS:
				Y = Math.asin(z);				// sin b
				X = (r != 0) ? Math.atan2(y,x)*r : 0.0;
				pp = [ X, Y];
				break;

			case Projection.PROJ_MERCATOR:
				if (r != 0) {
					X = Math.atan2(y,x);
					Y = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].atanh(z);
					pp = [ X, Y];
				} else {
					pp = null;
				}
				break;

			case Projection.PROJ_TAN:
				if (x > 0.0) {
					X = y/x;
					Y = z/x;
					pp = [ X, Y ];
				} else {
					pp = null;
				}
				break;

			case Projection.PROJ_TAN2:
				den = (1.0 + x)/2.0;
				if (den > 0.0)	{
					X = y/den;
					Y = z/den;
					pp = [ X, Y ];
				} else {
					pp = null;
				}
			 	break;

			case Projection.PROJ_ARC:
				if (x <= -1.0) {
					// Distance of 180 degrees
					X = Math.PI
					Y = 0.0;
				} else {
					// Arccos(x) = Arcsin(r)
					r = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].hypot(y,z);
					if (x > 0.0) den = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].asinc(r);
					else den = Math.acos(x)/r;
					X = y * den;
					Y = z * den;
				}
				pp = [ X, Y ];
				break;

			case Projection.PROJ_SIN:
				if (x >= 0.0) {
					X = y;
					Y = z;
					pp = [ X, Y ];
				} else {
					pp = null;
				}
				break;

			case Projection.PROJ_SIN2:	// Always possible
				den = Math.sqrt((1.0 + x)/2.0);
				if (den != 0)	{
					X = y / den;
					Y = z / den;
				} else {
					// For x = -1
					X = 2.0;
					Y = 0.0;
				}
				pp = [ X, Y ];
				break;

			case Projection.PROJ_LAMBERT:	// Always possible
				Y = z;
				X = 0;
				if (r != 0)	X = Math.atan2(y,x);
				pp = [ X, Y ];
				break;
	  }
	  return pp;
	},

	/**
	 * Computes Unit vector from a position in projection centered at position (0,0).
	 * proj = projection code
	 * X,Y : coordinates of the point in the projection
	 * returns : the unit vector u[3] or a face number for cube projection. 
	 *           null if the point is outside the limits, or if the projection is unknown.
	 */
	tr_pu: function( proj, X, Y ) {
		var r,s,x,y,z;

		switch(proj) {
			default:
			return null;

			case Projection.PROJ_AITOFF:
				// Limit is ellipse with axises 
				// a = 2 * sqrt(2) ,  b = sqrt(2)
				// Compute dir l/2, b
				r = X*X/8.e0 + Y*Y/2.e0; 	// 1 - cos b . cos l/2
				if (r > 1.0) {
	  				// Test outside domain */
					return null;
				}
				x = 1.0 - r ;	// cos b . cos l/2
				s = Math.sqrt(1.0 - r/2.0) ;	// sqrt(( 1 + cos b . cos l/2)/2)
				y = X * s / 2.0;
				z = Y * s ;
				// From (l/2,b) to (l,b)
				r = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].hypot( x, y ) ;	// cos b
				if (r != 0.0) {
					s = x;
					x = (s*s - y*y) /r;
					y = 2.0 * s * y/r;
				}
				break;

			case Projection.PROJ_GLS:
				// Limit is |Y| <= pi/2
				z = Math.sin(Y);
				r = 1 - z*z;		// cos(b) ** 2
				if (r < 0.0) {
					return null;
				}
				r = Math.sqrt(r);		// cos b
				if (r != 0.0) {
					s = X/r;	// Longitude
				} else {
					s = 0.0;	// For poles
				}
				x = r * Math.cos(s);
				y = r * Math.sin(s);
				break;

			case Projection.PROJ_MERCATOR:
				z = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].tanh(Y);
				r = 1.0/_astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosh(Y);
				x = r * Math.cos(X);
				y = r * Math.sin(X);
				break;

			case Projection.PROJ_LAMBERT:
				// Always possible
				z = Y;
				r = 1 - z*z;		// cos(b) ** 2
				if (r < 0.0) {
					return null;
				}
				r = Math.sqrt(r);		// cos b
				x = r * Math.cos(X);
				y = r * Math.sin(X);
				break;
	
			case Projection.PROJ_TAN:
				// No limit
				x = 1.0 / Math.sqrt(1.0 + X*X + Y*Y);
				y = X * x;
				z = Y * x;
				break;

			case Projection.PROJ_TAN2:
				// No limit
				r = (X*X + Y*Y)/4.0;
				s = 1.0 + r;
				x = (1.0 - r)/s;
				y = X / s;
				z = Y / s;
				break;

			case Projection.PROJ_ARC:
				// Limit is circle, radius PI
				r = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].hypot(X, Y);
				if (r > Math.PI) {
					return null;
				}
				s = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sinc(r);
				x = Math.cos(r);
				y = s * X;
				z = s * Y;
				break;

			case Projection.PROJ_SIN:
				// Limit is circle, radius 1
				s = 1.0 - X*X - Y*Y;
				if (s < 0.0) {
					return null;
				}
				x = Math.sqrt(s);
				y = X;
				z = Y;
				break;

			case Projection.PROJ_SIN2:
				// Limit is circle, radius 2	*/
				r = (X*X + Y*Y)/4.e0;
				if (r > 1.0) {
					return null;
				}
				s = Math.sqrt(1.0 - r);
				x = 1.0 - 2.0 * r;
				y = s * X;
				z = s * Y;
				break;
	  }
	  return [ x,y,z ];
	},

	/**
	 * Creates the rotation matrix R[3][3] defined as
	 * R[0] (first row) = unit vector towards Zenith
	 * R[1] (second row) = unit vector towards East
	 * R[2] (third row) = unit vector towards North
	 * o[2] original angles
	 * @return rotation matrix
	 */
	tr_oR: function(lon, lat) {
		var R = new Array(3);
		R[0] = new Array(3);
		R[1] = new Array(3);
		R[2] = new Array(3);
		R[2][2] =  _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(lat);
		R[0][2] =  _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(lat);
		R[1][1] =  _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(lon);
		R[1][0] =  -_astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(lon);
		R[1][2] =  0.0;
		R[0][0] =  R[2][2] * R[1][1];  
		R[0][1] = -R[2][2] * R[1][0];
		R[2][0] = -R[0][2] * R[1][1];
		R[2][1] =  R[0][2] * R[1][0];
		return R;
	},

	/**
	 * Transformation from polar coordinates to Unit vector
	 * @return U[3]
	 */
	tr_ou: function(ra, dec) {
		var u = new Array(3);
		var cosdec = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(dec);

		u[0] = cosdec * _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].cosd(ra);
		u[1] = cosdec * _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(ra);
		u[2] = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].sind(dec);

		return u;
	},

	/**
	 * Rotates the unit vector u1 using the rotation matrix
	 * u1[3] unit vector
	 * R[3][3] rotation matrix
	 * return resulting unit vector u2[3]
	 */
	tr_uu: function( u1, R ) {
		var u2 = new Array(3);
		var x = u1[0];
		var y = u1[1];
		var z = u1[2];

		u2[0] = R[0][0]*x + R[0][1]*y + R[0][2]*z ;
		u2[1] = R[1][0]*x + R[1][1]*y + R[1][2]*z ;
		u2[2] = R[2][0]*x + R[2][1]*y + R[2][2]*z ;

		return u2;
	},

	/**
	 * reverse rotation the unit vector u1 using the rotation matrix
	 * u1[3] unit vector
	 * R[3][3] rotation matrix
	 * return resulting unit vector u2[3]
	 */
	tr_uu1: function( u1 , R) {
		var u2 = new Array(3);
		var x = u1[0];
		var y = u1[1];
		var z = u1[2];

		u2[0] = R[0][0]*x + R[1][0]*y + R[2][0]*z;
		u2[1] = R[0][1]*x + R[1][1]*y + R[2][1]*z;
		u2[2] = R[0][2]*x + R[1][2]*y + R[2][2]*z;

		return u2;
	},

	/**
	 * Computes angles from direction cosines
	 * u[3] = direction cosines vector
	 * return o = [ ra, dec ]
	 */
	tr_uo: function(u) {
		var x = u[0]; var y = u[1]; var z = u[2];  
		var r2 = x*x + y*y;
		var ra, dec;
		if (r2  == 0.0) {
	 		// in case of poles
			if (z == 0.0) {
				return null;
			}
			ra = 0.0;
			dec = z > 0.0 ? 90.0 : -90.0;
		} else {
			dec = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].atand( z / Math.sqrt(r2));
			ra  = _astroMath_js__WEBPACK_IMPORTED_MODULE_0__["AstroMath"].atan2d (y , x );
			if (ra < 0.0) ra += 360.0;
		}

		return [ ra, dec ];
	}
}


/***/ }),

/***/ "./src/js/libs/fits.js":
/*!*****************************!*\
  !*** ./src/js/libs/fits.js ***!
  \*****************************/
/*! exports provided: astro */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "astro", function() { return astro; });
let astro = (function () {

  let astro = {};

  var Base, BinaryTable, CompressedImage, DataUnit, Decompress, FITS, HDU, Header, HeaderVerify, Image, ImageUtils, Parser, Table, Tabular, _ref, _ref1,
    __hasProp = {}.hasOwnProperty,
    __extends = function(child, parent) { for (var key in parent) { if (__hasProp.call(parent, key)) child[key] = parent[key]; } function ctor() { this.constructor = child; } ctor.prototype = parent.prototype; child.prototype = new ctor(); child.__super__ = parent.prototype; return child; },
    __slice = [].slice;


  Base = (function() {
    function Base() {}

    Base.include = function(obj) {
      var key, value;
      for (key in obj) {
        value = obj[key];
        this.prototype[key] = value;
      }
      return this;
    };

    Base.extend = function(obj) {
      var key, value;
      for (key in obj) {
        value = obj[key];
        this[key] = value;
      }
      return this;
    };

    Base.prototype.proxy = function(func) {
      var _this = this;
      return function() {
        return func.apply(_this, arguments);
      };
    };

    Base.prototype.invoke = function(callback, opts, data) {
      var context;
      context = (opts != null ? opts.context : void 0) != null ? opts.context : this;
      if (callback != null) {
        return callback.call(context, data, opts);
      }
    };

    return Base;

  })();

  Parser = (function(_super) {
    __extends(Parser, _super);

    Parser.prototype.LINEWIDTH = 80;

    Parser.prototype.BLOCKLENGTH = 2880;

    File.prototype.slice = File.prototype.slice || File.prototype.webkitSlice;

    Blob.prototype.slice = Blob.prototype.slice || Blob.prototype.webkitSlice;

    function Parser(arg, callback, opts) {
      var xhr,
        _this = this;
      this.arg = arg;
      this.callback = callback;
      this.opts = opts;
      this.hdus = [];
      this.blockCount = 0;
      this.begin = 0;
      this.end = this.BLOCKLENGTH;
      this.offset = 0;
      this.headerStorage = new Uint8Array();
      if (typeof this.arg === 'string') {
        this.readNextBlock = this._readBlockFromBuffer;
        xhr = new XMLHttpRequest();
        xhr.open('GET', this.arg);
        xhr.responseType = 'arraybuffer';

        // the onerror handling has been added wrt the original fitsjs library as retrieved on the astrojs github repo
        // if an error occurs, we return an empty object
        xhr.onerror = function() {
          _this.invoke(_this.callback, _this.opts);
        }

        xhr.onload = function() {
          if (xhr.status !== 200) {
            _this.invoke(_this.callback, _this.opts);
            return;
          }
          _this.arg = xhr.response;
          _this.length = _this.arg.byteLength;
          return _this.readFromBuffer();
        };
        xhr.send();
      } else {
        this.length = this.arg.size;
        this.readNextBlock = this._readBlockFromFile;
        this.readFromFile();
      }
    }

    Parser.prototype.readFromBuffer = function() {
      var block;
      block = this.arg.slice(this.begin + this.offset, this.end + this.offset);
      return this.readBlock(block);
    };

    Parser.prototype.readFromFile = function() {
      var block,
        _this = this;
      this.reader = new FileReader();
      this.reader.onloadend = function(e) {
        return _this.readBlock(e.target.result);
      };
      block = this.arg.slice(this.begin + this.offset, this.end + this.offset);
      return this.reader.readAsArrayBuffer(block);
    };

    Parser.prototype.readBlock = function(block) {
      var arr, dataLength, dataunit, header, rowIndex, rows, s, slice, tmp, value, _i, _len, _ref;
      arr = new Uint8Array(block);
      tmp = new Uint8Array(this.headerStorage);
      this.headerStorage = new Uint8Array(this.end);
      this.headerStorage.set(tmp, 0);
      this.headerStorage.set(arr, this.begin);
      rows = this.BLOCKLENGTH / this.LINEWIDTH;
      while (rows--) {
        rowIndex = rows * this.LINEWIDTH;
        if (arr[rowIndex] === 32) {
          continue;
        }
        if (arr[rowIndex] === 69 && arr[rowIndex + 1] === 78 && arr[rowIndex + 2] === 68 && arr[rowIndex + 3] === 32) {
          s = '';
          _ref = this.headerStorage;
          for (_i = 0, _len = _ref.length; _i < _len; _i++) {
            value = _ref[_i];
            s += String.fromCharCode(value);
          }
          header = new Header(s);
          this.start = this.end + this.offset;
          dataLength = header.getDataLength();
          slice = this.arg.slice(this.start, this.start + dataLength);
          if (header.hasDataUnit()) {
            dataunit = this.createDataUnit(header, slice);
          }
          this.hdus.push(new HDU(header, dataunit));
          this.offset += this.end + dataLength + this.excessBytes(dataLength);
          if (this.offset === this.length) {
            this.headerStorage = null;
            this.invoke(this.callback, this.opts, this);
            return;
          }
          this.blockCount = 0;
          this.begin = this.blockCount * this.BLOCKLENGTH;
          this.end = this.begin + this.BLOCKLENGTH;
          this.headerStorage = new Uint8Array();
          block = this.arg.slice(this.begin + this.offset, this.end + this.offset);
          this.readNextBlock(block);
          return;
        }
        break;
      }
      this.blockCount += 1;
      this.begin = this.blockCount * this.BLOCKLENGTH;
      this.end = this.begin + this.BLOCKLENGTH;
      block = this.arg.slice(this.begin + this.offset, this.end + this.offset);
      this.readNextBlock(block);
    };

    Parser.prototype._readBlockFromBuffer = function(block) {
      return this.readBlock(block);
    };

    Parser.prototype._readBlockFromFile = function(block) {
      return this.reader.readAsArrayBuffer(block);
    };

    Parser.prototype.createDataUnit = function(header, blob) {
      var type;
      type = header.getDataType();
      return new astro.FITS[type](header, blob);
    };

    Parser.prototype.excessBytes = function(length) {
      return (this.BLOCKLENGTH - (length % this.BLOCKLENGTH)) % this.BLOCKLENGTH;
    };

    Parser.prototype.isEOF = function() {
      if (this.offset === this.length) {
        return true;
      } else {
        return false;
      }
    };

    return Parser;

  })(Base);

   FITS = (function(_super) {
    __extends(FITS, _super);

    function FITS(arg, callback, opts) {
      var parser,
        _this = this;
      this.arg = arg;
      parser = new Parser(this.arg, function(fits) {
        _this.hdus = parser.hdus;
        return _this.invoke(callback, opts, _this);
      });
    }

    FITS.prototype.getHDU = function(index) {
      var hdu, _i, _len, _ref;
      if ((index != null) && (this.hdus[index] != null)) {
        return this.hdus[index];
      }
      _ref = this.hdus;
      for (_i = 0, _len = _ref.length; _i < _len; _i++) {
        hdu = _ref[_i];
        if (hdu.hasData()) {
          return hdu;
        }
      }
    };

    FITS.prototype.getHeader = function(index) {
      return this.getHDU(index).header;
    };

    FITS.prototype.getDataUnit = function(index) {
      return this.getHDU(index).data;
    };

    return FITS;

  })(Base);

  FITS.version = '0.6.5';

  astro.FITS = FITS;

  DataUnit = (function(_super) {
    __extends(DataUnit, _super);

    DataUnit.swapEndian = {
      B: function(value) {
        return value;
      },
      I: function(value) {
        return (value << 8) | (value >> 8);
      },
      J: function(value) {
        return ((value & 0xFF) << 24) | ((value & 0xFF00) << 8) | ((value >> 8) & 0xFF00) | ((value >> 24) & 0xFF);
      }
    };

    DataUnit.swapEndian[8] = DataUnit.swapEndian['B'];

    DataUnit.swapEndian[16] = DataUnit.swapEndian['I'];

    DataUnit.swapEndian[32] = DataUnit.swapEndian['J'];

    function DataUnit(header, data) {
      if (data instanceof ArrayBuffer) {
        this.buffer = data;
      } else {
        this.blob = data;
      }
    }

    return DataUnit;

  })(Base);

  astro.FITS.DataUnit = DataUnit;

  HeaderVerify = {
    verifyOrder: function(keyword, order) {
      if (order !== this.cardIndex) {
        return console.warn("" + keyword + " should appear at index " + this.cardIndex + " in the FITS header");
      }
    },
    verifyBetween: function(keyword, value, lower, upper) {
      if (!(value >= lower && value <= upper)) {
        throw "The " + keyword + " value of " + value + " is not between " + lower + " and " + upper;
      }
    },
    verifyBoolean: function(value) {
      if (value === "T") {
        return true;
      } else {
        return false;
      }
    },
    VerifyFns: {
      SIMPLE: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = arguments[0];
        this.primary = true;
        this.verifyOrder("SIMPLE", 0);
        return this.verifyBoolean(value);
      },
      XTENSION: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        this.extension = true;
        this.extensionType = arguments[0];
        this.verifyOrder("XTENSION", 0);
        return this.extensionType;
      },
      BITPIX: function() {
        var args, key, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        key = "BITPIX";
        value = parseInt(arguments[0]);
        this.verifyOrder(key, 1);
        if (value !== 8 && value !== 16 && value !== 32 && value !== (-32) && value !== (-64)) {
          throw "" + key + " value " + value + " is not permitted";
        }
        return value;
      },
      NAXIS: function() {
        var args, array, key, required, value, _ref;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        key = "NAXIS";
        value = parseInt(arguments[0]);
        array = arguments[1];
        if (!array) {
          this.verifyOrder(key, 2);
          this.verifyBetween(key, value, 0, 999);
          if (this.isExtension()) {
            if ((_ref = this.extensionType) === "TABLE" || _ref === "BINTABLE") {
              required = 2;
              if (value !== required) {
                throw "" + key + " must be " + required + " for TABLE and BINTABLE extensions";
              }
            }
          }
        }
        return value;
      },
      PCOUNT: function() {
        var args, key, order, required, value, _ref;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        key = "PCOUNT";
        value = parseInt(arguments[0]);
        order = 1 + 1 + 1 + this.get("NAXIS");
        this.verifyOrder(key, order);
        if (this.isExtension()) {
          if ((_ref = this.extensionType) === "IMAGE" || _ref === "TABLE") {
            required = 0;
            if (value !== required) {
              throw "" + key + " must be " + required + " for the " + this.extensionType + " extensions";
            }
          }
        }
        return value;
      },
      GCOUNT: function() {
        var args, key, order, required, value, _ref;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        key = "GCOUNT";
        value = parseInt(arguments[0]);
        order = 1 + 1 + 1 + this.get("NAXIS") + 1;
        this.verifyOrder(key, order);
        if (this.isExtension()) {
          if ((_ref = this.extensionType) === "IMAGE" || _ref === "TABLE" || _ref === "BINTABLE") {
            required = 1;
            if (value !== required) {
              throw "" + key + " must be " + required + " for the " + this.extensionType + " extensions";
            }
          }
        }
        return value;
      },
      EXTEND: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = arguments[0];
        if (!this.isPrimary()) {
          throw "EXTEND must only appear in the primary header";
        }
        return this.verifyBoolean(value);
      },
      BSCALE: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseFloat(arguments[0]);
      },
      BZERO: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseFloat(arguments[0]);
      },
      BLANK: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = arguments[0];
        if (!(this.get("BITPIX") > 0)) {
          console.warn("BLANK is not to be used for BITPIX = " + (this.get('BITPIX')));
        }
        return parseInt(value);
      },
      DATAMIN: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseFloat(arguments[0]);
      },
      DATAMAX: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseFloat(arguments[0]);
      },
      EXTVER: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      },
      EXTLEVEL: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      },
      TFIELDS: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = parseInt(arguments[0]);
        this.verifyBetween("TFIELDS", value, 0, 999);
        return value;
      },
      TBCOL: function() {
        var args, index, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = arguments[0];
        index = arguments[2];
        this.verifyBetween("TBCOL", index, 0, this.get("TFIELDS"));
        return value;
      },
      ZIMAGE: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return this.verifyBoolean(arguments[0]);
      },
      ZCMPTYPE: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = arguments[0];
        if (value !== 'GZIP_1' && value !== 'RICE_1' && value !== 'PLIO_1' && value !== 'HCOMPRESS_1') {
          throw "ZCMPTYPE value " + value + " is not permitted";
        }
        if (value !== 'RICE_1') {
          throw "Compress type " + value + " is not yet implement";
        }
        return value;
      },
      ZBITPIX: function() {
        var args, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = parseInt(arguments[0]);
        if (value !== 8 && value !== 16 && value !== 32 && value !== 64 && value !== (-32) && value !== (-64)) {
          throw "ZBITPIX value " + value + " is not permitted";
        }
        return value;
      },
      ZNAXIS: function() {
        var args, array, value;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        value = parseInt(arguments[0]);
        array = arguments[1];
        value = value;
        if (!array) {
          this.verifyBetween("ZNAXIS", value, 0, 999);
        }
        return value;
      },
      ZTILE: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      },
      ZSIMPLE: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        if (arguments[0] === "T") {
          return true;
        } else {
          return false;
        }
      },
      ZPCOUNT: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      },
      ZGCOUNT: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      },
      ZDITHER0: function() {
        var args;
        args = 1 <= arguments.length ? __slice.call(arguments, 0) : [];
        return parseInt(arguments[0]);
      }
    }
  };

  astro.FITS.HeaderVerify = HeaderVerify;

  Header = (function(_super) {
    __extends(Header, _super);

    Header.include(HeaderVerify);

    Header.prototype.arrayPattern = /(\D+)(\d+)/;

    Header.prototype.maxLines = 600;

    function Header(block) {
      var method, name, _ref;
      this.primary = false;
      this.extension = false;
      this.verifyCard = {};
      _ref = this.VerifyFns;
      for (name in _ref) {
        method = _ref[name];
        this.verifyCard[name] = this.proxy(method);
      }
      this.cards = {};
      this.cards["COMMENT"] = [];
      this.cards["HISTORY"] = [];
      this.cardIndex = 0;
      this.block = block;
      this.readBlock(block);
    }

    Header.prototype.get = function(key) {
      if (this.contains(key)) {
        return this.cards[key].value;
      } else {
        return null;
      }
    };

    Header.prototype.set = function(key, value, comment) {
      comment = comment || '';
      this.cards[key] = {
        index: this.cardIndex,
        value: value,
        comment: comment
      };
      return this.cardIndex += 1;
    };

    Header.prototype.contains = function(key) {
      return this.cards.hasOwnProperty(key);
    };

    Header.prototype.readLine = function(l) {
      var blank, comment, firstByte, indicator, key, value, _ref;
      key = l.slice(0, 8).trim();
      blank = key === '';
      if (blank) {
        return;
      }
      indicator = l.slice(8, 10);
      value = l.slice(10);
      if (indicator !== "= ") {
        if (key === 'COMMENT' || key === 'HISTORY') {
          this.cards[key].push(value.trim());
        }
        return;
      }
      _ref = value.split(' /'), value = _ref[0], comment = _ref[1];
      value = value.trim();
      firstByte = value[0];
      if (firstByte === "'") {
        value = value.slice(1, -1).trim();
      } else {
        if (value !== 'T' && value !== 'F') {
          value = parseFloat(value);
        }
      }
      value = this.validate(key, value);
      return this.set(key, value, comment);
    };

    Header.prototype.validate = function(key, value) {
      var baseKey, index, isArray, match, _ref;
      index = null;
      baseKey = key;
      isArray = this.arrayPattern.test(key);
      if (isArray) {
        match = this.arrayPattern.exec(key);
        _ref = match.slice(1), baseKey = _ref[0], index = _ref[1];
      }
      if (baseKey in this.verifyCard) {
        value = this.verifyCard[baseKey](value, isArray, index);
      }
      return value;
    };

    Header.prototype.readBlock = function(block) {
      var i, line, lineWidth, nLines, _i, _ref, _results;
      lineWidth = 80;
      nLines = block.length / lineWidth;
      nLines = nLines < this.maxLines ? nLines : this.maxLines;
      _results = [];
      for (i = _i = 0, _ref = nLines - 1; 0 <= _ref ? _i <= _ref : _i >= _ref; i = 0 <= _ref ? ++_i : --_i) {
        line = block.slice(i * lineWidth, (i + 1) * lineWidth);
        _results.push(this.readLine(line));
      }
      return _results;
    };

    Header.prototype.hasDataUnit = function() {
      if (this.get("NAXIS") === 0) {
        return false;
      } else {
        return true;
      }
    };

    Header.prototype.getDataLength = function() {
      var i, length, naxis, _i, _ref;
      if (!this.hasDataUnit()) {
        return 0;
      }
      naxis = [];
      for (i = _i = 1, _ref = this.get("NAXIS"); 1 <= _ref ? _i <= _ref : _i >= _ref; i = 1 <= _ref ? ++_i : --_i) {
        naxis.push(this.get("NAXIS" + i));
      }
      length = naxis.reduce(function(a, b) {
        return a * b;
      }) * Math.abs(this.get("BITPIX")) / 8;
      length += this.get("PCOUNT");
      return length;
    };

    Header.prototype.getDataType = function() {
      switch (this.extensionType) {
        case 'BINTABLE':
          if (this.contains('ZIMAGE')) {
            return 'CompressedImage';
          }
          return 'BinaryTable';
        case 'TABLE':
          return 'Table';
        default:
          if (this.hasDataUnit()) {
            return 'Image';
          } else {
            return null;
          }
      }
    };

    Header.prototype.isPrimary = function() {
      return this.primary;
    };

    Header.prototype.isExtension = function() {
      return this.extension;
    };

    return Header;

  })(Base);

  astro.FITS.Header = Header;

  ImageUtils = {
    getExtent: function(arr) {
      var index, max, min, value;
      index = arr.length;
      while (index--) {
        value = arr[index];
        if (isNaN(value)) {
          continue;
        }
        min = max = value;
        break;
      }
      if (index === -1) {
        return [NaN, NaN];
      }
      while (index--) {
        value = arr[index];
        if (isNaN(value)) {
          continue;
        }
        if (value < min) {
          min = value;
        }
        if (value > max) {
          max = value;
        }
      }
      return [min, max];
    },
    getPixel: function(arr, x, y) {
      return arr[y * this.width + x];
    }
  };

  astro.FITS.ImageUtils = ImageUtils;

  Image = (function(_super) {
    __extends(Image, _super);

    Image.include(ImageUtils);

    Image.prototype.allocationSize = 16777216;

    function Image(header, data) {
      var begin, frame, i, naxis, _i, _j, _ref;
      Image.__super__.constructor.apply(this, arguments);
      naxis = header.get("NAXIS");
      this.bitpix = header.get("BITPIX");
      this.naxis = [];
      for (i = _i = 1; 1 <= naxis ? _i <= naxis : _i >= naxis; i = 1 <= naxis ? ++_i : --_i) {
        this.naxis.push(header.get("NAXIS" + i));
      }
      this.width = header.get("NAXIS1");
      this.height = header.get("NAXIS2") || 1;
      this.depth = header.get("NAXIS3") || 1;
      this.bzero = header.get("BZERO") || 0;
      this.bscale = header.get("BSCALE") || 1;
      this.bytes = Math.abs(this.bitpix) / 8;
      this.length = this.naxis.reduce(function(a, b) {
        return a * b;
      }) * Math.abs(this.bitpix) / 8;
      this.frame = 0;
      this.frameOffsets = [];
      this.frameLength = this.bytes * this.width * this.height;
      this.nBuffers = this.buffer != null ? 1 : 2;
      for (i = _j = 0, _ref = this.depth - 1; 0 <= _ref ? _j <= _ref : _j >= _ref; i = 0 <= _ref ? ++_j : --_j) {
        begin = i * this.frameLength;
        frame = {
          begin: begin
        };
        if (this.buffer != null) {
          frame.buffers = [this.buffer.slice(begin, begin + this.frameLength)];
        }
        this.frameOffsets.push(frame);
      }
    }

    Image.prototype._getFrame = function(buffer, bitpix, bzero, bscale) {
      var arr, bytes, dataType, i, nPixels, swapEndian, tmp, value;
      bytes = Math.abs(bitpix) / 8;
      nPixels = i = buffer.byteLength / bytes;
      dataType = Math.abs(bitpix);
      if (bitpix > 0) {
        switch (bitpix) {
          case 8:
            tmp = new Uint8Array(buffer);
            tmp = new Uint16Array(tmp);
            swapEndian = function(value) {
              return value;
            };
            break;
          case 16:
            tmp = new Int16Array(buffer);
            swapEndian = function(value) {
              return ((value & 0xFF) << 8) | ((value >> 8) & 0xFF);
            };
            break;
          case 32:
            tmp = new Int32Array(buffer);
            swapEndian = function(value) {
              return ((value & 0xFF) << 24) | ((value & 0xFF00) << 8) | ((value >> 8) & 0xFF00) | ((value >> 24) & 0xFF);
            };
        }
        if (!(parseInt(bzero) === bzero && parseInt(bscale) === bscale)) {
          arr = new Float32Array(tmp.length);
        } else {
          arr = tmp;
        }
        while (nPixels--) {
          tmp[nPixels] = swapEndian(tmp[nPixels]);
          arr[nPixels] = bzero + bscale * tmp[nPixels];
        }
      } else {
        arr = new Uint32Array(buffer);
        swapEndian = function(value) {
          return ((value & 0xFF) << 24) | ((value & 0xFF00) << 8) | ((value >> 8) & 0xFF00) | ((value >> 24) & 0xFF);
        };
        while (i--) {
          value = arr[i];
          arr[i] = swapEndian(value);
        }
        arr = new Float32Array(buffer);
        while (nPixels--) {
          arr[nPixels] = bzero + bscale * arr[nPixels];
        }
      }
      return arr;
    };

    Image.prototype._getFrameAsync = function(buffers, callback, opts) {
      var URL, blobGetFrame, blobOnMessage, fn1, fn2, i, mime, msg, onmessage, pixels, start, urlGetFrame, urlOnMessage, worker,
        _this = this;
      onmessage = function(e) {
        var arr, bitpix, bscale, buffer, bzero, data, url;
        data = e.data;
        buffer = data.buffer;
        bitpix = data.bitpix;
        bzero = data.bzero;
        bscale = data.bscale;
        url = data.url;
        importScripts(url);
        arr = _getFrame(buffer, bitpix, bzero, bscale);
        return postMessage(arr);
      };
      fn1 = onmessage.toString().replace('return postMessage', 'postMessage');
      fn1 = "onmessage = " + fn1;
      fn2 = this._getFrame.toString();
      fn2 = fn2.replace('function', 'function _getFrame');
      mime = "application/javascript";
      blobOnMessage = new Blob([fn1], {
        type: mime
      });
      blobGetFrame = new Blob([fn2], {
        type: mime
      });
      URL = window.URL || window.webkitURL;
      urlOnMessage = URL.createObjectURL(blobOnMessage);
      urlGetFrame = URL.createObjectURL(blobGetFrame);
      worker = new Worker(urlOnMessage);
      msg = {
        buffer: buffers[0],
        bitpix: this.bitpix,
        bzero: this.bzero,
        bscale: this.bscale,
        url: urlGetFrame
      };
      i = 0;
      pixels = null;
      start = 0;
      worker.onmessage = function(e) {
        var arr;
        arr = e.data;
        if (pixels == null) {
          pixels = new arr.constructor(_this.width * _this.height);
        }
        pixels.set(arr, start);
        start += arr.length;
        i += 1;
        if (i === _this.nBuffers) {
          _this.invoke(callback, opts, pixels);
          URL.revokeObjectURL(urlOnMessage);
          URL.revokeObjectURL(urlGetFrame);
          return worker.terminate();
        } else {
          msg.buffer = buffers[i];
          return worker.postMessage(msg, [buffers[i]]);
        }
      };
      worker.postMessage(msg, [buffers[0]]);
    };

    Image.prototype.getFrame = function(frame, callback, opts) {
      var begin, blobFrame, blobs, buffers, bytesPerBuffer, frameInfo, i, nRowsPerBuffer, reader, start, _i, _ref,
        _this = this;
      this.frame = frame || this.frame;
      frameInfo = this.frameOffsets[this.frame];
      buffers = frameInfo.buffers;
      if ((buffers != null ? buffers.length : void 0) === this.nBuffers) {
        return this._getFrameAsync(buffers, callback, opts);
      } else {
        this.frameOffsets[this.frame].buffers = [];
        begin = frameInfo.begin;
        blobFrame = this.blob.slice(begin, begin + this.frameLength);
        blobs = [];
        nRowsPerBuffer = Math.floor(this.height / this.nBuffers);
        bytesPerBuffer = nRowsPerBuffer * this.bytes * this.width;
        for (i = _i = 0, _ref = this.nBuffers - 1; 0 <= _ref ? _i <= _ref : _i >= _ref; i = 0 <= _ref ? ++_i : --_i) {
          start = i * bytesPerBuffer;
          if (i === this.nBuffers - 1) {
            blobs.push(blobFrame.slice(start));
          } else {
            blobs.push(blobFrame.slice(start, start + bytesPerBuffer));
          }
        }
        buffers = [];
        reader = new FileReader();
        reader.frame = this.frame;
        i = 0;
        reader.onloadend = function(e) {
          var buffer;
          frame = e.target.frame;
          buffer = e.target.result;
          _this.frameOffsets[frame].buffers.push(buffer);
          i += 1;
          if (i === _this.nBuffers) {
            return _this.getFrame(frame, callback, opts);
          } else {
            return reader.readAsArrayBuffer(blobs[i]);
          }
        };
        return reader.readAsArrayBuffer(blobs[0]);
      }
    };

    Image.prototype.getFrames = function(frame, number, callback, opts) {
      var cb,
        _this = this;
      cb = function(arr, opts) {
        _this.invoke(callback, opts, arr);
        number -= 1;
        frame += 1;
        if (!number) {
          return;
        }
        return _this.getFrame(frame, cb, opts);
      };
      return this.getFrame(frame, cb, opts);
    };

    Image.prototype.isDataCube = function() {
      if (this.naxis.length > 2) {
        return true;
      } else {
        return false;
      }
    };

    return Image;

  })(DataUnit);

  astro.FITS.Image = Image;

  Tabular = (function(_super) {
    __extends(Tabular, _super);

    Tabular.prototype.maxMemory = 1048576;

    function Tabular(header, data) {
      Tabular.__super__.constructor.apply(this, arguments);
      this.rowByteSize = header.get("NAXIS1");
      this.rows = header.get("NAXIS2");
      this.cols = header.get("TFIELDS");
      this.length = this.rowByteSize * this.rows;
      this.heapLength = header.get("PCOUNT");
      this.columns = this.getColumns(header);
      if (this.buffer != null) {
        this.rowsInMemory = this._rowsInMemoryBuffer;
        this.heap = this.buffer.slice(this.length, this.length + this.heapLength);
      } else {
        this.rowsInMemory = this._rowsInMemoryBlob;
        this.firstRowInBuffer = this.lastRowInBuffer = 0;
        this.nRowsInBuffer = Math.floor(this.maxMemory / this.rowByteSize);
      }
      this.accessors = [];
      this.descriptors = [];
      this.elementByteLengths = [];
      this.setAccessors(header);
    }

    Tabular.prototype._rowsInMemoryBuffer = function() {
      return true;
    };

    Tabular.prototype._rowsInMemoryBlob = function(firstRow, lastRow) {
      if (firstRow < this.firstRowInBuffer) {
        return false;
      }
      if (lastRow > this.lastRowInBuffer) {
        return false;
      }
      return true;
    };

    Tabular.prototype.getColumns = function(header) {
      var columns, i, key, _i, _ref;
      columns = [];
      for (i = _i = 1, _ref = this.cols; 1 <= _ref ? _i <= _ref : _i >= _ref; i = 1 <= _ref ? ++_i : --_i) {
        key = "TTYPE" + i;
        if (!header.contains(key)) {
          return null;
        }
        columns.push(header.get(key));
      }
      return columns;
    };

    Tabular.prototype.getColumn = function(name, callback, opts) {
      var accessor, cb, column, descriptor, elementByteLength, elementByteOffset, factor, i, index, iterations, rowsPerIteration,
        _this = this;
      if (this.blob != null) {
        index = this.columns.indexOf(name);
        descriptor = this.descriptors[index];
        accessor = this.accessors[index];
        elementByteLength = this.elementByteLengths[index];
        elementByteOffset = this.elementByteLengths.slice(0, index);
        if (elementByteOffset.length === 0) {
          elementByteOffset = 0;
        } else {
          elementByteOffset = elementByteOffset.reduce(function(a, b) {
            return a + b;
          });
        }
        column = this.typedArray[descriptor] != null ? new this.typedArray[descriptor](this.rows) : [];
        rowsPerIteration = ~~(this.maxMemory / this.rowByteSize);
        rowsPerIteration = Math.min(rowsPerIteration, this.rows);
        factor = this.rows / rowsPerIteration;
        iterations = Math.floor(factor) === factor ? factor : Math.floor(factor) + 1;
        i = 0;
        index = 0;
        cb = function(buffer, opts) {
          var nRows, offset, startRow, view;
          nRows = buffer.byteLength / _this.rowByteSize;
          view = new DataView(buffer);
          offset = elementByteOffset;
          while (nRows--) {
            column[i] = accessor(view, offset)[0];
            i += 1;
            offset += _this.rowByteSize;
          }
          iterations -= 1;
          index += 1;
          if (iterations) {
            startRow = index * rowsPerIteration;
            return _this.getTableBuffer(startRow, rowsPerIteration, cb, opts);
          } else {
            _this.invoke(callback, opts, column);
          }
        };
        return this.getTableBuffer(0, rowsPerIteration, cb, opts);
      } else {
        cb = function(rows, opts) {
          column = rows.map(function(d) {
            return d[name];
          });
          return _this.invoke(callback, opts, column);
        };
        return this.getRows(0, this.rows, cb, opts);
      }
    };

    Tabular.prototype.getTableBuffer = function(row, number, callback, opts) {
      var begin, blobRows, end, reader,
        _this = this;
      number = Math.min(this.rows - row, number);
      begin = row * this.rowByteSize;
      end = begin + number * this.rowByteSize;
      blobRows = this.blob.slice(begin, end);
      reader = new FileReader();
      reader.row = row;
      reader.number = number;
      reader.onloadend = function(e) {
        return _this.invoke(callback, opts, e.target.result);
      };
      return reader.readAsArrayBuffer(blobRows);
    };

    Tabular.prototype.getRows = function(row, number, callback, opts) {
      var begin, blobRows, buffer, end, reader, rows,
        _this = this;
      if (this.rowsInMemory(row, row + number)) {
        if (this.blob != null) {
          buffer = this.buffer;
        } else {
          begin = row * this.rowByteSize;
          end = begin + number * this.rowByteSize;
          buffer = this.buffer.slice(begin, end);
        }
        rows = this._getRows(buffer, number);
        this.invoke(callback, opts, rows);
        return rows;
      } else {
        begin = row * this.rowByteSize;
        end = begin + Math.max(this.nRowsInBuffer * this.rowByteSize, number * this.rowByteSize);
        blobRows = this.blob.slice(begin, end);
        reader = new FileReader();
        reader.row = row;
        reader.number = number;
        reader.onloadend = function(e) {
          var target;
          target = e.target;
          _this.buffer = target.result;
          _this.firstRowInBuffer = _this.lastRowInBuffer = target.row;
          _this.lastRowInBuffer += target.number;
          return _this.getRows(row, number, callback, opts);
        };
        return reader.readAsArrayBuffer(blobRows);
      }
    };

    return Tabular;

  })(DataUnit);

  astro.FITS.Tabular = Tabular;

  Table = (function(_super) {
    __extends(Table, _super);

    function Table() {
      _ref = Table.__super__.constructor.apply(this, arguments);
      return _ref;
    }

    Table.prototype.dataAccessors = {
      A: function(value) {
        return value.trim();
      },
      I: function(value) {
        return parseInt(value);
      },
      F: function(value) {
        return parseFloat(value);
      },
      E: function(value) {
        return parseFloat(value);
      },
      D: function(value) {
        return parseFloat(value);
      }
    };

    Table.prototype.setAccessors = function(header) {
      var descriptor, form, i, match, pattern, type, _i, _ref1, _results,
        _this = this;
      pattern = /([AIFED])(\d+)\.*(\d+)*/;
      _results = [];
      for (i = _i = 1, _ref1 = this.cols; 1 <= _ref1 ? _i <= _ref1 : _i >= _ref1; i = 1 <= _ref1 ? ++_i : --_i) {
        form = header.get("TFORM" + i);
        type = header.get("TTYPE" + i);
        match = pattern.exec(form);
        descriptor = match[1];
        _results.push((function(descriptor) {
          var accessor;
          accessor = function(value) {
            return _this.dataAccessors[descriptor](value);
          };
          return _this.accessors.push(accessor);
        })(descriptor));
      }
      return _results;
    };

    Table.prototype._getRows = function(buffer) {
      var accessor, arr, begin, end, i, index, line, nRows, row, rows, subarray, value, _i, _j, _k, _len, _len1, _ref1, _ref2;
      nRows = buffer.byteLength / this.rowByteSize;
      arr = new Uint8Array(buffer);
      rows = [];
      for (i = _i = 0, _ref1 = nRows - 1; 0 <= _ref1 ? _i <= _ref1 : _i >= _ref1; i = 0 <= _ref1 ? ++_i : --_i) {
        begin = i * this.rowByteSize;
        end = begin + this.rowByteSize;
        subarray = arr.subarray(begin, end);
        line = '';
        for (_j = 0, _len = subarray.length; _j < _len; _j++) {
          value = subarray[_j];
          line += String.fromCharCode(value);
        }
        line = line.trim().split(/\s+/);
        row = {};
        _ref2 = this.accessors;
        for (index = _k = 0, _len1 = _ref2.length; _k < _len1; index = ++_k) {
          accessor = _ref2[index];
          value = line[index];
          row[this.columns[index]] = accessor(value);
        }
        rows.push(row);
      }
      return rows;
    };

    return Table;

  })(Tabular);

  astro.FITS.Table = Table;

  BinaryTable = (function(_super) {
    __extends(BinaryTable, _super);

    function BinaryTable() {
      _ref1 = BinaryTable.__super__.constructor.apply(this, arguments);
      return _ref1;
    }

    BinaryTable.prototype.typedArray = {
      B: Uint8Array,
      I: Uint16Array,
      J: Uint32Array,
      E: Float32Array,
      D: Float64Array,
      1: Uint8Array,
      2: Uint16Array,
      4: Uint32Array
    };

    BinaryTable.offsets = {
      L: 1,
      B: 1,
      I: 2,
      J: 4,
      K: 8,
      A: 1,
      E: 4,
      D: 8,
      C: 8,
      M: 16
    };

    BinaryTable.prototype.dataAccessors = {
      L: function(view, offset) {
        var val, x;
        x = view.getInt8(offset);
        offset += 1;
        val = x === 84 ? true : false;
        return [val, offset];
      },
      B: function(view, offset) {
        var val;
        val = view.getUint8(offset);
        offset += 1;
        return [val, offset];
      },
      I: function(view, offset) {
        var val;
        val = view.getInt16(offset);
        offset += 2;
        return [val, offset];
      },
      J: function(view, offset) {
        var val;
        val = view.getInt32(offset);
        offset += 4;
        return [val, offset];
      },
      K: function(view, offset) {
        var factor, highByte, lowByte, mod, val;
        highByte = Math.abs(view.getInt32(offset));
        offset += 4;
        lowByte = Math.abs(view.getInt32(offset));
        offset += 4;
        mod = highByte % 10;
        factor = mod ? -1 : 1;
        highByte -= mod;
        val = factor * ((highByte << 32) | lowByte);
        return [val, offset];
      },
      A: function(view, offset) {
        var val;
        val = view.getUint8(offset);
        val = String.fromCharCode(val);
        offset += 1;
        return [val, offset];
      },
      E: function(view, offset) {
        var val;
        val = view.getFloat32(offset);
        offset += 4;
        return [val, offset];
      },
      D: function(view, offset) {
        var val;
        val = view.getFloat64(offset);
        offset += 8;
        return [val, offset];
      },
      C: function(view, offset) {
        var val, val1, val2;
        val1 = view.getFloat32(offset);
        offset += 4;
        val2 = view.getFloat32(offset);
        offset += 4;
        val = [val1, val2];
        return [val, offset];
      },
      M: function(view, offset) {
        var val, val1, val2;
        val1 = view.getFloat64(offset);
        offset += 8;
        val2 = view.getFloat64(offset);
        offset += 8;
        val = [val1, val2];
        return [val, offset];
      }
    };

    BinaryTable.prototype.toBits = function(byte) {
      var arr, i;
      arr = [];
      i = 128;
      while (i >= 1) {
        arr.push((byte & i ? 1 : 0));
        i /= 2;
      }
      return arr;
    };

    BinaryTable.prototype.getFromHeap = function(view, offset, descriptor) {
      var arr, heapOffset, heapSlice, i, length;
      length = view.getInt32(offset);
      offset += 4;
      heapOffset = view.getInt32(offset);
      offset += 4;
      heapSlice = this.heap.slice(heapOffset, heapOffset + length);
      arr = new this.typedArray[descriptor](heapSlice);
      i = arr.length;
      while (i--) {
        arr[i] = this.constructor.swapEndian[descriptor](arr[i]);
      }
      return [arr, offset];
    };

    BinaryTable.prototype.setAccessors = function(header) {
      var count, descriptor, form, i, isArray, match, pattern, type, _i, _ref2, _results,
        _this = this;
      pattern = /(\d*)([P|Q]*)([L|X|B|I|J|K|A|E|D|C|M]{1})/;
      _results = [];
      for (i = _i = 1, _ref2 = this.cols; 1 <= _ref2 ? _i <= _ref2 : _i >= _ref2; i = 1 <= _ref2 ? ++_i : --_i) {
        form = header.get("TFORM" + i);
        type = header.get("TTYPE" + i);
        match = pattern.exec(form);
        count = parseInt(match[1]) || 1;
        isArray = match[2];
        descriptor = match[3];
        _results.push((function(descriptor, count) {
          var accessor, nBytes;
          _this.descriptors.push(descriptor);
          _this.elementByteLengths.push(_this.constructor.offsets[descriptor] * count);
          if (isArray) {
            switch (type) {
              case "COMPRESSED_DATA":
                accessor = function(view, offset) {
                  var arr, pixels, _ref3;
                  _ref3 = _this.getFromHeap(view, offset, descriptor), arr = _ref3[0], offset = _ref3[1];
                  pixels = new _this.typedArray[_this.algorithmParameters["BYTEPIX"]](_this.ztile[0]);
                  Decompress.Rice(arr, _this.algorithmParameters["BLOCKSIZE"], _this.algorithmParameters["BYTEPIX"], pixels, _this.ztile[0], Decompress.RiceSetup);
                  return [pixels, offset];
                };
                break;
              case "GZIP_COMPRESSED_DATA":
                accessor = function(view, offset) {
                  var arr;
                  arr = new Float32Array(_this.width);
                  i = arr.length;
                  while (i--) {
                    arr[i] = NaN;
                  }
                  return [arr, offset];
                };
                break;
              default:
                accessor = function(view, offset) {
                  return _this.getFromHeap(view, offset, descriptor);
                };
            }
          } else {
            if (count === 1) {
              accessor = function(view, offset) {
                var value, _ref3;
                _ref3 = _this.dataAccessors[descriptor](view, offset), value = _ref3[0], offset = _ref3[1];
                return [value, offset];
              };
            } else {
              if (descriptor === 'X') {
                nBytes = Math.log(count) / Math.log(2);
                accessor = function(view, offset) {
                  var arr, bits, buffer, byte, bytes, _j, _len;
                  buffer = view.buffer.slice(offset, offset + nBytes);
                  bytes = new Uint8Array(buffer);
                  bits = [];
                  for (_j = 0, _len = bytes.length; _j < _len; _j++) {
                    byte = bytes[_j];
                    arr = _this.toBits(byte);
                    bits = bits.concat(arr);
                  }
                  offset += nBytes;
                  return [bits.slice(0, +(count - 1) + 1 || 9e9), offset];
                };
              } else if (descriptor === 'A') {
                accessor = function(view, offset) {
                  var arr, buffer, s, value, _j, _len;
                  buffer = view.buffer.slice(offset, offset + count);
                  arr = new Uint8Array(buffer);
                  s = '';
                  for (_j = 0, _len = arr.length; _j < _len; _j++) {
                    value = arr[_j];
                    s += String.fromCharCode(value);
                  }
                  s = s.trim();
                  offset += count;
                  return [s, offset];
                };
              } else {
                accessor = function(view, offset) {
                  var data, value, _ref3;
                  i = count;
                  data = [];
                  while (i--) {
                    _ref3 = _this.dataAccessors[descriptor](view, offset), value = _ref3[0], offset = _ref3[1];
                    data.push(value);
                  }
                  return [data, offset];
                };
              }
            }
          }
          return _this.accessors.push(accessor);
        })(descriptor, count));
      }
      return _results;
    };

    BinaryTable.prototype._getRows = function(buffer, nRows) {
      var accessor, index, offset, row, rows, value, view, _i, _len, _ref2, _ref3;
      view = new DataView(buffer);
      offset = 0;
      rows = [];
      while (nRows--) {
        row = {};
        _ref2 = this.accessors;
        for (index = _i = 0, _len = _ref2.length; _i < _len; index = ++_i) {
          accessor = _ref2[index];
          _ref3 = accessor(view, offset), value = _ref3[0], offset = _ref3[1];
          row[this.columns[index]] = value;
        }
        rows.push(row);
      }
      return rows;
    };

    return BinaryTable;

  })(Tabular);

  astro.FITS.BinaryTable = BinaryTable;

  Decompress = {
    RiceSetup: {
      1: function(array) {
        var fsbits, fsmax, lastpix, pointer;
        pointer = 1;
        fsbits = 3;
        fsmax = 6;
        lastpix = array[0];
        return [fsbits, fsmax, lastpix, pointer];
      },
      2: function(array) {
        var bytevalue, fsbits, fsmax, lastpix, pointer;
        pointer = 2;
        fsbits = 4;
        fsmax = 14;
        lastpix = 0;
        bytevalue = array[0];
        lastpix = lastpix | (bytevalue << 8);
        bytevalue = array[1];
        lastpix = lastpix | bytevalue;
        return [fsbits, fsmax, lastpix, pointer];
      },
      4: function(array) {
        var bytevalue, fsbits, fsmax, lastpix, pointer;
        pointer = 4;
        fsbits = 5;
        fsmax = 25;
        lastpix = 0;
        bytevalue = array[0];
        lastpix = lastpix | (bytevalue << 24);
        bytevalue = array[1];
        lastpix = lastpix | (bytevalue << 16);
        bytevalue = array[2];
        lastpix = lastpix | (bytevalue << 8);
        bytevalue = array[3];
        lastpix = lastpix | bytevalue;
        return [fsbits, fsmax, lastpix, pointer];
      }
    },
    Rice: function(array, blocksize, bytepix, pixels, nx, setup) {
      var b, bbits, diff, fs, fsbits, fsmax, i, imax, k, lastpix, nbits, nonzeroCount, nzero, pointer, _ref2, _ref3;
      bbits = 1 << fsbits;
      _ref2 = setup[bytepix](array), fsbits = _ref2[0], fsmax = _ref2[1], lastpix = _ref2[2], pointer = _ref2[3];
      nonzeroCount = new Uint8Array(256);
      nzero = 8;
      _ref3 = [128, 255], k = _ref3[0], i = _ref3[1];
      while (i >= 0) {
        while (i >= k) {
          nonzeroCount[i] = nzero;
          i -= 1;
        }
        k = k / 2;
        nzero -= 1;
      }
      nonzeroCount[0] = 0;
      b = array[pointer++];
      nbits = 8;
      i = 0;
      while (i < nx) {
        nbits -= fsbits;
        while (nbits < 0) {
          b = (b << 8) | array[pointer++];
          nbits += 8;
        }
        fs = (b >> nbits) - 1;
        b &= (1 << nbits) - 1;
        imax = i + blocksize;
        if (imax > nx) {
          imax = nx;
        }
        if (fs < 0) {
          while (i < imax) {
            pixels[i] = lastpix;
            i += 1;
          }
        } else if (fs === fsmax) {
          while (i < imax) {
            k = bbits - nbits;
            diff = b << k;
            k -= 8;
            while (k >= 0) {
              b = array[pointer++];
              diff |= b << k;
              k -= 8;
            }
            if (nbits > 0) {
              b = array[pointer++];
              diff |= b >> (-k);
              b &= (1 << nbits) - 1;
            } else {
              b = 0;
            }
            if ((diff & 1) === 0) {
              diff = diff >> 1;
            } else {
              diff = ~(diff >> 1);
            }
            pixels[i] = diff + lastpix;
            lastpix = pixels[i];
            i++;
          }
        } else {
          while (i < imax) {
            while (b === 0) {
              nbits += 8;
              b = array[pointer++];
            }
            nzero = nbits - nonzeroCount[b];
            nbits -= nzero + 1;
            b ^= 1 << nbits;
            nbits -= fs;
            while (nbits < 0) {
              b = (b << 8) | array[pointer++];
              nbits += 8;
            }
            diff = (nzero << fs) | (b >> nbits);
            b &= (1 << nbits) - 1;
            if ((diff & 1) === 0) {
              diff = diff >> 1;
            } else {
              diff = ~(diff >> 1);
            }
            pixels[i] = diff + lastpix;
            lastpix = pixels[i];
            i++;
          }
        }
      }
      return pixels;
    }
  };

  astro.FITS.Decompress = Decompress;

  CompressedImage = (function(_super) {
    __extends(CompressedImage, _super);

    CompressedImage.include(ImageUtils);

    CompressedImage.extend(Decompress);

    CompressedImage.randomGenerator = function() {
      var a, i, m, random, seed, temp, _i;
      a = 16807;
      m = 2147483647;
      seed = 1;
      random = new Float32Array(10000);
      for (i = _i = 0; _i <= 9999; i = ++_i) {
        temp = a * seed;
        seed = temp - m * parseInt(temp / m);
        random[i] = seed / m;
      }
      return random;
    };

    CompressedImage.randomSequence = CompressedImage.randomGenerator();

    function CompressedImage(header, data) {
      var i, key, value, ztile, _i, _ref2;
      CompressedImage.__super__.constructor.apply(this, arguments);
      this.zcmptype = header.get("ZCMPTYPE");
      this.zbitpix = header.get("ZBITPIX");
      this.znaxis = header.get("ZNAXIS");
      this.zblank = header.get("ZBLANK");
      this.blank = header.get("BLANK");
      this.zdither = header.get('ZDITHER0') || 0;
      this.ztile = [];
      for (i = _i = 1, _ref2 = this.znaxis; 1 <= _ref2 ? _i <= _ref2 : _i >= _ref2; i = 1 <= _ref2 ? ++_i : --_i) {
        ztile = header.contains("ZTILE" + i) ? header.get("ZTILE" + i) : i === 1 ? header.get("ZNAXIS1") : 1;
        this.ztile.push(ztile);
      }
      this.width = header.get("ZNAXIS1");
      this.height = header.get("ZNAXIS2") || 1;
      this.algorithmParameters = {};
      if (this.zcmptype === 'RICE_1') {
        this.algorithmParameters["BLOCKSIZE"] = 32;
        this.algorithmParameters["BYTEPIX"] = 4;
      }
      i = 1;
      while (true) {
        key = "ZNAME" + i;
        if (!header.contains(key)) {
          break;
        }
        value = "ZVAL" + i;
        this.algorithmParameters[header.get(key)] = header.get(value);
        i += 1;
      }
      this.zmaskcmp = header.get("ZMASKCMP");
      this.zquantiz = header.get("ZQUANTIZ") || "LINEAR_SCALING";
      this.bzero = header.get("BZERO") || 0;
      this.bscale = header.get("BSCALE") || 1;
    }

    CompressedImage.prototype._getRows = function(buffer, nRows) {
      var accessor, arr, blank, data, i, index, nTile, offset, r, rIndex, row, scale, seed0, seed1, value, view, zero, _i, _j, _len, _len1, _ref2, _ref3;
      view = new DataView(buffer);
      offset = 0;
      arr = new Float32Array(this.width * this.height);
      while (nRows--) {
        row = {};
        _ref2 = this.accessors;
        for (index = _i = 0, _len = _ref2.length; _i < _len; index = ++_i) {
          accessor = _ref2[index];
          _ref3 = accessor(view, offset), value = _ref3[0], offset = _ref3[1];
          row[this.columns[index]] = value;
        }
        data = row['COMPRESSED_DATA'] || row['UNCOMPRESSED_DATA'] || row['GZIP_COMPRESSED_DATA'];
        blank = row['ZBLANK'] || this.zblank;
        scale = row['ZSCALE'] || this.bscale;
        zero = row['ZZERO'] || this.bzero;
        nTile = this.height - nRows;
        seed0 = nTile + this.zdither - 1;
        seed1 = (seed0 - 1) % 10000;
        rIndex = parseInt(this.constructor.randomSequence[seed1] * 500);
        for (index = _j = 0, _len1 = data.length; _j < _len1; index = ++_j) {
          value = data[index];
          i = (nTile - 1) * this.width + index;
          if (value === -2147483647) {
            arr[i] = NaN;
          } else if (value === -2147483646) {
            arr[i] = 0;
          } else {
            r = this.constructor.randomSequence[rIndex];
            arr[i] = (value - r + 0.5) * scale + zero;
          }
          rIndex += 1;
          if (rIndex === 10000) {
            seed1 = (seed1 + 1) % 10000;
            rIndex = parseInt(this.randomSequence[seed1] * 500);
          }
        }
      }
      return arr;
    };

    CompressedImage.prototype.getFrame = function(nFrame, callback, opts) {
      var heapBlob, reader,
        _this = this;
      if (this.heap) {
        this.frame = nFrame || this.frame;
        return this.getRows(0, this.rows, callback, opts);
      } else {
        heapBlob = this.blob.slice(this.length, this.length + this.heapLength);
        reader = new FileReader();
        reader.onloadend = function(e) {
          _this.heap = e.target.result;
          return _this.getFrame(nFrame, callback, opts);
        };
        return reader.readAsArrayBuffer(heapBlob);
      }
    };

    return CompressedImage;

  })(BinaryTable);

  astro.FITS.CompressedImage = CompressedImage;

  HDU = (function() {
    function HDU(header, data) {
      this.header = header;
      this.data = data;
    }

    HDU.prototype.hasData = function() {
      if (this.data != null) {
        return true;
      } else {
        return false;
      }
    };

    return HDU;

  })();

  astro.FITS.HDU = HDU;

  return astro;

})();


/***/ }),

/***/ "./src/js/libs/healpix.js":
/*!********************************!*\
  !*** ./src/js/libs/healpix.js ***!
  \********************************/
/*! exports provided: Constants, HealpixIndex, SpatialVector */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Constants", function() { return Constants; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "HealpixIndex", function() { return HealpixIndex; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "SpatialVector", function() { return SpatialVector; });
	/**
 	* HEALPix Javascript code derived from the jhealpix Java library
 	* 
 	* Class Constants
 	* 
 	* @author: Thomas Boch [CDS]
 	*/

	 let Constants = {};

	 /** The Constant PI. */
	 Constants.PI = Math.PI;//3.141592653589793238462643383279502884197;
 
	 /** The Constant C_PR. */
	 Constants.C_PR = Math.PI / 180;
 
	 /** The Constant VLEV. */
	 Constants.VLEV = 2;
 
	 /** The Constant EPS. */
	 Constants.EPS = 0.0000001;
 
	 /** The Constant C. */
	 Constants.c = 0.105;
 
	 /** The Constant LN10. */
	 Constants.LN10 = Math.log(10);
 
	 /** The Constant PIOVER2. */
	 Constants.PIOVER2 = Math.PI / 2.;
 
	 /** The Constant TWOPI. */
	 Constants.TWOPI = 2 * Math.PI;//6.283185307179586476925286766559005768394;// 2 * PI
 
	 /** The Constant TWOTHIRD. */
	 Constants.TWOTHIRD = 2. / 3.;
 
	 /** The Constant 1 arcsecond in units of radians. */
	 Constants.ARCSECOND_RADIAN = 4.84813681109536e-6;

/**
 * HEALPix Javascript code derived from the jhealpix Java library
 * 
 * Class HealpixIndex
 * 
 * Main methods :
 * - ang2pix_nest
 * - pix2ang_nest
 * - nest2ring
 * - corners_nest
 * - queryDisc
 * - calculateNSide
 * 
 * @author: Thomas Boch [CDS]
 */


let HealpixIndex = (function () {
	/**
	 * Some utility functions
	 *
	 * @author Thomas Boch [CDS]
	 *
	 */

	let Utils = function () { }

	Utils.radecToPolar = function (ra, dec) {
		return {
			theta: Math.PI / 2. - dec / 180. * Math.PI,
			phi: ra / 180. * Math.PI
		};
	}

	Utils.polarToRadec = function (theta, phi) {
		return {
			ra: phi * 180. / Math.PI,
			dec: (Math.PI / 2. - theta) * 180. / Math.PI
		};
	}


	Utils.castToInt = function (nb) {
		if (nb > 0) {
			return Math.floor(nb);
		}
		else {
			return Math.ceil(nb);
		}
	}

	/**
 * HEALPix Javascript code derived from the jhealpix Java library
	* 
 * Class SpatialVector
	* 
 * @author: Thomas Boch[CDS]
	* /



	/**
 * HEALPix Javascript code derived from the jhealpix Java library
 * 
 * Class AngularPosition
 * 
 * @author: Thomas Boch [CDS]
 */


	let AngularPosition = (function () {

		/** Constructor
		 * 
		 *  @theta theta in radians [0, 2*Pi] 
		 *  @phi phi in radians [0, Pi]
		 */
		function AngularPosition(theta, phi) {
			"use strict";
			this.theta = theta;
			this.phi = phi;
		};

		AngularPosition.prototype.toString = function () {
			"use strict";
			return "theta: " + this.theta + ", phi: " + this.phi;
		};

		return AngularPosition;
	})();

	/**
 * HEALPix Javascript code derived from the jhealpix Java library
 * 
 * Class LongRangeSetBuilder
 * 
 * @author: Thomas Boch [CDS]
 */

	let LongRangeSetBuilder = (function () {
		/* Constructor */
		function LongRangeSetBuilder() {
			this.items = [];
		}

		LongRangeSetBuilder.prototype.appendRange = function (lo, hi) {
			for (var i = lo; i <= hi; i++) {
				if (i in this.items) {
					continue;
				}

				this.items.push(i);
			}
		};
		return LongRangeSetBuilder;
	})();



	/** Constructor */
	function HealpixIndex(nside) {
		"use strict";
		this.nside = nside;
	};

	/** Constants * */
	HealpixIndex.NS_MAX = 16384/*536870912*/;

	HealpixIndex.ORDER_MAX = 14/*29*/;


	/** Available nsides ..always power of 2 ..* */
	HealpixIndex.NSIDELIST = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048,
		4096, 8192, 16384/*, 32768, 65536, 131072, 262144, 524288,
                               1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
                               67108864, 134217728,  268435456, 536870912*/ ];

	// coordinate of the lowest corner of each face
	HealpixIndex.JRLL = [2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
	HealpixIndex.JPLL = [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7];

	HealpixIndex.XOFFSET = [-1, -1, 0, 1, 1, 1, 0, -1];
	HealpixIndex.YOFFSET = [0, 1, 1, 1, 0, -1, -1, -1];
	HealpixIndex.FACEARRAY =
		[[8, 9, 10, 11, -1, -1, -1, -1, 10, 11, 8, 9],   // S
		[5, 6, 7, 4, 8, 9, 10, 11, 9, 10, 11, 8],   // SE
		[-1, -1, -1, -1, 5, 6, 7, 4, -1, -1, -1, -1],   // E
		[4, 5, 6, 7, 11, 8, 9, 10, 11, 8, 9, 10],   // SW
		[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],   // center
		[1, 2, 3, 0, 0, 1, 2, 3, 5, 6, 7, 4],   // NE
		[-1, -1, -1, -1, 7, 4, 5, 6, -1, -1, -1, -1],   // W
		[3, 0, 1, 2, 3, 0, 1, 2, 4, 5, 6, 7],   // NW
		[2, 3, 0, 1, -1, -1, -1, -1, 0, 1, 2, 3]]; // N
	HealpixIndex.SWAPARRAY =
		[[0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3],   // S
		[0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 6, 6],   // SE
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // E
		[0, 0, 0, 0, 0, 0, 0, 0, 5, 5, 5, 5],   // SW
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // center
		[5, 5, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0],   // NE
		[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],   // W
		[6, 6, 6, 6, 0, 0, 0, 0, 0, 0, 0, 0],   // NW
		[3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0]]; // N
	/** The Constant z0. */
	HealpixIndex.Z0 = Constants.TWOTHIRD; // 2/3



	HealpixIndex.prototype.init = function () {
		"use strict";
		var tabmax = 0x100;
		this.ctab = new Array(tabmax);
		this.utab = new Array(tabmax);
		for (var m = 0; m < 0x100; ++m) {
			this.ctab[m] = ((m & 0x1) | ((m & 0x2) << 7) | ((m & 0x4) >> 1) | ((m & 0x8) << 6) |
				((m & 0x10) >> 2) | ((m & 0x20) << 5) | ((m & 0x40) >> 3) | ((m & 0x80) << 4));
			this.utab[m] = ((m & 0x1) | ((m & 0x2) << 1) | ((m & 0x4) << 2) | ((m & 0x8) << 3) |
				((m & 0x10) << 4) | ((m & 0x20) << 5) | ((m & 0x40) << 6) | ((m & 0x80) << 7));
		}

		// end tablefiller
		this.nl2 = 2 * this.nside;
		this.nl3 = 3 * this.nside;
		this.nl4 = 4 * this.nside;
		this.npface = this.nside * this.nside;
		this.ncap = 2 * this.nside * (this.nside - 1);// points in each polar cap,
		// =0 for

		this.npix = 12 * this.npface;
		this.fact2 = 4.0 / this.npix;
		this.fact1 = (this.nside << 1) * this.fact2;

		this.order = HealpixIndex.nside2order(this.nside);
	}

    /**
     * calculate required nside given pixel size in arcsec
     *
     * @param pixsize in arcsec
     * @return long nside parameter
     */
	HealpixIndex.calculateNSide = function (pixsize) {
		var res = 0;
		var pixelArea = pixsize * pixsize;
		var degrad = 180. / Constants.PI;
		var skyArea = 4. * Constants.PI * degrad * degrad * 3600. * 3600.;
		var npixels = Utils.castToInt(skyArea / pixelArea);
		var nsidesq = npixels / 12;
		var nside_req = Math.sqrt(nsidesq);
		var mindiff = HealpixIndex.NS_MAX;
		var indmin = 0;
		for (var i = 0; i < HealpixIndex.NSIDELIST.length; i++) {
			if (Math.abs(nside_req - HealpixIndex.NSIDELIST[i]) <= mindiff) {
				mindiff = Math.abs(nside_req - HealpixIndex.NSIDELIST[i]);
				res = HealpixIndex.NSIDELIST[i];
				indmin = i;
			}
			if ((nside_req > res) && (nside_req < HealpixIndex.NS_MAX))
				res = HealpixIndex.NSIDELIST[indmin + 1];
			if (nside_req > HealpixIndex.NS_MAX) {
				console.log("nside cannot be bigger than " + HealpixIndex.NS_MAX);
				return HealpixIndex.NS_MAX;
			}

		}
		return res;
	}
    /**
     * static method to find order from nside
     * 
     * @param nside
     * @return order
     */
	HealpixIndex.nside2order = function (nside) {
		"use strict";
		if ((nside & (nside - 1)) > 0) {
			return -1;
		}
		return Utils.castToInt(HealpixIndex.log2(nside));
	}

    /**
     * Log base two
     * 
     * @param num
     * @return log2
     */
	HealpixIndex.log2 = function (num) {
		"use strict";
		return (Math.log(num) / Math.log(2));
	}


    /**
     * TESTED. Works OK for nside<=8192
     *
     * renders the pixel number ipix ( scheme as defined for object) for a pixel
     * which contains a point on a sphere at coordinates theta and phi, given the
     * map resolution parameter nside
     * 
     * @param theta
     *            angle (along meridian), in [0,Pi], theta=0 : north pole
     * @param phi
     *            angle (along parallel), in [0,2*Pi]
     * @return pixel index number
     * @throws Exception
     */
	HealpixIndex.prototype.ang2pix_nest = function (theta, phi) {
		"use strict";

		var ipix;
		var z, za, tt, tp;
		var ifp, ifm;
		var jp, jm;
		var ntt, face_num, ix, iy;

		if (phi >= Constants.TWOPI)
			phi = phi - Constants.TWOPI;
		if (phi < 0.)
			phi = phi + Constants.TWOPI;
		if (theta > Constants.PI || theta < 0) {
			throw { name: "Illegal argument", message: "theta must be between 0 and " + Constants.PI };
		}
		if (phi > Constants.TWOPI || phi < 0) {
			throw { name: "Illegal argument", message: "phi must be between 0 and " + Constants.TWOPI };
		}

		z = Math.cos(theta);
		za = Math.abs(z);
		tt = phi / Constants.PIOVER2;// in [0,4]


		if (za <= HealpixIndex.Z0) { // Equatorial region
			// (the index of edge lines increase when the longitude=phi goes up)
			var temp1 = this.nside * (0.5 + tt);
			var temp2 = this.nside * (z * 0.75);

			var jp = (temp1 - temp2);
			// ascending edge line index
			var jm = (temp1 + temp2);
			// descending edge line index

			// finds the face
			ifp = jp >> this.order; // in {0,4}
			ifm = jm >> this.order;
			if (ifp == ifm) { // faces 4 to 7
				face_num = (ifp == 4 ? 4 : ifp + 4);
			} else {
				if (ifp < ifm) { // (half-)faces 0 to 3
					face_num = ifp;
				} else { // (half-)faces 8 to 11
					face_num = ifm + 8;
				}
			}

			ix = Utils.castToInt(jm & (this.nside - 1));
			iy = Utils.castToInt(this.nside - (jp & (this.nside - 1)) - 1);
		} else { // polar region, za > 2/3
			ntt = Utils.castToInt(tt);
			if (ntt >= 4)
				ntt = 3;
			tp = tt - ntt;
			var tmp = this.nside * Math.sqrt(3.0 * (1.0 - za));


			// (the index of edge lines increase when distance from the closest
			// pole goes up)
			jp = Utils.castToInt(tp * tmp);// line going toward the
			// pole as phi increases
			jm = Utils.castToInt((1.0 - tp) * tmp); // that one goes
			// away of the closest pole
			jp = Math.min(HealpixIndex.NS_MAX - 1, jp);
			// for points too close to the boundary
			jm = Math.min(HealpixIndex.NS_MAX - 1, jm);



			// finds the face and pixel's (x,y)
			if (z >= 0) { // North Pole
				// System.out.println("Polar z>=0 ntt:"+ntt+" tt:"+tt);
				face_num = ntt; // in {0,3}
				ix = Utils.castToInt(this.nside - jm - 1);
				iy = Utils.castToInt(this.nside - jp - 1);

			} else {
				// System.out.println("Polar z<0 ntt:"+ntt+" tt:"+tt);
				face_num = ntt + 8;// in {8,11}
				ix = jp;
				iy = jm;
			}
		}

		ipix = this.xyf2nest(ix, iy, face_num);

		return ipix;
	}

	HealpixIndex.prototype.xyf2nest = function (ix, iy, face_num) {
		"use strict";
		return ((face_num) << (2 * this.order)) +
			(((this.utab[ix & 0xff]))
				| ((this.utab[(ix >> 8) & 0xff]) << 16)
				| ((this.utab[(ix >> 16) & 0xff]) << 32)
				| ((this.utab[(ix >> 24) & 0xff]) << 48)
				| ((this.utab[iy & 0xff]) << 1)
				| ((this.utab[(iy >> 8) & 0xff]) << 17)
				| ((this.utab[(iy >> 16) & 0xff]) << 33)
				| ((this.utab[(iy >> 24) & 0xff]) << 49));
	}

	HealpixIndex.prototype.nest2xyf = function (ipix) {
		"use strict";
		var ret = {};
		ret.face_num = ipix >> (2 * this.order);
		var pix = ipix & (this.npface - 1);
		// need o check the & here - they were unsigned in cpp ...
		var raw = (((pix & 0x555500000000) >> 16)
			| ((pix & 0x5555000000000000) >> 31)
			| (pix & 0x5555)
			| ((pix & 0x55550000) >> 15));
		ret.ix = this.ctab[raw & 0xff]
			| (this.ctab[(raw >> 8) & 0xff] << 4)
			| (this.ctab[(raw >> 16) & 0xff] << 16)
			| (this.ctab[(raw >> 24) & 0xff] << 20);
		pix >>= 1;
		raw = (((pix & 0x555500000000) >> 16)
			| ((pix & 0x5555000000000000) >> 31)
			| (pix & 0x5555)
			| ((pix & 0x55550000) >> 15));
		ret.iy = this.ctab[raw & 0xff]
			| (this.ctab[(raw >> 8) & 0xff] << 4)
			| (this.ctab[(raw >> 16) & 0xff] << 16)
			| (this.ctab[(raw >> 24) & 0xff] << 20);

		return ret;
	}

    /**
     * TESTED. Works OK for nside<=8192
     * 
     * Convert from pix number to angle renders theta and phi coordinates of the
     * nominal pixel center for the pixel number ipix (NESTED scheme) given the
     * map resolution parameter nside
     *
     * @param ipix
     *            pixel index number
     * @return double array of [theta, phi] angles in radians [0,Pi], [0,2*Pi]
     * @throws Exception if ipix not in expected range for norder
     */
	HealpixIndex.prototype.pix2ang_nest = function (ipix) {
		"use strict";

		if (ipix < 0 || ipix > this.npix - 1) {
			throw { name: "Illegal argument", message: "ipix out of range" };
		}

		var x = this.nest2xyf(ipix);

		var ix = x.ix;
		var iy = x.iy;
		var face_num = x.face_num;

		// TODO this c++ bit shift givesa differnt jr to the Healpix Code - why ?
		var jr = ((HealpixIndex.JRLL[face_num] << this.order)) - ix - iy - 1;
		var nr, z, kshift;

		// ring number in {1,4*nside-1}

		if (jr < this.nside) { // north pole region
			nr = jr;
			z = 1.0 - nr * nr * this.fact2;
			kshift = 0;
		} else if (jr > this.nl3) { // south pole region
			nr = this.nl4 - jr;
			z = nr * nr * this.fact2 - 1.0;
			kshift = 0;
		} else {
			nr = this.nside;
			z = (this.nl2 - jr) * this.fact1;
			kshift = (jr - this.nside) & 1;
		}
		var theta = Math.acos(z);

		// computes the phi coordinate on the sphere, in [0,2Pi]
		var jp = (HealpixIndex.JPLL[face_num] * nr + ix - iy + 1 + kshift) / 2;
		// 'phi' number in the ring in {1,4*nr}
		if (jp > this.nl4) {
			jp = jp - this.nl4;
		}
		if (jp < 1) {
			jp = jp + this.nl4;
		}

		var phi = (jp - (kshift + 1) * 0.50) * (Constants.PIOVER2 / nr);

		// if (phi < 0)
		// phi += 2.0 * Math.PI; // phi in [0, 2pi]

		return { theta: theta, phi: phi };
	}

	HealpixIndex.nside2Npix = function (nside) {
		"use strict";

		// check if power of 2 and if nside<NS_MAX
		if (nside < 0 || (nside & -nside) != nside || nside > HealpixIndex.NS_MAX) {
			throw { name: "Illegal argument", message: "nside should be >0, power of 2, <" + HealpixIndex.NS_MAX };
		}
		var res = 12 * nside * nside;
		return res;
	}

	HealpixIndex.prototype.xyf2ring = function (ix, iy, face_num) {
		"use strict";

		var jr = HealpixIndex.JRLL[face_num] * this.nside - ix - iy - 1;

		var nr, kshift, n_before;
		if (jr < this.nside) {
			nr = jr;
			n_before = 2 * nr * (nr - 1);
			kshift = 0;
		}
		else if (jr > 3 * this.nside) {
			nr = this.nl4 - jr;
			n_before = this.npix - 2 * (nr + 1) * nr;
			kshift = 0;
		}
		else {
			nr = this.nside;
			n_before = this.ncap + (jr - this.nside) * this.nl4;
			kshift = (jr - this.nside) & 1;
		}

		var jp = (HealpixIndex.JPLL[face_num] * nr + ix - iy + 1 + kshift) / 2;
		if (jp > this.nl4) {
			jp -= this.nl4;
		}
		else {
			if (jp < 1) {
				jp += this.nl4;
			}
		}

		return n_before + jp - 1;
	}

    /**
     * 
     * TESTED. Works OK for nside<=8192
     * 
     * performs conversion from NESTED to RING pixel number
     *
     * @param ipnest
     *            pixel NEST index number
     * @return RING pixel index number
     * @throws Exception
     */
	HealpixIndex.prototype.nest2ring = function (ipnest) {
		"use strict";
		var xyf = this.nest2xyf(ipnest);
		var ipring = this.xyf2ring(xyf.ix, xyf.iy, xyf.face_num);
		return ipring;
	}

    /**
     * 
     * TESTED. Works OK for nside<=8192
     * 
     * Returns set of points along the boundary of the given pixel in NEST
     * scheme. Step 1 gives 4 points on the corners.
     *
     * @param pix
     *            pixel index number in nest scheme
     * @param step
     * @return {@link SpatialVector} for each points
     * @throws Exception
     */
	HealpixIndex.prototype.corners_nest = function (pix, step) {
		"use strict";

		var pixr = this.nest2ring(pix);
		return this.corners_ring(pixr, step);
	}


    /**
     * Convert from pix number to angle renders theta and phi coordinates of the
     * nominal pixel center for the pixel number ipix (RING scheme) given the
     * map resolution parameter nside
     *
     * @param ipix
     *            pixel index number
     * @return double array of [theta, phi] angles in radians [0,Pi], [0,2*Pi]
     * @throws Exception
     */
	HealpixIndex.prototype.pix2ang_ring = function (ipix) {
		"use strict";

		var theta, phi;
		var iring, iphi, ip, ipix1;
		var fodd, hip, fihip;
		// -----------------------------------------------------------------------
		if (ipix < 0 || ipix > this.npix - 1) {
			throw { name: "Illegal argument", message: "ipix out of range" };
		}

		ipix1 = ipix + 1;// in {1, npix}
		if (ipix1 <= this.ncap) { // North Polar cap -------------

			hip = ipix1 / 2.0;
			fihip = Utils.castToInt(hip);
			iring = Utils.castToInt(Math.sqrt(hip - Math.sqrt(fihip))) + 1;
			// counted from North pole
			iphi = ipix1 - 2 * iring * (iring - 1);

			theta = Math.acos(1.0 - (iring * iring * this.fact2));
			phi = ((iphi) - 0.50) * Constants.PI / (2.0 * iring);

		} else {
			if (ipix < (this.npix - this.ncap)) { // Equatorial region
				ip = ipix - this.ncap;
				iring = (ip / this.nl4) + this.nside;// counted from North pole
				iphi = ip % this.nl4 + 1;

				fodd = (((iring + this.nside) & 1) > 0) ? 1 : 0.5;
				// 1 if iring+nside is odd, 1/2 otherwise
				theta = Math.acos((this.nl2 - iring) * this.fact1);
				phi = ((iphi) - fodd) * Constants.PI
					/ this.nl2;
			} else { // South Polar cap -----------------------------------
				ip = this.npix - ipix;
				iring = Utils.castToInt(0.5 * (1 + Math.sqrt(2 * ip - 1)));
				// counted from South pole
				iphi = 4 * iring + 1 - (ip - 2 * iring * (iring - 1));

				theta = Math.acos(-1.0 + Math.pow(iring, 2) * this.fact2);
				phi = ((iphi) - 0.50) * Constants.PI
					/ (2.0 * iring);

			}
		};

		return [theta, phi];
	}

    /**
     * return ring number for given pix in ring scheme
     *
     * @param ipix
     *            pixel index number in ring scheme
     * @return ring number
     * @throws Exception
     */
	HealpixIndex.prototype.ring = function (ipix) {
		"use strict";
		var iring = 0;
		var ipix1 = ipix + 1;// in {1, npix}
		var ip;
		var hip, fihip = 0;
		if (ipix1 <= this.ncap) { // North Polar cap -------------
			hip = (ipix1 / 2.0);
			fihip = Utils.castToInt(hip);
			iring = Utils.castToInt(Math.sqrt(hip - Math.sqrt(fihip))) + 1;// counted
			// from
			// North
			// pole
		} else {
			if (ipix1 <= this.nl2 * (5 * this.nside + 1)) { // Equatorial region
				// ------
				ip = Utils.castToInt(ipix1 - this.ncap - 1);
				iring = Utils.castToInt((ip / this.nl4) + this.nside);// counted from North pole
			} else { // South Polar cap -----------------------------------
				ip = (this.npix - ipix1 + 1);
				hip = (ip / 2.0);
				fihip = Utils.castToInt(hip);
				iring = Utils.castToInt(Math.sqrt(hip - Math.sqrt(fihip))) + 1;// counted
				// from
				// South
				// pole
				iring = (this.nl4 - iring);
			}
		}
		return iring;
	}

    /**
     * integration limits in cos(theta) for a given ring i_th, i_th > 0
     *
     * @param i_th
     *            ith ring
     * @return limits
     */
	HealpixIndex.prototype.integration_limits_in_costh = function (i_th) {
		"use strict";
		var a, ab, b, r_n_side;
		// integration limits in cos(theta) for a given ring i_th
		// i > 0 !!!
		r_n_side = 1.0 * this.nside;
		if (i_th <= this.nside) {
			ab = 1.0 - (Math.pow(i_th, 2.0) / 3.0) / this.npface;
			b = 1.0 - (Math.pow((i_th - 1), 2.0) / 3.0) / this.npface;
			if (i_th == this.nside) {
				a = 2.0 * (this.nside - 1.0) / 3.0 / r_n_side;
			} else {
				a = 1.0 - Math.pow((i_th + 1), 2) / 3.0 / this.npface;
			}
		} else {
			if (i_th < this.nl3) {
				ab = 2.0 * (2 * this.nside - i_th) / 3.0 / r_n_side;
				b = 2.0 * (2 * this.nside - i_th + 1) / 3.0 / r_n_side;
				a = 2.0 * (2 * this.nside - i_th - 1) / 3.0 / r_n_side;
			} else {
				if (i_th == this.nl3) {
					b = 2.0 * (-this.nside + 1) / 3.0 / r_n_side;
				} else {
					b = -1.0 + Math.pow((4 * this.nside - i_th + 1), 2) / 3.0
						/ this.npface;
				}

				a = -1.0 + Math.pow((this.nl4 - i_th - 1), 2) / 3.0
					/ this.npface;
				ab = -1.0 + Math.pow((this.nl4 - i_th), 2) / 3.0 / this.npface;
			}

		}
		// END integration limits in cos(theta)
		return [b, ab, a];
	}

    /**
     * calculate the points of crossing for a given theata on the boundaries of
     * the pixel - returns the left and right phi crossings
     *
     * @param i_th
     *            ith pixel
     * @param i_phi
     *            phi angle
     * @param i_zone
     *            ith zone (0,...,3), a quarter of sphere
     * @param cos_theta
     *            theta cosinus
     * @return the left and right phi crossings
     */
	HealpixIndex.prototype.pixel_boundaries = function (i_th, i_phi, i_zone, cos_theta) {
		var sq3th, factor, jd, ju, ku, kd, phi_l, phi_r;
		var r_n_side = 1.0 * this.nside;

		// HALF a pixel away from both poles
		if (Math.abs(cos_theta) >= 1.0 - 1.0 / 3.0 / this.npface) {
			phi_l = i_zone * Constants.PIOVER2;
			phi_r = (i_zone + 1) * Constants.PIOVER2;
			return [phi_l, phi_r];
		}
		// -------
		// NORTH POLAR CAP
		if (1.50 * cos_theta >= 1.0) {
			sq3th = Math.sqrt(3.0 * (1.0 - cos_theta));
			factor = 1.0 / r_n_side / sq3th;
			jd = (i_phi);
			ju = jd - 1;
			ku = (i_th - i_phi);
			kd = ku + 1;

			phi_l = Constants.PIOVER2
				* (Math.max((ju * factor), (1.0 - (kd * factor))) + i_zone);
			phi_r = Constants.PIOVER2
				* (Math.min((1.0 - (ku * factor)), (jd * factor)) + i_zone);

		} else {
			if (-1.0 < 1.50 * cos_theta) {
				// -------
				// -------
				// EQUATORIAL ZONE
				var cth34 = 0.50 * (1.0 - 1.50 * cos_theta);
				var cth34_1 = cth34 + 1.0;
				var modfactor = (this.nside + (i_th % 2));

				jd = i_phi - (modfactor - i_th) / 2.0;
				ju = jd - 1;
				ku = (modfactor + i_th) / 2.0 - i_phi;
				kd = ku + 1;

				phi_l = Constants.PIOVER2
					* (Math.max((cth34_1 - (kd / r_n_side)),
						(-cth34 + (ju / r_n_side))) + i_zone);

				phi_r = Constants.PIOVER2
					* (Math.min((cth34_1 - (ku / r_n_side)),
						(-cth34 + (jd / r_n_side))) + i_zone);
				// -------
				// -------
				// SOUTH POLAR CAP

			} else {
				sq3th = Math.sqrt(3.0 * (1.0 + cos_theta));
				factor = 1.0 / r_n_side / sq3th;
				var ns2 = 2 * this.nside;

				jd = i_th - ns2 + i_phi;
				ju = jd - 1;
				ku = ns2 - i_phi;
				kd = ku + 1;

				phi_l = Constants.PIOVER2
					* (Math.max((1.0 - (ns2 - ju) * factor),
						((ns2 - kd) * factor)) + i_zone);

				phi_r = Constants.PIOVER2
					* (Math.min((1.0 - (ns2 - jd) * factor),
						((ns2 - ku) * factor)) + i_zone);
			}// of SOUTH POLAR CAP
		}
		// and that's it
		// System.out.println(" nside:"+nside+" i_th:"+i_th+" i_phi:"+i_phi+"
		// izone:"+i_zone+" cos_theta:"+cos_theta+" phi_l:"+phi_l+"
		// phi_r:"+phi_r);

		return [phi_l, phi_r];
	}

    /**
     * Construct a {@link SpatialVector} from the angle (theta,phi)
     *
     * @param theta
     *            angle (along meridian), in [0,Pi], theta=0 : north pole
     * @param phi
     *            angle (along parallel), in [0,2*Pi]
     * @return vector {@link SpatialVector}
     */
	HealpixIndex.vector = function (theta, phi) {
		"use strict";
		var x = 1 * Math.sin(theta) * Math.cos(phi);
		var y = 1 * Math.sin(theta) * Math.sin(phi);
		var z = 1 * Math.cos(theta);
		return new SpatialVector(x, y, z);
	}

    /**
     * Returns set of points along the boundary of the given pixel in RING
     * scheme. Step 1 gives 4 points on the corners.
     * Mainly for graphics = you may not want to use LARGE NSIDEs..
     *
     * @param pix
     *            pixel index number in ring scheme
     * @param step
     * @return {@link SpatialVector} for each points
     * @throws Exception
     */
	HealpixIndex.prototype.corners_ring = function (pix, step) {
		"use strict";

		var nPoints = step * 2 + 2;
		var points = new Array(nPoints);
		var p0 = this.pix2ang_ring(pix);
		var cos_theta = Math.cos(p0[0]);
		var theta = p0[0];
		var phi = p0[1];

		var i_zone = Utils.castToInt(phi / Constants.PIOVER2);
		var ringno = this.ring(pix);
		var i_phi_count = Math.min(ringno, Math.min(this.nside, (this.nl4) - ringno));
		var i_phi = 0;
		var phifac = Constants.PIOVER2 / i_phi_count;
		if (ringno >= this.nside && ringno <= this.nl3) {
			// adjust by 0.5 for odd numbered rings in equatorial since
			// they start out of phase by half phifac.
			i_phi = Utils.castToInt(phi / phifac + ((ringno % 2) / 2.0)) + 1;
		} else {
			i_phi = Utils.castToInt(phi / phifac) + 1;
		}
		// adjust for zone offset
		i_phi = i_phi - (i_zone * i_phi_count);
		var spoint = (nPoints / 2);

		// get north south middle - middle should match theta !
		var nms = this.integration_limits_in_costh(ringno);
		var ntheta = Math.acos(nms[0]);
		var stheta = Math.acos(nms[2]);
		var philr = this.pixel_boundaries(ringno, i_phi, i_zone, nms[0]);

		if (i_phi > (i_phi_count / 2)) {
			points[0] = HealpixIndex.vector(ntheta, philr[1]);
		} else {
			points[0] = HealpixIndex.vector(ntheta, philr[0]);
		}
		philr = this.pixel_boundaries(ringno, i_phi, i_zone, nms[2]);
		if (i_phi > (i_phi_count / 2)) {
			points[spoint] = HealpixIndex.vector(stheta, philr[1]);
		} else {
			points[spoint] = HealpixIndex.vector(stheta, philr[0]);
		}
		if (step == 1) {
			var mtheta = Math.acos(nms[1]);
			philr = this.pixel_boundaries(ringno, i_phi, i_zone, nms[1]);
			points[1] = HealpixIndex.vector(mtheta, philr[0]);
			points[3] = HealpixIndex.vector(mtheta, philr[1]);
		} else {
			var cosThetaLen = nms[2] - nms[0];
			var cosThetaStep = (cosThetaLen / (step + 1)); // skip
			// North
			// and south
			for (var p = 1; p <= step; p++) {
				/* Integrate points along the sides */
				cos_theta = nms[0] + (cosThetaStep * p);
				theta = Math.acos(cos_theta);
				philr = this.pixel_boundaries(ringno, i_phi, i_zone, cos_theta);
				points[p] = HealpixIndex.vector(theta, philr[0]);
				points[nPoints - p] = HealpixIndex.vector(theta, philr[1]);
			}
		}
		return points;
	}

    /**
     * converts a SpatialVector in a tuple of angles tup[0] = theta co-latitude
     * measured from North pole, in [0,PI] radians, tup[1] = phi longitude
     * measured eastward, in [0,2PI] radians
     *
     * @param v
     *            SpatialVector
     * @return double[] out_tup out_tup[0] = theta out_tup[1] = phi
     */
	HealpixIndex.vec2Ang = function (v) {
		"use strict";

		var z = v.z / v.length();
		var theta = Math.acos(z);
		var phi = 0.;
		if ((v.x != 0.) || (v.y != 0)) {
			phi = Math.atan2(v.y, v.x); // phi in [-pi,pi]
		}
		if (phi < 0)
			phi += 2.0 * Math.PI; // phi in [0, 2pi]
		return [theta, phi];
	}

    /**
     * generates in the RING or NESTED scheme all pixels that lies within an
     * angular distance Radius of the center.
     *
     * TESTED. Works OK for nside<=8192
     *
     * @param nside
     *            long map resolution
     * @param vector
     *            Vector3d pointing to the disc center
     * @param radius
     *            double angular radius of the disk (in RADIAN )
     * @param do_nest
     *            if true, output in NESTED scheme
     *            if false, output in RING scheme
     * @param do_inclusive
     *            if set to false: only pixels whose center lie in the triangle
     *            are listed, if set to true, all pixels overlapping the triangle
     *            are listed
     * @return ArrayList of pixel numbers calls: RingNum(nside, ir)
     *         InRing(nside, iz, phi0, dphi,nest)
     */
	HealpixIndex.prototype.queryDisc = function (vector, radius, do_nest, do_inclusive) {
		"use strict";

		if (radius < 0.0 || radius > Constants.PI) {
			throw { "name": "Illegal argument", "message": "angular radius is in RADIAN and should be in [0,pi]" };
		}

		var res = new LongRangeSetBuilder();
		var irmin, irmax, iz;
		var ang = null;
		var z0, radius_eff, theta, phi, cosang, x, ysq;
		var dth1, dth2, dphi;
		var rlat1, rlat2, zmin, zmax, z, xa;

		var radius_eff = radius;
		if (do_inclusive) {
			radius_eff += Constants.PI / (this.nl4); // increase radius by
			// half pixel: different in C++ version where a 'magic' number is used.
		}

		// this pix back abnf fourth is ok until you put in  precise vector like a pole .
		// then it shifts the whole elipse...
		ang = HealpixIndex.vec2Ang(vector);

		theta = ang[0];
		phi = ang[1];
		dth1 = this.fact2;
		dth2 = this.fact1;
		z0 = Math.cos(theta);
		xa = 1. / Math.sqrt((1.0 - z0) * (1.0 + z0));

		/* coordinate z of highest and lowest points in the disc */

		rlat1 = theta - radius_eff;
		rlat2 = theta + radius_eff;


		cosang = Math.cos(radius_eff);
		zmax = Math.cos(rlat1);
		irmin = this.ringAbove(zmax) + 1;
		zmin = Math.cos(rlat2);
		irmax = this.ringAbove(zmin);

		if (irmax < irmin) {// in this case no pixels are returned - need irmax=irmin to loop
			if (irmax == 0) {
				irmax = irmin;
			}
		}

		if (rlat1 <= 0) {// north pole in the disc
			for (var m = 1; m < irmin; ++m) {// rings completely in the disc
				this.inRing(m, 0, Math.PI, res);
			}
		}

		/* loop on ring number */
		for (iz = irmin; iz <= irmax; ++iz) {
			if (iz < this.nside) { // north polar cap
				z = 1.0 - iz * iz * dth1;
			} else if (iz <= (this.nl3)) { // tropical band + equator
				z = (this.nl2 - iz) * dth2;
			} else {
				z = -1.0 + (this.nl4 - iz) * (this.nl4 - iz) * dth1;
			}
			/* find phi range in the disc for each z */
			x = (cosang - z * z0) * xa;
			ysq = 1.0 - z * z - x * x;
			// up north (and south ?) this atan does not work
			// dphi becomes NaN.
			dphi = Math.atan2(Math.sqrt(ysq), x);
			if (isNaN(dphi)) {
				dphi = radius_eff;
			}
			this.inRing(iz, phi, dphi, res);

		}
		if (rlat2 >= Math.PI) {// south pole in the disc
			for (var m = irmax + 1; m < (this.nl4); ++m) {
				// rings completely in the disc
				this.inRing(m, 0, Math.PI, res, false);
			}
		}

		var ret;
		if (do_nest) {
			var items = res.items;
			var items_nest = [];
			for (var i = 0; i < items.length; i++) {
				var nestIdx = this.ring2nest(items[i]);
				if (items_nest.indexOf(nestIdx) >= 0) {
					continue;
				}
				items_nest.push(nestIdx);
			}
			ret = items_nest;
		}
		else {
			ret = res.items;
		}

		return ret;

	}

    /**
     * returns the list of pixels in RING scheme with latitude in [phi0 -
     * dpi, phi0 + dphi] on the ring iz in [1, 4*nside -1 ] The pixel id numbers
     * are in [0, 12*nside^2 - 1] the indexing is in RING, unless nest is set to
     * 1
     * NOTE: this is the f90 code 'in_ring' method ported to java with 'conservative' flag to false
     *
     * @param nside
     *            long the map resolution
     * @param iz
     *            long ring number
     * @param phi0
     *            double
     * @param dphi
     *            double
     * @param res result
     */
	HealpixIndex.prototype.inRing = function (iz, phi0, dphi, res, conservative) {
		"use strict";

		var take_all = false;
		var to_top = false;

		//	String SID = "InRing:";
		var epsilon = 1e-12;//Double.MIN_VALUE; // the constant to eliminate
		// java calculation jitter
		var shift = 0.;
		var ir = 0;
		var kshift, nr, ipix1, ipix2;//nir1, nir2,
		var ip_low = 0, ip_hi = 0; //,in, nir;
		//	long inext;

		var phi_low = ((phi0 - dphi) % Constants.TWOPI) - epsilon; // phi min,															  // excluding
		// 2pi period
		//	double phi_low = phi0 - dphi - epsilon; // phi min,
		// excluding
		var phi_hi = phi0 + dphi + epsilon;

		// this was being moduloed but why ?? around the 2pi that casues a problem
		var phi_hi_mod = ((phi0 + dphi) % Constants.TWOPI) + epsilon;

		//
		if (Math.abs(dphi - Constants.PI) < epsilon) {
			take_all = true;
		}
		// what happens when phi_hi wraps round ??

		/* identifies ring number */
		if ((iz >= this.nside) && (iz <= this.nl3)) { // equatorial region
			ir = iz - this.nside + 1; // in [1, 2*nside + 1]
			ipix1 = this.ncap + this.nl4 * (ir - 1); // lowest pixel number in the
			// ring
			ipix2 = ipix1 + this.nl4 - 1; // highest pixel number in the ring
			kshift = ir % 2;

			nr = this.nl4;
		}
		else {
			if (iz < this.nside) { // north pole
				ir = iz;
				ipix1 = 2 * ir * (ir - 1); // lowest pixel number
				ipix2 = ipix1 + (4 * ir) - 1; // highest pixel number
			} else { // south pole
				ir = 4 * this.nside - iz;

				ipix1 = this.npix - 2 * ir * (ir + 1); // lowest pixel number
				ipix2 = ipix1 + 4 * ir - 1;       // highest pixel number
			}
			nr = ir * 4;
			kshift = 1;
		}

		// Construct the pixel list
		if (take_all) {
			res.appendRange(ipix1, ipix2);
			return;
		}

		shift = kshift / 2.0;

		// conservative : include every intersected pixel, even if the
		// pixel center is out of the [phi_low, phi_hi] region
		if (conservative) {
			ip_low = Math.round((nr * phi_low) / Constants.TWOPI - shift);
			ip_hi = Math.round((nr * phi_hi) / Constants.TWOPI - shift);

			ip_low = (ip_low % nr); // in [0, nr - 1]
			if (ip_hi > nr) { // ifit is =nr then this sets it to zero - not good
				ip_hi = (ip_hi % nr); // in [0, nr - 1]
			}
			//		System.out.println("ip_low="+ip_low+" ip_hi="+ip_hi);
		}
		else { // strict: includes only pixels whose center is in
			//                                                    [phi_low,phi_hi]

			ip_low = Math.ceil((nr * phi_low) / Constants.TWOPI - shift);
			ip_hi = Utils.castToInt((nr * phi_hi_mod) / Constants.TWOPI - shift);
			if (ip_hi < ip_low && iz == 1) {//this is not good - problem on pole with direction.
				ip_hi = Utils.castToInt((nr * phi_hi) / Constants.TWOPI - shift);
			}
			if (ip_low == ip_hi + 1) {
				ip_low = ip_hi;
			}

			if ((ip_low - ip_hi == 1) && (dphi * nr < Constants.PI)) {
				// the interval is too small ( and away from pixel center)
				// so no pixels is included in the list

				console.log("the interval is too small and avay from center");

				return; // return empty list
			}

			ip_low = Math.min(ip_low, nr - 1);
			ip_hi = Math.max(ip_hi, 0);
		}

		//
		if (ip_low > ip_hi) {
			to_top = true;
		}

		if (to_top) {
			ip_low += ipix1;
			ip_hi += ipix1;

			res.appendRange(ipix1, ip_hi);
			res.appendRange(ip_low, ipix2);
		} else {
			if (ip_low < 0) {
				ip_low = Math.abs(ip_low);

				res.appendRange(ipix1, ipix1 + ip_hi);
				res.appendRange(ipix2 - ip_low + 1, ipix2);
				return;

			}
			ip_low += ipix1;
			ip_hi += ipix1;

			res.appendRange(ip_low, ip_hi);
		}
	}

	HealpixIndex.prototype.ringAbove = function (z) {
		"use strict";

		var az = Math.abs(z);
		if (az > Constants.TWOTHIRD) { // polar caps
			var iring = Utils.castToInt(this.nside * Math.sqrt(3 * (1 - az)));
			return (z > 0) ? iring : 4 * this.nside - iring - 1;
		}
		else { // ----- equatorial region ---------
			return Utils.castToInt(this.nside * (2.0 - 1.5 * z));
		}
	}

	HealpixIndex.prototype.ring2nest = function (ipring) {
		"use strict";

		var xyf = this.ring2xyf(ipring);
		return this.xyf2nest(xyf.ix, xyf.iy, xyf.face_num);
	}

	HealpixIndex.prototype.ring2xyf = function (pix) {
		"use strict";

		var ret = {};
		var iring, iphi, kshift, nr;

		if (pix < this.ncap) { // North Polar cap
			iring = Utils.castToInt(0.5 * (1 + Math.sqrt(1 + 2 * pix))); //counted from North pole
			iphi = (pix + 1) - 2 * iring * (iring - 1);
			kshift = 0;
			nr = iring;
			ret.face_num = 0;
			var tmp = iphi - 1;
			if (tmp >= (2 * iring)) {
				ret.face_num = 2;
				tmp -= 2 * iring;
			}
			if (tmp >= iring) {
				++ret.face_num;
			}
		}
		else if (pix < (this.npix - this.ncap)) { // Equatorial region
			var ip = pix - this.ncap;
			if (this.order >= 0) {
				iring = (ip >> (this.order + 2)) + this.nside; // counted from North pole
				iphi = (ip & (this.nl4 - 1)) + 1;
			}
			else {
				iring = (ip / (this.nl4)) + this.nside; // counted from North pole
				iphi = (ip % (this.nl4)) + 1;
			}
			kshift = (iring + this.nside) & 1;
			nr = this.nside;
			var ire = iring - this.nside + 1;
			var irm = this.nl2 + 2 - ire;
			var ifm, ifp;
			if (this.order >= 0) {
				ifm = (iphi - Utils.castToInt(ire / 2) + this.nside - 1) >> this.order;
				ifp = (iphi - Utils.castToInt(irm / 2) + this.nside - 1) >> this.order;
			}
			else {
				ifm = (iphi - Utils.castToInt(ire / 2) + this.nside - 1) / this.nside;
				ifp = (iphi - Utils.castToInt(irm / 2) + this.nside - 1) / this.nside;
			}
			if (ifp == ifm) { // faces 4 to 7
				ret.face_num = (ifp == 4) ? 4 : Utils.castToInt(ifp) + 4;
			}
			else if (ifp < ifm) { // (half-)faces 0 to 3
				ret.face_num = Utils.castToInt(ifp);
			}
			else { // (half-)faces 8 to 11
				ret.face_num = Utils.castToInt(ifm) + 8;
			}
		}
		else { // South Polar cap
			var ip = this.npix - pix;
			iring = Utils.castToInt(0.5 * (1 + Math.sqrt(2 * ip - 1))); //counted from South pole
			iphi = 4 * iring + 1 - (ip - 2 * iring * (iring - 1));
			kshift = 0;
			nr = iring;
			iring = 2 * this.nl2 - iring;
			ret.face_num = 8;
			var tmp = iphi - 1;
			if (tmp >= (2 * nr)) {
				ret.face_num = 10;
				tmp -= 2 * nr;
			}
			if (tmp >= nr) {
				++ret.face_num;
			}
		}

		var irt = iring - (HealpixIndex.JRLL[ret.face_num] * this.nside) + 1;
		var ipt = 2 * iphi - HealpixIndex.JPLL[ret.face_num] * nr - kshift - 1;
		if (ipt >= this.nl2) {
			ipt -= 8 * this.nside;
		}


		ret.ix = ((ipt - irt) >> 1);
		ret.iy = ((-(ipt + irt)) >> 1);

		return ret;
	};

	HealpixIndex.utils = Utils;

	return HealpixIndex;
})();


/**
 * The SpatialVector contains standard 3D vector with the addition that each
 * coordinate (x,y,z) is also kept in ra,dec since we expect the vector to live
 * on the surface of the unit sphere, i.e.
 * 
 * <pre>
 *  2   2   2
 *  x + y + z  = 1
 * </pre>
 * 
 * This is not enforced, so you can specify a vector that has not unit length.
 * If you request the ra/dec of such a vector, it will be automatically
 * normalized to length 1 and you get the ra/dec of that vector (the
 * intersection of the vector's direction with the unit sphere.
 * 
 * This code comes originally from the HTM library of Peter Kunst during his
 * time at JHU.
 */


let SpatialVector = (function () {

	/**
	 * Constructor from three coordinates
	 * 
	 * @param x
	 * @param y
	 * @param z
	 */
	function SpatialVector(x, y, z) {
		"use strict";
		this.x = x;
		this.y = y;
		this.z = z;
		this.ra_ = 0;
		this.dec_ = 0;
		this.okRaDec_ = false;
	}
	;
	SpatialVector.prototype.setXYZ = function (x, y, z) {
		this.x = x;
		this.y = y;
		this.z = z;
		this.okRaDec_ = false;
	};

	/**
	 * Returns the length of this vector.
	 * 
	 * @return the length of this vector
	 */
	SpatialVector.prototype.length = function () {
		"use strict";
		return Math.sqrt(this.lengthSquared());
	};

	/**
	 * Returns the squared length of this vector.
	 * 
	 * @return the squared length of this vector
	 */
	SpatialVector.prototype.lengthSquared = function () {
		"use strict";
		return this.x * this.x + this.y * this.y + this.z * this.z;
	};

	/**
	 * Normalized this vector
	 */
	SpatialVector.prototype.normalized = function () {
		"use strict";
		var d = this.length();
		// zero-div may occur.
		this.x /= d;
		this.y /= d;
		this.z /= d;
	};

	/**
	 * Sets the ra and dec angles in degrees
	 * 
	 * @param ra
	 *            right ascension angle in degrees
	 * @param dec
	 *            declination angle in degrees
	 * 
	 */
	SpatialVector.prototype.set = function (ra, dec) {
		"use strict";
		this.ra_ = ra;
		this.dec_ = dec;
		this.okRaDec_ = true;
		this.updateXYZ();
	};

	/**
	 * Returns the angle in radians between this vector and the vector
	 * parameter; the return value is constrained to the range [0,PI].
	 * 
	 * @param v1
	 *            the other vector
	 * @return the angle in radians in the range [0,PI]
	 */
	SpatialVector.prototype.angle = function (v1) {
		"use strict";
		// return (double)Math.acos(dot(v1)/v1.length()/v.length());
		// Numerically, near 0 and PI are very bad condition for acos.
		// In 3-space, |atan2(sin,cos)| is much stable.
		var xx = this.y * v1.z - this.z * v1.y;
		var yy = this.z * v1.x - this.x * v1.z;
		var zz = this.x * v1.y - this.y * v1.x;
		var cross = Math.sqrt(xx * xx + yy * yy + zz * zz);
		return Math.abs(Math.atan2(cross, dot(v1)));
	};

	/**
	 * Get the coordinates in a 3 elements 1D array
	 * 
	 * @return coordinates [x,y,z]
	 */
	SpatialVector.prototype.get = function () {
		"use strict";
		return [x, y, z];
	};

	SpatialVector.prototype.toString = function () {
		"use strict";
		return "SpatialVector[" + this.x + ", " + this.y + ", " + this.z + "]";
	};

	/**
	 * vector cross product
	 * 
	 * @param v
	 *            the vector to cross
	 * @return the vector cross product
	 */
	SpatialVector.prototype.cross = function (v) {
		"use strict";
		return new SpatialVector(this.y * v.z - v.y * this.z, this.z * v.x - v.z * this.x, this.x * v.y - v.x() * this.y);
	};

	/**
	 * Compare vectors if coordinates are equals
	 * 
	 * @param v
	 *            the vector to be compared with
	 * @return true if both coordinates of vectors are equal
	 */
	SpatialVector.prototype.equal = function (v) {
		"use strict";
		return ((this.x == v.x && this.y == v.y && this.z == v.z()) ? true : false);
	};


	/**
	 * multiply with a number
	 * 
	 * @param n
	 *            the scale number to be multiply to the coordinates x,y,z
	 * @return the vector with coordinates multiplied by n
	 */
	SpatialVector.prototype.mult = function (n) {
		"use strict";
		return new SpatialVector((n * this.x), (n * this.y), (n * this.z));
	};

	/**
	 * Computes the dot product of the this vector and vector v1.
	 * 
	 * @param v1
	 *            the other vector
	 * @return dot product
	 */
	SpatialVector.prototype.dot = function (v1) {
		"use strict";
		return this.x * v1.x + this.y * v1.y + this.z * v1.z;
	};

	/**
	 * vector addition
	 * 
	 * @param v
	 *            the vector to be added
	 * @return vector result by addition
	 */
	SpatialVector.prototype.add = function (v) {
		"use strict";
		return new SpatialVector(this.x + v.x, this.y + v.y, this.z + v.z);
	};

	/**
	 * vector subtraction
	 * 
	 * @param v
	 *            the vector to be substracted
	 * @return vector result by substraction
	 */
	SpatialVector.prototype.sub = function (v) {
		"use strict";
		return new SpatialVector(this.x - v.x, this.y - v.y, this.z - v.z);
	};

	/**
	 * Get the dec angle in degrees
	 * 
	 * @return declination angle
	 */
	SpatialVector.prototype.dec = function () {
		"use strict";
		if (!this.okRaDec_) {
			this.normalized();
			this.updateRaDec();
		}
		return this.dec_;
	};

	/**
	 * Get the ra angle in degrees
	 * 
	 * @return right ascension
	 */
	SpatialVector.prototype.ra = function () {
		"use strict";
		if (!this.okRaDec_) {
			this.normalized();
			this.updateRaDec();
		}
		return this.ra_;
	};

	/**
	 * Update x_ y_ z_ from ra_ and dec_ variables
	 */
	SpatialVector.prototype.updateXYZ = function () {
		"use strict";
		var cd = Math.cos(this.dec_ * Constants.C_PR);
		this.x = Math.cos(this.ra_ * Constants.C_PR) * cd;
		this.y = Math.sin(this.ra_ * Constants.C_PR) * cd;
		this.z = Math.sin(this.dec_ * Constants.C_PR);
	};

	/**
	 * Update ra_ and dec_ from x_ y_ z_ variables
	 */
	SpatialVector.prototype.updateRaDec = function () {
		"use strict";
		this.dec_ = Math.asin(this.z) / Constants.C_PR; // easy.
		var cd = Math.cos(this.dec_ * Constants.C_PR);
		if (cd > Constants.EPS || cd < -Constants.EPS) {
			if (this.y > Constants.EPS || this.y < -Constants.EPS) {
				if (this.y < 0.0) {
					this.ra_ = 360 - Math.acos(this.x / cd) / Constants.C_PR;
				}
				else {
					this.ra_ = Math.acos(this.x / cd) / Constants.C_PR;
				}
			} else {
				this.ra_ = (this.x < 0.0 ? 180 : 0.0);
			}
		}
		else {
			this.ra_ = 0.0;
		}
		this.okRaDec_ = true;
	};

	/**
	 * @return Right Ascension of this vector in radians
	 */
	SpatialVector.prototype.toRaRadians = function () {
		"use strict";
		var phi = 0.;
		if ((this.x != 0.) || (this.y != 0)) {
			phi = Math.atan2(this.y, this.x); // phi in [-pi,pi]
		}

		if (phi < 0) {
			phi += 2.0 * Math.PI; // phi in [0, 2pi]
		}

		return phi;
	};

	/**
	 * @return Declination of this vector in radians
	 */
	SpatialVector.prototype.toDeRadians = function () {
		var z2 = z / this.length();
		var theta = Math.acos(z2);
		return Math.PI / 2 - theta;
	};

	return SpatialVector;
})();

/***/ })

/******/ });
//# sourceMappingURL=aladin.js.map