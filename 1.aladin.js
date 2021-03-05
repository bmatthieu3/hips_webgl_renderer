(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./node_modules/webpack/buildin/global.js":
/*!***********************************!*\
  !*** (webpack)/buildin/global.js ***!
  \***********************************/
/*! no static exports found */
/***/ (function(module, exports) {

var g;

// This works in non-strict mode
g = (function() {
	return this;
})();

try {
	// This works if eval is allowed (see CSP)
	g = g || new Function("return this")();
} catch (e) {
	// This works if the window reference is available
	if (typeof window === "object") g = window;
}

// g can still be undefined, but nothing to do about it...
// We return undefined, instead of nothing here, so it's
// easier to handle this case. if(!global) { ...}

module.exports = g;


/***/ }),

/***/ "./src/core/pkg/index.js":
/*!*******************************!*\
  !*** ./src/core/pkg/index.js ***!
  \*******************************/
/*! exports provided: GALCooSys, ICRSJ2000CooSys, CooSystem, WebClient, __wbindgen_json_serialize, __wbindgen_object_drop_ref, __wbg_canvas_dd578e51a2bc736f, __wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430, __wbg_setwidth_41b2497107faaff7, __wbg_setheight_e15cb9243262e701, __wbg_viewport_86b156d5858adab9, __wbg_scissor_1f78ef0050a93516, __wbindgen_string_new, __wbindgen_memory, __wbg_buffer_e35e010c3ba9f945, __wbg_new_fe24eae01e10f223, __wbg_subarray_3c6f7cfb4edcc351, __wbg_measureText_2a4b2ca71061d96c, __wbg_width_979b596f39ba8319, __wbg_bindVertexArray_520c05423d3d6641, __wbg_bindBuffer_4a7874f09df12419, __wbg_bufferSubData_51f29e78449b2095, __wbg_bufferData_80963d2bd1ecb1bc, __wbg_performance_800ff37c906b5f3b, __wbg_now_9f22124bc74da886, __wbg_activeTexture_32edab6336bd38a9, __wbg_bindTexture_d659843380f373b5, __wbg_texSubImage2D_e13399a16dfb0646, __wbg_blendFuncSeparate_13c318610edadb4a, __wbg_enable_65590f4951fd0112, __wbg_cullFace_b0941c23a53ee9fc, __wbg_innerWidth_aab6ec3242dff39e, __wbindgen_number_get, __wbg_innerHeight_7e514d9823f7864e, __wbg_new_17bf587bb9ce55f1, __wbg_setcrossOrigin_054bb95c5a2b2640, __wbg_new_4f8fb2c75215d83a, __wbg_setresponseType_09ae5e5481a8947d, __wbg_createTexture_8ba2e566eb313fcf, __wbindgen_object_clone_ref, __wbg_setonload_69f9426b613d7bd2, __wbg_setonerror_e519d2d2cbd89b1d, __wbg_setsrc_8742008d92b4e70e, __wbg_texParameteri_c0b2b665319f6a16, __wbg_texImage2D_a5dad82b8f689bbd, __wbg_createFramebuffer_edeb035499d73077, __wbg_bindFramebuffer_abbc9985c473f160, __wbg_framebufferTexture2D_bb45b3c3d234ddcd, __wbg_createVertexArray_5cbff3d8bbe1c324, __wbg_useProgram_b1cc885b00b8f52c, __wbg_createBuffer_4302ddbcbfc99048, __wbg_lineWidth_a2c6059f833032d4, __wbg_vertexAttribPointer_3bb013e284cd07bf, __wbg_enableVertexAttribArray_413ef49912a23f9e, __wbg_document_d8cce4c1031c64eb, __wbg_getElementsByClassName_a5ef560ae6918226, __wbg_getwithindex_bcf1a04b716019a9, __wbg_setAttribute_fb8737b4573a65f8, __wbg_getContext_d277f710e8035242, __wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3, __wbg_scale_e0fdce059098cd1b, __wbg_texImage2D_bc294af8c1a6a435, __wbg_log_a39f164b49616cb0, __wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8, __wbg_new_c77df81d6c892c35, __wbg_newwithbyteoffsetandlength_836859e5deb44d3f, __wbg_new_139e70222494b1ff, __wbg_newwithbyteoffsetandlength_c274c3296a37fcb4, __wbg_new_5b74a8dd0c5b71ac, __wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1, __wbg_deleteBuffer_988823f4e76e697d, __wbg_createProgram_128698dd90ec070d, __wbg_attachShader_5d53b7b00823cafb, __wbg_linkProgram_370ed11b34456c89, __wbg_getProgramParameter_b949ba1d9662f6a2, __wbindgen_boolean_get, __wbg_getProgramInfoLog_f8f65be65281f691, __wbg_getActiveUniform_70b770a58f551f8f, __wbg_name_4f3b7294acbeabad, __wbg_getUniformLocation_472b7459010900a5, __wbg_createShader_26e4f959d5d64d80, __wbg_shaderSource_96ace5133c032f2f, __wbg_compileShader_f7e245515fa1405d, __wbg_getShaderParameter_cced0ff8ba83f3e7, __wbg_getShaderInfoLog_5412e8bc642139e8, __wbg_clearColor_fc22409197a5bd68, __wbg_clear_25e035ed3961f1c6, __wbg_width_d9e3643c351ff015, __wbg_height_b92a879a29e66010, __wbg_uniform1f_fa50abe89ff891ea, __wbg_uniform2f_ab7c909be2949448, __wbg_drawElementsInstanced_6a606cd25bdbafb3, __wbg_drawElements_c109bfea7998fd99, __wbg_uniform4f_9941fe9c32da60ea, __wbg_drawArrays_f6e7af9c06f4f4ae, __wbg_uniform1i_a1e8f5ad954fa6b5, __wbg_clearRect_620b55f817af6080, __wbg_setfont_7d7b206c4c017729, __wbg_settextAlign_0ab90671be8e1137, __wbg_setfillStyle_1b068f8d99084158, __wbg_save_be2f4340f20bfe6f, __wbg_translate_a603cdd310297ee8, __wbg_rotate_4ae42333a58388ed, __wbg_fillText_aee0d6016521a3b2, __wbg_restore_e6861230b7a8a25e, __wbg_disable_827be6d0f77447e1, __wbg_parse_b89e797098b3bc7b, __wbg_getContext_a5ae0a2c4fe6f42b, __wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb, __wbindgen_cb_drop, __wbindgen_json_parse, __wbg_uniform1fv_7b33ccba8ca090e4, __wbg_uniformMatrix4fv_82825540b9315680, __wbg_uniform1iv_2c8af1d8286865f0, __wbg_uniform3f_a7c04d3d1c2b18aa, __wbg_isArray_3320300beb1837ab, __wbg_length_8f15bbb4ecbf7e33, __wbg_get_40375c2067f479fc, __wbg_response_c70a68323728a385, __wbg_length_2cfa674c2a529bc1, __wbg_set_d771848e3c7935bb, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_deleteVertexArray_3f37aabaae61ca26, __wbg_disableVertexAttribArray_1401ee870505cf02, __wbg_deleteTexture_cf22af6782ebc54f, __wbg_self_eeabd9085c04fc17, __wbg_window_f110c13310da2c8f, __wbg_globalThis_a2669bee93faee43, __wbg_global_a5584d717f4d6761, __wbindgen_is_undefined, __wbg_newnoargs_179d393e4626fcf7, __wbg_call_8487a9f580e47219, __wbg_newwithbyteoffsetandlength_a20c8edf0fedac40, __wbg_newwithbyteoffsetandlength_7b9a415096aef9c1, __wbindgen_debug_string, __wbg_blendFunc_8bd5998b54c12fd3, __wbg_open_c1608202d44b7d1c, __wbg_setonload_c71ccab98777e104, __wbg_setonerror_1bea8ceda68d0d63, __wbg_send_3e459af287bba919, __wbg_texSubImage2D_43d09711529aa698, __wbindgen_throw, __wbindgen_rethrow, __wbg_instanceof_Window_fa4595281eb5ba83, __wbindgen_closure_wrapper166, __wbindgen_closure_wrapper170 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ "./src/core/pkg/index_bg.wasm");
/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./index_bg.js */ "./src/core/pkg/index_bg.js");
/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "GALCooSys", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["GALCooSys"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "ICRSJ2000CooSys", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["ICRSJ2000CooSys"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "CooSystem", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["CooSystem"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "WebClient", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["WebClient"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_serialize", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_json_serialize"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_drop_ref", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_object_drop_ref"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_canvas_dd578e51a2bc736f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_canvas_dd578e51a2bc736f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setwidth_41b2497107faaff7", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setwidth_41b2497107faaff7"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setheight_e15cb9243262e701", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setheight_e15cb9243262e701"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_viewport_86b156d5858adab9", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_viewport_86b156d5858adab9"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_scissor_1f78ef0050a93516", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_scissor_1f78ef0050a93516"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_string_new", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_string_new"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_memory", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_memory"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_buffer_e35e010c3ba9f945", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_buffer_e35e010c3ba9f945"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_fe24eae01e10f223", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_fe24eae01e10f223"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_subarray_3c6f7cfb4edcc351", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_subarray_3c6f7cfb4edcc351"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_measureText_2a4b2ca71061d96c", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_measureText_2a4b2ca71061d96c"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_width_979b596f39ba8319", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_width_979b596f39ba8319"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindVertexArray_520c05423d3d6641", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bindVertexArray_520c05423d3d6641"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindBuffer_4a7874f09df12419", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bindBuffer_4a7874f09df12419"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bufferSubData_51f29e78449b2095", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bufferSubData_51f29e78449b2095"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bufferData_80963d2bd1ecb1bc", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bufferData_80963d2bd1ecb1bc"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_performance_800ff37c906b5f3b", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_performance_800ff37c906b5f3b"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_now_9f22124bc74da886", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_now_9f22124bc74da886"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_activeTexture_32edab6336bd38a9", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_activeTexture_32edab6336bd38a9"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindTexture_d659843380f373b5", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bindTexture_d659843380f373b5"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_texSubImage2D_e13399a16dfb0646", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_texSubImage2D_e13399a16dfb0646"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_blendFuncSeparate_13c318610edadb4a", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_blendFuncSeparate_13c318610edadb4a"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_enable_65590f4951fd0112", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_enable_65590f4951fd0112"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_cullFace_b0941c23a53ee9fc", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_cullFace_b0941c23a53ee9fc"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_innerWidth_aab6ec3242dff39e", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_innerWidth_aab6ec3242dff39e"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_number_get", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_number_get"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_innerHeight_7e514d9823f7864e", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_innerHeight_7e514d9823f7864e"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_17bf587bb9ce55f1", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_17bf587bb9ce55f1"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setcrossOrigin_054bb95c5a2b2640", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setcrossOrigin_054bb95c5a2b2640"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_4f8fb2c75215d83a", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_4f8fb2c75215d83a"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setresponseType_09ae5e5481a8947d", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setresponseType_09ae5e5481a8947d"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createTexture_8ba2e566eb313fcf", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createTexture_8ba2e566eb313fcf"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_clone_ref", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_object_clone_ref"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonload_69f9426b613d7bd2", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setonload_69f9426b613d7bd2"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonerror_e519d2d2cbd89b1d", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setonerror_e519d2d2cbd89b1d"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setsrc_8742008d92b4e70e", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setsrc_8742008d92b4e70e"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_texParameteri_c0b2b665319f6a16", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_texParameteri_c0b2b665319f6a16"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_texImage2D_a5dad82b8f689bbd", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_texImage2D_a5dad82b8f689bbd"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createFramebuffer_edeb035499d73077", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createFramebuffer_edeb035499d73077"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindFramebuffer_abbc9985c473f160", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_bindFramebuffer_abbc9985c473f160"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_framebufferTexture2D_bb45b3c3d234ddcd", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_framebufferTexture2D_bb45b3c3d234ddcd"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createVertexArray_5cbff3d8bbe1c324", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createVertexArray_5cbff3d8bbe1c324"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_useProgram_b1cc885b00b8f52c", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_useProgram_b1cc885b00b8f52c"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createBuffer_4302ddbcbfc99048", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createBuffer_4302ddbcbfc99048"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_lineWidth_a2c6059f833032d4", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_lineWidth_a2c6059f833032d4"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_vertexAttribPointer_3bb013e284cd07bf", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_vertexAttribPointer_3bb013e284cd07bf"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_enableVertexAttribArray_413ef49912a23f9e", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_enableVertexAttribArray_413ef49912a23f9e"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_document_d8cce4c1031c64eb", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_document_d8cce4c1031c64eb"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getElementsByClassName_a5ef560ae6918226", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getElementsByClassName_a5ef560ae6918226"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getwithindex_bcf1a04b716019a9", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getwithindex_bcf1a04b716019a9"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setAttribute_fb8737b4573a65f8", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setAttribute_fb8737b4573a65f8"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getContext_d277f710e8035242", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getContext_d277f710e8035242"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_scale_e0fdce059098cd1b", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_scale_e0fdce059098cd1b"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_texImage2D_bc294af8c1a6a435", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_texImage2D_bc294af8c1a6a435"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_log_a39f164b49616cb0", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_log_a39f164b49616cb0"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_c77df81d6c892c35", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_c77df81d6c892c35"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_836859e5deb44d3f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newwithbyteoffsetandlength_836859e5deb44d3f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_139e70222494b1ff", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_139e70222494b1ff"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_c274c3296a37fcb4", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newwithbyteoffsetandlength_c274c3296a37fcb4"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_5b74a8dd0c5b71ac", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_5b74a8dd0c5b71ac"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteBuffer_988823f4e76e697d", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_deleteBuffer_988823f4e76e697d"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createProgram_128698dd90ec070d", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createProgram_128698dd90ec070d"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_attachShader_5d53b7b00823cafb", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_attachShader_5d53b7b00823cafb"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_linkProgram_370ed11b34456c89", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_linkProgram_370ed11b34456c89"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getProgramParameter_b949ba1d9662f6a2", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getProgramParameter_b949ba1d9662f6a2"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_boolean_get", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_boolean_get"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getProgramInfoLog_f8f65be65281f691", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getProgramInfoLog_f8f65be65281f691"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getActiveUniform_70b770a58f551f8f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getActiveUniform_70b770a58f551f8f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_name_4f3b7294acbeabad", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_name_4f3b7294acbeabad"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getUniformLocation_472b7459010900a5", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getUniformLocation_472b7459010900a5"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_createShader_26e4f959d5d64d80", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_createShader_26e4f959d5d64d80"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_shaderSource_96ace5133c032f2f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_shaderSource_96ace5133c032f2f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_compileShader_f7e245515fa1405d", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_compileShader_f7e245515fa1405d"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getShaderParameter_cced0ff8ba83f3e7", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getShaderParameter_cced0ff8ba83f3e7"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getShaderInfoLog_5412e8bc642139e8", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getShaderInfoLog_5412e8bc642139e8"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_clearColor_fc22409197a5bd68", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_clearColor_fc22409197a5bd68"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_clear_25e035ed3961f1c6", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_clear_25e035ed3961f1c6"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_width_d9e3643c351ff015", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_width_d9e3643c351ff015"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_height_b92a879a29e66010", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_height_b92a879a29e66010"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1f_fa50abe89ff891ea", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform1f_fa50abe89ff891ea"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform2f_ab7c909be2949448", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform2f_ab7c909be2949448"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawElementsInstanced_6a606cd25bdbafb3", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_drawElementsInstanced_6a606cd25bdbafb3"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawElements_c109bfea7998fd99", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_drawElements_c109bfea7998fd99"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform4f_9941fe9c32da60ea", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform4f_9941fe9c32da60ea"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawArrays_f6e7af9c06f4f4ae", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_drawArrays_f6e7af9c06f4f4ae"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1i_a1e8f5ad954fa6b5", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform1i_a1e8f5ad954fa6b5"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_clearRect_620b55f817af6080", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_clearRect_620b55f817af6080"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setfont_7d7b206c4c017729", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setfont_7d7b206c4c017729"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_settextAlign_0ab90671be8e1137", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_settextAlign_0ab90671be8e1137"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setfillStyle_1b068f8d99084158", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setfillStyle_1b068f8d99084158"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_save_be2f4340f20bfe6f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_save_be2f4340f20bfe6f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_translate_a603cdd310297ee8", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_translate_a603cdd310297ee8"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_rotate_4ae42333a58388ed", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_rotate_4ae42333a58388ed"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_fillText_aee0d6016521a3b2", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_fillText_aee0d6016521a3b2"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_restore_e6861230b7a8a25e", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_restore_e6861230b7a8a25e"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_disable_827be6d0f77447e1", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_disable_827be6d0f77447e1"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_parse_b89e797098b3bc7b", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_parse_b89e797098b3bc7b"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_getContext_a5ae0a2c4fe6f42b", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_getContext_a5ae0a2c4fe6f42b"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_cb_drop", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_cb_drop"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_parse", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_json_parse"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1fv_7b33ccba8ca090e4", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform1fv_7b33ccba8ca090e4"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniformMatrix4fv_82825540b9315680", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniformMatrix4fv_82825540b9315680"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1iv_2c8af1d8286865f0", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform1iv_2c8af1d8286865f0"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform3f_a7c04d3d1c2b18aa", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_uniform3f_a7c04d3d1c2b18aa"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_isArray_3320300beb1837ab", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_isArray_3320300beb1837ab"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_length_8f15bbb4ecbf7e33", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_length_8f15bbb4ecbf7e33"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_get_40375c2067f479fc", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_get_40375c2067f479fc"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_response_c70a68323728a385", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_response_c70a68323728a385"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_length_2cfa674c2a529bc1", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_length_2cfa674c2a529bc1"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_set_d771848e3c7935bb", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_set_d771848e3c7935bb"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_59cb74e423758ede", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_new_59cb74e423758ede"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_stack_558ba5917b466edd", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_stack_558ba5917b466edd"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_error_4bb6c2a97407129a", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_error_4bb6c2a97407129a"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteVertexArray_3f37aabaae61ca26", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_deleteVertexArray_3f37aabaae61ca26"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_disableVertexAttribArray_1401ee870505cf02", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_disableVertexAttribArray_1401ee870505cf02"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteTexture_cf22af6782ebc54f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_deleteTexture_cf22af6782ebc54f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_self_eeabd9085c04fc17", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_self_eeabd9085c04fc17"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_window_f110c13310da2c8f", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_window_f110c13310da2c8f"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_globalThis_a2669bee93faee43", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_globalThis_a2669bee93faee43"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_global_a5584d717f4d6761", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_global_a5584d717f4d6761"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_is_undefined", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_is_undefined"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newnoargs_179d393e4626fcf7", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newnoargs_179d393e4626fcf7"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_call_8487a9f580e47219", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_call_8487a9f580e47219"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_a20c8edf0fedac40", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newwithbyteoffsetandlength_a20c8edf0fedac40"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_7b9a415096aef9c1", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_newwithbyteoffsetandlength_7b9a415096aef9c1"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_debug_string", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_debug_string"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_blendFunc_8bd5998b54c12fd3", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_blendFunc_8bd5998b54c12fd3"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_open_c1608202d44b7d1c", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_open_c1608202d44b7d1c"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonload_c71ccab98777e104", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setonload_c71ccab98777e104"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonerror_1bea8ceda68d0d63", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_setonerror_1bea8ceda68d0d63"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_send_3e459af287bba919", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_send_3e459af287bba919"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_texSubImage2D_43d09711529aa698", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_texSubImage2D_43d09711529aa698"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_throw"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_rethrow", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_rethrow"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_Window_fa4595281eb5ba83", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbg_instanceof_Window_fa4595281eb5ba83"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_closure_wrapper166", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_closure_wrapper166"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_closure_wrapper170", function() { return _index_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_closure_wrapper170"]; });




/***/ }),

/***/ "./src/core/pkg/index_bg.js":
/*!**********************************!*\
  !*** ./src/core/pkg/index_bg.js ***!
  \**********************************/
/*! exports provided: GALCooSys, ICRSJ2000CooSys, CooSystem, WebClient, __wbindgen_json_serialize, __wbindgen_object_drop_ref, __wbg_canvas_dd578e51a2bc736f, __wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430, __wbg_setwidth_41b2497107faaff7, __wbg_setheight_e15cb9243262e701, __wbg_viewport_86b156d5858adab9, __wbg_scissor_1f78ef0050a93516, __wbindgen_string_new, __wbindgen_memory, __wbg_buffer_e35e010c3ba9f945, __wbg_new_fe24eae01e10f223, __wbg_subarray_3c6f7cfb4edcc351, __wbg_measureText_2a4b2ca71061d96c, __wbg_width_979b596f39ba8319, __wbg_bindVertexArray_520c05423d3d6641, __wbg_bindBuffer_4a7874f09df12419, __wbg_bufferSubData_51f29e78449b2095, __wbg_bufferData_80963d2bd1ecb1bc, __wbg_performance_800ff37c906b5f3b, __wbg_now_9f22124bc74da886, __wbg_activeTexture_32edab6336bd38a9, __wbg_bindTexture_d659843380f373b5, __wbg_texSubImage2D_e13399a16dfb0646, __wbg_blendFuncSeparate_13c318610edadb4a, __wbg_enable_65590f4951fd0112, __wbg_cullFace_b0941c23a53ee9fc, __wbg_innerWidth_aab6ec3242dff39e, __wbindgen_number_get, __wbg_innerHeight_7e514d9823f7864e, __wbg_new_17bf587bb9ce55f1, __wbg_setcrossOrigin_054bb95c5a2b2640, __wbg_new_4f8fb2c75215d83a, __wbg_setresponseType_09ae5e5481a8947d, __wbg_createTexture_8ba2e566eb313fcf, __wbindgen_object_clone_ref, __wbg_setonload_69f9426b613d7bd2, __wbg_setonerror_e519d2d2cbd89b1d, __wbg_setsrc_8742008d92b4e70e, __wbg_texParameteri_c0b2b665319f6a16, __wbg_texImage2D_a5dad82b8f689bbd, __wbg_createFramebuffer_edeb035499d73077, __wbg_bindFramebuffer_abbc9985c473f160, __wbg_framebufferTexture2D_bb45b3c3d234ddcd, __wbg_createVertexArray_5cbff3d8bbe1c324, __wbg_useProgram_b1cc885b00b8f52c, __wbg_createBuffer_4302ddbcbfc99048, __wbg_lineWidth_a2c6059f833032d4, __wbg_vertexAttribPointer_3bb013e284cd07bf, __wbg_enableVertexAttribArray_413ef49912a23f9e, __wbg_document_d8cce4c1031c64eb, __wbg_getElementsByClassName_a5ef560ae6918226, __wbg_getwithindex_bcf1a04b716019a9, __wbg_setAttribute_fb8737b4573a65f8, __wbg_getContext_d277f710e8035242, __wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3, __wbg_scale_e0fdce059098cd1b, __wbg_texImage2D_bc294af8c1a6a435, __wbg_log_a39f164b49616cb0, __wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8, __wbg_new_c77df81d6c892c35, __wbg_newwithbyteoffsetandlength_836859e5deb44d3f, __wbg_new_139e70222494b1ff, __wbg_newwithbyteoffsetandlength_c274c3296a37fcb4, __wbg_new_5b74a8dd0c5b71ac, __wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1, __wbg_deleteBuffer_988823f4e76e697d, __wbg_createProgram_128698dd90ec070d, __wbg_attachShader_5d53b7b00823cafb, __wbg_linkProgram_370ed11b34456c89, __wbg_getProgramParameter_b949ba1d9662f6a2, __wbindgen_boolean_get, __wbg_getProgramInfoLog_f8f65be65281f691, __wbg_getActiveUniform_70b770a58f551f8f, __wbg_name_4f3b7294acbeabad, __wbg_getUniformLocation_472b7459010900a5, __wbg_createShader_26e4f959d5d64d80, __wbg_shaderSource_96ace5133c032f2f, __wbg_compileShader_f7e245515fa1405d, __wbg_getShaderParameter_cced0ff8ba83f3e7, __wbg_getShaderInfoLog_5412e8bc642139e8, __wbg_clearColor_fc22409197a5bd68, __wbg_clear_25e035ed3961f1c6, __wbg_width_d9e3643c351ff015, __wbg_height_b92a879a29e66010, __wbg_uniform1f_fa50abe89ff891ea, __wbg_uniform2f_ab7c909be2949448, __wbg_drawElementsInstanced_6a606cd25bdbafb3, __wbg_drawElements_c109bfea7998fd99, __wbg_uniform4f_9941fe9c32da60ea, __wbg_drawArrays_f6e7af9c06f4f4ae, __wbg_uniform1i_a1e8f5ad954fa6b5, __wbg_clearRect_620b55f817af6080, __wbg_setfont_7d7b206c4c017729, __wbg_settextAlign_0ab90671be8e1137, __wbg_setfillStyle_1b068f8d99084158, __wbg_save_be2f4340f20bfe6f, __wbg_translate_a603cdd310297ee8, __wbg_rotate_4ae42333a58388ed, __wbg_fillText_aee0d6016521a3b2, __wbg_restore_e6861230b7a8a25e, __wbg_disable_827be6d0f77447e1, __wbg_parse_b89e797098b3bc7b, __wbg_getContext_a5ae0a2c4fe6f42b, __wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb, __wbindgen_cb_drop, __wbindgen_json_parse, __wbg_uniform1fv_7b33ccba8ca090e4, __wbg_uniformMatrix4fv_82825540b9315680, __wbg_uniform1iv_2c8af1d8286865f0, __wbg_uniform3f_a7c04d3d1c2b18aa, __wbg_isArray_3320300beb1837ab, __wbg_length_8f15bbb4ecbf7e33, __wbg_get_40375c2067f479fc, __wbg_response_c70a68323728a385, __wbg_length_2cfa674c2a529bc1, __wbg_set_d771848e3c7935bb, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbg_deleteVertexArray_3f37aabaae61ca26, __wbg_disableVertexAttribArray_1401ee870505cf02, __wbg_deleteTexture_cf22af6782ebc54f, __wbg_self_eeabd9085c04fc17, __wbg_window_f110c13310da2c8f, __wbg_globalThis_a2669bee93faee43, __wbg_global_a5584d717f4d6761, __wbindgen_is_undefined, __wbg_newnoargs_179d393e4626fcf7, __wbg_call_8487a9f580e47219, __wbg_newwithbyteoffsetandlength_a20c8edf0fedac40, __wbg_newwithbyteoffsetandlength_7b9a415096aef9c1, __wbindgen_debug_string, __wbg_blendFunc_8bd5998b54c12fd3, __wbg_open_c1608202d44b7d1c, __wbg_setonload_c71ccab98777e104, __wbg_setonerror_1bea8ceda68d0d63, __wbg_send_3e459af287bba919, __wbg_texSubImage2D_43d09711529aa698, __wbindgen_throw, __wbindgen_rethrow, __wbg_instanceof_Window_fa4595281eb5ba83, __wbindgen_closure_wrapper166, __wbindgen_closure_wrapper170 */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* WEBPACK VAR INJECTION */(function(TextEncoder, module, TextDecoder, global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "GALCooSys", function() { return GALCooSys; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "ICRSJ2000CooSys", function() { return ICRSJ2000CooSys; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "CooSystem", function() { return CooSystem; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "WebClient", function() { return WebClient; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_serialize", function() { return __wbindgen_json_serialize; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_drop_ref", function() { return __wbindgen_object_drop_ref; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_canvas_dd578e51a2bc736f", function() { return __wbg_canvas_dd578e51a2bc736f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430", function() { return __wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setwidth_41b2497107faaff7", function() { return __wbg_setwidth_41b2497107faaff7; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setheight_e15cb9243262e701", function() { return __wbg_setheight_e15cb9243262e701; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_viewport_86b156d5858adab9", function() { return __wbg_viewport_86b156d5858adab9; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_scissor_1f78ef0050a93516", function() { return __wbg_scissor_1f78ef0050a93516; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_string_new", function() { return __wbindgen_string_new; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_memory", function() { return __wbindgen_memory; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_buffer_e35e010c3ba9f945", function() { return __wbg_buffer_e35e010c3ba9f945; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_fe24eae01e10f223", function() { return __wbg_new_fe24eae01e10f223; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_subarray_3c6f7cfb4edcc351", function() { return __wbg_subarray_3c6f7cfb4edcc351; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_measureText_2a4b2ca71061d96c", function() { return __wbg_measureText_2a4b2ca71061d96c; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_width_979b596f39ba8319", function() { return __wbg_width_979b596f39ba8319; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindVertexArray_520c05423d3d6641", function() { return __wbg_bindVertexArray_520c05423d3d6641; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindBuffer_4a7874f09df12419", function() { return __wbg_bindBuffer_4a7874f09df12419; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bufferSubData_51f29e78449b2095", function() { return __wbg_bufferSubData_51f29e78449b2095; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bufferData_80963d2bd1ecb1bc", function() { return __wbg_bufferData_80963d2bd1ecb1bc; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_performance_800ff37c906b5f3b", function() { return __wbg_performance_800ff37c906b5f3b; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_now_9f22124bc74da886", function() { return __wbg_now_9f22124bc74da886; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_activeTexture_32edab6336bd38a9", function() { return __wbg_activeTexture_32edab6336bd38a9; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindTexture_d659843380f373b5", function() { return __wbg_bindTexture_d659843380f373b5; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_texSubImage2D_e13399a16dfb0646", function() { return __wbg_texSubImage2D_e13399a16dfb0646; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_blendFuncSeparate_13c318610edadb4a", function() { return __wbg_blendFuncSeparate_13c318610edadb4a; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_enable_65590f4951fd0112", function() { return __wbg_enable_65590f4951fd0112; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_cullFace_b0941c23a53ee9fc", function() { return __wbg_cullFace_b0941c23a53ee9fc; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_innerWidth_aab6ec3242dff39e", function() { return __wbg_innerWidth_aab6ec3242dff39e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_number_get", function() { return __wbindgen_number_get; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_innerHeight_7e514d9823f7864e", function() { return __wbg_innerHeight_7e514d9823f7864e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_17bf587bb9ce55f1", function() { return __wbg_new_17bf587bb9ce55f1; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setcrossOrigin_054bb95c5a2b2640", function() { return __wbg_setcrossOrigin_054bb95c5a2b2640; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_4f8fb2c75215d83a", function() { return __wbg_new_4f8fb2c75215d83a; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setresponseType_09ae5e5481a8947d", function() { return __wbg_setresponseType_09ae5e5481a8947d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createTexture_8ba2e566eb313fcf", function() { return __wbg_createTexture_8ba2e566eb313fcf; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_object_clone_ref", function() { return __wbindgen_object_clone_ref; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonload_69f9426b613d7bd2", function() { return __wbg_setonload_69f9426b613d7bd2; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonerror_e519d2d2cbd89b1d", function() { return __wbg_setonerror_e519d2d2cbd89b1d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setsrc_8742008d92b4e70e", function() { return __wbg_setsrc_8742008d92b4e70e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_texParameteri_c0b2b665319f6a16", function() { return __wbg_texParameteri_c0b2b665319f6a16; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_texImage2D_a5dad82b8f689bbd", function() { return __wbg_texImage2D_a5dad82b8f689bbd; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createFramebuffer_edeb035499d73077", function() { return __wbg_createFramebuffer_edeb035499d73077; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_bindFramebuffer_abbc9985c473f160", function() { return __wbg_bindFramebuffer_abbc9985c473f160; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_framebufferTexture2D_bb45b3c3d234ddcd", function() { return __wbg_framebufferTexture2D_bb45b3c3d234ddcd; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createVertexArray_5cbff3d8bbe1c324", function() { return __wbg_createVertexArray_5cbff3d8bbe1c324; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_useProgram_b1cc885b00b8f52c", function() { return __wbg_useProgram_b1cc885b00b8f52c; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createBuffer_4302ddbcbfc99048", function() { return __wbg_createBuffer_4302ddbcbfc99048; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_lineWidth_a2c6059f833032d4", function() { return __wbg_lineWidth_a2c6059f833032d4; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_vertexAttribPointer_3bb013e284cd07bf", function() { return __wbg_vertexAttribPointer_3bb013e284cd07bf; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_enableVertexAttribArray_413ef49912a23f9e", function() { return __wbg_enableVertexAttribArray_413ef49912a23f9e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_document_d8cce4c1031c64eb", function() { return __wbg_document_d8cce4c1031c64eb; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getElementsByClassName_a5ef560ae6918226", function() { return __wbg_getElementsByClassName_a5ef560ae6918226; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getwithindex_bcf1a04b716019a9", function() { return __wbg_getwithindex_bcf1a04b716019a9; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setAttribute_fb8737b4573a65f8", function() { return __wbg_setAttribute_fb8737b4573a65f8; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getContext_d277f710e8035242", function() { return __wbg_getContext_d277f710e8035242; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3", function() { return __wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_scale_e0fdce059098cd1b", function() { return __wbg_scale_e0fdce059098cd1b; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_texImage2D_bc294af8c1a6a435", function() { return __wbg_texImage2D_bc294af8c1a6a435; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_log_a39f164b49616cb0", function() { return __wbg_log_a39f164b49616cb0; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8", function() { return __wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_c77df81d6c892c35", function() { return __wbg_new_c77df81d6c892c35; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_836859e5deb44d3f", function() { return __wbg_newwithbyteoffsetandlength_836859e5deb44d3f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_139e70222494b1ff", function() { return __wbg_new_139e70222494b1ff; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_c274c3296a37fcb4", function() { return __wbg_newwithbyteoffsetandlength_c274c3296a37fcb4; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_5b74a8dd0c5b71ac", function() { return __wbg_new_5b74a8dd0c5b71ac; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1", function() { return __wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteBuffer_988823f4e76e697d", function() { return __wbg_deleteBuffer_988823f4e76e697d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createProgram_128698dd90ec070d", function() { return __wbg_createProgram_128698dd90ec070d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_attachShader_5d53b7b00823cafb", function() { return __wbg_attachShader_5d53b7b00823cafb; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_linkProgram_370ed11b34456c89", function() { return __wbg_linkProgram_370ed11b34456c89; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getProgramParameter_b949ba1d9662f6a2", function() { return __wbg_getProgramParameter_b949ba1d9662f6a2; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_boolean_get", function() { return __wbindgen_boolean_get; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getProgramInfoLog_f8f65be65281f691", function() { return __wbg_getProgramInfoLog_f8f65be65281f691; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getActiveUniform_70b770a58f551f8f", function() { return __wbg_getActiveUniform_70b770a58f551f8f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_name_4f3b7294acbeabad", function() { return __wbg_name_4f3b7294acbeabad; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getUniformLocation_472b7459010900a5", function() { return __wbg_getUniformLocation_472b7459010900a5; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_createShader_26e4f959d5d64d80", function() { return __wbg_createShader_26e4f959d5d64d80; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_shaderSource_96ace5133c032f2f", function() { return __wbg_shaderSource_96ace5133c032f2f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_compileShader_f7e245515fa1405d", function() { return __wbg_compileShader_f7e245515fa1405d; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getShaderParameter_cced0ff8ba83f3e7", function() { return __wbg_getShaderParameter_cced0ff8ba83f3e7; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getShaderInfoLog_5412e8bc642139e8", function() { return __wbg_getShaderInfoLog_5412e8bc642139e8; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_clearColor_fc22409197a5bd68", function() { return __wbg_clearColor_fc22409197a5bd68; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_clear_25e035ed3961f1c6", function() { return __wbg_clear_25e035ed3961f1c6; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_width_d9e3643c351ff015", function() { return __wbg_width_d9e3643c351ff015; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_height_b92a879a29e66010", function() { return __wbg_height_b92a879a29e66010; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1f_fa50abe89ff891ea", function() { return __wbg_uniform1f_fa50abe89ff891ea; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform2f_ab7c909be2949448", function() { return __wbg_uniform2f_ab7c909be2949448; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawElementsInstanced_6a606cd25bdbafb3", function() { return __wbg_drawElementsInstanced_6a606cd25bdbafb3; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawElements_c109bfea7998fd99", function() { return __wbg_drawElements_c109bfea7998fd99; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform4f_9941fe9c32da60ea", function() { return __wbg_uniform4f_9941fe9c32da60ea; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_drawArrays_f6e7af9c06f4f4ae", function() { return __wbg_drawArrays_f6e7af9c06f4f4ae; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1i_a1e8f5ad954fa6b5", function() { return __wbg_uniform1i_a1e8f5ad954fa6b5; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_clearRect_620b55f817af6080", function() { return __wbg_clearRect_620b55f817af6080; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setfont_7d7b206c4c017729", function() { return __wbg_setfont_7d7b206c4c017729; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_settextAlign_0ab90671be8e1137", function() { return __wbg_settextAlign_0ab90671be8e1137; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setfillStyle_1b068f8d99084158", function() { return __wbg_setfillStyle_1b068f8d99084158; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_save_be2f4340f20bfe6f", function() { return __wbg_save_be2f4340f20bfe6f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_translate_a603cdd310297ee8", function() { return __wbg_translate_a603cdd310297ee8; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_rotate_4ae42333a58388ed", function() { return __wbg_rotate_4ae42333a58388ed; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_fillText_aee0d6016521a3b2", function() { return __wbg_fillText_aee0d6016521a3b2; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_restore_e6861230b7a8a25e", function() { return __wbg_restore_e6861230b7a8a25e; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_disable_827be6d0f77447e1", function() { return __wbg_disable_827be6d0f77447e1; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_parse_b89e797098b3bc7b", function() { return __wbg_parse_b89e797098b3bc7b; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_getContext_a5ae0a2c4fe6f42b", function() { return __wbg_getContext_a5ae0a2c4fe6f42b; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb", function() { return __wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_cb_drop", function() { return __wbindgen_cb_drop; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_json_parse", function() { return __wbindgen_json_parse; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1fv_7b33ccba8ca090e4", function() { return __wbg_uniform1fv_7b33ccba8ca090e4; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniformMatrix4fv_82825540b9315680", function() { return __wbg_uniformMatrix4fv_82825540b9315680; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform1iv_2c8af1d8286865f0", function() { return __wbg_uniform1iv_2c8af1d8286865f0; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_uniform3f_a7c04d3d1c2b18aa", function() { return __wbg_uniform3f_a7c04d3d1c2b18aa; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_isArray_3320300beb1837ab", function() { return __wbg_isArray_3320300beb1837ab; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_length_8f15bbb4ecbf7e33", function() { return __wbg_length_8f15bbb4ecbf7e33; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_get_40375c2067f479fc", function() { return __wbg_get_40375c2067f479fc; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_response_c70a68323728a385", function() { return __wbg_response_c70a68323728a385; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_length_2cfa674c2a529bc1", function() { return __wbg_length_2cfa674c2a529bc1; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_set_d771848e3c7935bb", function() { return __wbg_set_d771848e3c7935bb; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_new_59cb74e423758ede", function() { return __wbg_new_59cb74e423758ede; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_stack_558ba5917b466edd", function() { return __wbg_stack_558ba5917b466edd; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_error_4bb6c2a97407129a", function() { return __wbg_error_4bb6c2a97407129a; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteVertexArray_3f37aabaae61ca26", function() { return __wbg_deleteVertexArray_3f37aabaae61ca26; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_disableVertexAttribArray_1401ee870505cf02", function() { return __wbg_disableVertexAttribArray_1401ee870505cf02; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_deleteTexture_cf22af6782ebc54f", function() { return __wbg_deleteTexture_cf22af6782ebc54f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_self_eeabd9085c04fc17", function() { return __wbg_self_eeabd9085c04fc17; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_window_f110c13310da2c8f", function() { return __wbg_window_f110c13310da2c8f; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_globalThis_a2669bee93faee43", function() { return __wbg_globalThis_a2669bee93faee43; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_global_a5584d717f4d6761", function() { return __wbg_global_a5584d717f4d6761; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_is_undefined", function() { return __wbindgen_is_undefined; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newnoargs_179d393e4626fcf7", function() { return __wbg_newnoargs_179d393e4626fcf7; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_call_8487a9f580e47219", function() { return __wbg_call_8487a9f580e47219; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_a20c8edf0fedac40", function() { return __wbg_newwithbyteoffsetandlength_a20c8edf0fedac40; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_newwithbyteoffsetandlength_7b9a415096aef9c1", function() { return __wbg_newwithbyteoffsetandlength_7b9a415096aef9c1; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_debug_string", function() { return __wbindgen_debug_string; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_blendFunc_8bd5998b54c12fd3", function() { return __wbg_blendFunc_8bd5998b54c12fd3; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_open_c1608202d44b7d1c", function() { return __wbg_open_c1608202d44b7d1c; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonload_c71ccab98777e104", function() { return __wbg_setonload_c71ccab98777e104; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_setonerror_1bea8ceda68d0d63", function() { return __wbg_setonerror_1bea8ceda68d0d63; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_send_3e459af287bba919", function() { return __wbg_send_3e459af287bba919; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_texSubImage2D_43d09711529aa698", function() { return __wbg_texSubImage2D_43d09711529aa698; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return __wbindgen_throw; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_rethrow", function() { return __wbindgen_rethrow; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbg_instanceof_Window_fa4595281eb5ba83", function() { return __wbg_instanceof_Window_fa4595281eb5ba83; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_closure_wrapper166", function() { return __wbindgen_closure_wrapper166; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_closure_wrapper170", function() { return __wbindgen_closure_wrapper170; });
/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ "./src/core/pkg/index_bg.wasm");


const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetUint8Memory0 = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetInt32Memory0 = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetInt32Memory0;
}

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetFloat64Memory0 = new Float64Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetFloat64Memory0;
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_export_2"].get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

function logError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            let error = (function () {
                try {
                    return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
                } catch(_) {
                    return "<failed to stringify thrown value>";
                }
            }());
            console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
            throw e;
        }
    };
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_28(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["_dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h47dd8a349271e080"](arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_export_2"].get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_31(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["_dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h342357a9a09a4a44"](arg0, arg1);
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetUint32Memory0 = new Uint32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4);
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayF64FromWasm0(ptr, len) {
    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
* @returns {number}
*/
function GALCooSys() {
    var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["GALCooSys"]();
    return ret >>> 0;
}

/**
* @returns {number}
*/
function ICRSJ2000CooSys() {
    var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["ICRSJ2000CooSys"]();
    return ret >>> 0;
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_exn_store"](addHeapObject(e));
        }
    };
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetFloat32Memory0 = new Float32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayI32FromWasm0(ptr, len) {
    return getInt32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
*/
const CooSystem = Object.freeze({ ICRSJ2000:0,"0":"ICRSJ2000",GAL:1,"1":"GAL", });
/**
*/
class WebClient {

    static __wrap(ptr) {
        const obj = Object.create(WebClient.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_webclient_free"](ptr);
    }
    /**
    * Create a new web client
    * @param {any} shaders
    * @param {any} resources
    */
    constructor(shaders, resources) {
        try {
            var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_new"](addBorrowedObject(shaders), addBorrowedObject(resources));
            return WebClient.__wrap(ret);
        } finally {
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Main update method
    * The force parameter ensures to force the update of some elements
    * even if the camera has not moved
    * @param {number} dt
    * @param {boolean} force
    */
    update(dt, force) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertBoolean(force);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_update"](this.ptr, dt, force);
    }
    /**
    * Resize the window
    * @param {number} width
    * @param {number} height
    */
    resize(width, height) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_resize"](this.ptr, width, height);
    }
    /**
    * Update our WebGL Water application.
    * @param {boolean} force
    */
    render(force) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertBoolean(force);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_render"](this.ptr, force);
    }
    /**
    * Change the current projection of the HiPS
    * @param {string} name
    */
    setProjection(name) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(name, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setProjection"](this.ptr, ptr0, len0);
    }
    /**
    * Change the current projection of the HiPS
    * @param {boolean} reversed
    */
    setLongitudeReversed(reversed) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertBoolean(reversed);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setLongitudeReversed"](this.ptr, reversed);
    }
    /**
    * Image surveys
    * Check whether the app is ready
    *
    * Aladin Lite is in a good state when the root tiles of the
    * HiPS chosen have all been retrieved and accessible for the GPU
    *
    * The javascript can change the HiPSes only if aladin lite is ready
    * @returns {boolean}
    */
    isReady() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_isReady"](this.ptr);
        return ret !== 0;
    }
    /**
    * @param {any[]} surveys
    */
    setImageSurveys(surveys) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passArrayJsValueToWasm0(surveys, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setImageSurveys"](this.ptr, ptr0, len0);
    }
    /**
    * Move a layer forward
    *
    * # Panics
    *
    * If the layer specified is not found
    * @param {string} layer_name
    */
    moveImageSurveysLayerForward(layer_name) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(layer_name, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_moveImageSurveysLayerForward"](this.ptr, ptr0, len0);
    }
    /**
    * @param {number} opacity
    * @param {string} layer_name
    */
    setOpacityLayer(opacity, layer_name) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(layer_name, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setOpacityLayer"](this.ptr, opacity, ptr0, len0);
    }
    /**
    * Grid
    * Change grid color
    * @param {number} red
    * @param {number} green
    * @param {number} blue
    * @param {number} alpha
    */
    setGridColor(red, green, blue, alpha) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setGridColor"](this.ptr, red, green, blue, alpha);
    }
    /**
    * Enable the draw of the grid
    */
    enableGrid() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_enableGrid"](this.ptr);
    }
    /**
    * Disable the draw of the grid
    */
    disableGrid() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_disableGrid"](this.ptr);
    }
    /**
    */
    hideGridLabels() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_hideGridLabels"](this.ptr);
    }
    /**
    */
    showGridLabels() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_showGridLabels"](this.ptr);
    }
    /**
    * ICRS in J2000 to galactic conversion functions
    * @returns {number}
    */
    cooSystem() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_cooSystem"](this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} coo_system
    */
    setCooSystem(coo_system) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _assertNum(coo_system);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setCooSystem"](this.ptr, coo_system);
    }
    /**
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    J20002Gal(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_J20002Gal"](retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    Gal2J2000(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_Gal2J2000"](retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * Camera moving functions
    * @returns {number}
    */
    getFieldOfView() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getFieldOfView"](this.ptr);
        return ret;
    }
    /**
    * Set directly the field of view (for pinch zooming)
    * @param {number} fov
    */
    setFieldOfView(fov) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setFieldOfView"](this.ptr, fov);
    }
    /**
    * @param {number} theta
    */
    setRotationAroundCenter(theta) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setRotationAroundCenter"](this.ptr, theta);
    }
    /**
    * @returns {number}
    */
    getRotationAroundCenter() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getRotationAroundCenter"](this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getMaxFieldOfView() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getMaxFieldOfView"](this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getClipZoomFactor() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getClipZoomFactor"](this.ptr);
        return ret;
    }
    /**
    * Set directly the center position
    * @param {number} lon
    * @param {number} lat
    */
    setCenter(lon, lat) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setCenter"](this.ptr, lon, lat);
    }
    /**
    * Set directly the center position
    * @returns {Float64Array}
    */
    getCenter() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getCenter"](retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * Initiate a finite state machine that will move to a specific location
    * @param {number} lon
    * @param {number} lat
    */
    moveToLocation(lon, lat) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_moveToLocation"](this.ptr, lon, lat);
    }
    /**
    * @param {number} s1x
    * @param {number} s1y
    * @param {number} s2x
    * @param {number} s2y
    */
    goFromTo(s1x, s1y, s2x, s2y) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_goFromTo"](this.ptr, s1x, s1y, s2x, s2y);
    }
    /**
    * World to screen projection
    *
    * Coordinates must be given in ICRS J2000
    * They will be converted accordingly to the current frame of Aladin Lite
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    worldToScreen(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_worldToScreen"](retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * @param {any[]} sources
    * @returns {Float64Array}
    */
    worldToScreenVec(sources) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            var ptr0 = passArrayJsValueToWasm0(sources, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_worldToScreenVec"](retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            return v1;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * @param {number} pos_x
    * @param {number} pos_y
    * @returns {Float64Array | undefined}
    */
    screenToWorld(pos_x, pos_y) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_screenToWorld"](retptr, this.ptr, pos_x, pos_y);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * Tell the backend when the left mouse button has been
    * released. This is useful for beginning inerting
    */
    releaseLeftButtonMouse() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_releaseLeftButtonMouse"](this.ptr);
    }
    /**
    * Tell the backend when the left mouse button has been pressed
    */
    pressLeftMouseButton() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_pressLeftMouseButton"](this.ptr);
    }
    /**
    * @param {number} delta
    */
    registerWheelEvent(delta) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_registerWheelEvent"](this.ptr, delta);
    }
    /**
    * Catalogs
    * @param {string} name_catalog
    * @param {any} data
    * @param {string} colormap
    */
    addCatalog(name_catalog, data, colormap) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(colormap, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len1 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_addCatalog"](this.ptr, ptr0, len0, addHeapObject(data), ptr1, len1);
    }
    /**
    * @returns {boolean}
    */
    isCatalogLoaded() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_isCatalogLoaded"](this.ptr);
        return ret !== 0;
    }
    /**
    * @param {string} name_catalog
    * @param {string} colormap
    */
    setCatalogColormap(name_catalog, colormap) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        var ptr1 = passStringToWasm0(colormap, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len1 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setCatalogColormap"](this.ptr, ptr0, len0, ptr1, len1);
    }
    /**
    * Set the heatmap global opacity
    * @param {string} name_catalog
    * @param {number} opacity
    */
    setCatalogOpacity(name_catalog, opacity) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setCatalogOpacity"](this.ptr, ptr0, len0, opacity);
    }
    /**
    * @param {string} name_catalog
    * @param {number} strength
    */
    setCatalogKernelStrength(name_catalog, strength) {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
        var len0 = WASM_VECTOR_LEN;
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_setCatalogKernelStrength"](this.ptr, ptr0, len0, strength);
    }
    /**
    * Utilities
    * @param {number} lon1
    * @param {number} lat1
    * @param {number} lon2
    * @param {number} lat2
    * @returns {Float64Array}
    */
    projectLine(lon1, lat1, lon2, lat2) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_projectLine"](retptr, this.ptr, lon1, lat1, lon2, lat2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v0 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_add_to_stack_pointer"](16);
        }
    }
    /**
    * @returns {any}
    */
    getAvailableColormapList() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["webclient_getAvailableColormapList"](this.ptr);
        return takeObject(ret);
    }
}

const __wbindgen_json_serialize = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

const __wbg_canvas_dd578e51a2bc736f = logError(function(arg0) {
    var ret = getObject(arg0).canvas;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430 = logError(function(arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    _assertBoolean(ret);
    return ret;
});

const __wbg_setwidth_41b2497107faaff7 = logError(function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
});

const __wbg_setheight_e15cb9243262e701 = logError(function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
});

const __wbg_viewport_86b156d5858adab9 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
});

const __wbg_scissor_1f78ef0050a93516 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
});

const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

const __wbindgen_memory = function() {
    var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"];
    return addHeapObject(ret);
};

const __wbg_buffer_e35e010c3ba9f945 = logError(function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
});

const __wbg_new_fe24eae01e10f223 = logError(function(arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
});

const __wbg_subarray_3c6f7cfb4edcc351 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_measureText_2a4b2ca71061d96c = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).measureText(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
});

const __wbg_width_979b596f39ba8319 = logError(function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
});

const __wbg_bindVertexArray_520c05423d3d6641 = logError(function(arg0, arg1) {
    getObject(arg0).bindVertexArray(getObject(arg1));
});

const __wbg_bindBuffer_4a7874f09df12419 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
});

const __wbg_bufferSubData_51f29e78449b2095 = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
});

const __wbg_bufferData_80963d2bd1ecb1bc = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
});

const __wbg_performance_800ff37c906b5f3b = logError(function(arg0) {
    var ret = getObject(arg0).performance;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_now_9f22124bc74da886 = logError(function(arg0) {
    var ret = getObject(arg0).now();
    return ret;
});

const __wbg_activeTexture_32edab6336bd38a9 = logError(function(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
});

const __wbg_bindTexture_d659843380f373b5 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
});

const __wbg_texSubImage2D_e13399a16dfb0646 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
});

const __wbg_blendFuncSeparate_13c318610edadb4a = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
});

const __wbg_enable_65590f4951fd0112 = logError(function(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
});

const __wbg_cullFace_b0941c23a53ee9fc = logError(function(arg0, arg1) {
    getObject(arg0).cullFace(arg1 >>> 0);
});

const __wbg_innerWidth_aab6ec3242dff39e = handleError(function(arg0) {
    var ret = getObject(arg0).innerWidth;
    return addHeapObject(ret);
});

const __wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'number' ? obj : undefined;
    if (!isLikeNone(ret)) {
        _assertNum(ret);
    }
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

const __wbg_innerHeight_7e514d9823f7864e = handleError(function(arg0) {
    var ret = getObject(arg0).innerHeight;
    return addHeapObject(ret);
});

const __wbg_new_17bf587bb9ce55f1 = handleError(function() {
    var ret = new Image();
    return addHeapObject(ret);
});

const __wbg_setcrossOrigin_054bb95c5a2b2640 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
});

const __wbg_new_4f8fb2c75215d83a = handleError(function() {
    var ret = new XMLHttpRequest();
    return addHeapObject(ret);
});

const __wbg_setresponseType_09ae5e5481a8947d = logError(function(arg0, arg1) {
    getObject(arg0).responseType = takeObject(arg1);
});

const __wbg_createTexture_8ba2e566eb313fcf = logError(function(arg0) {
    var ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

const __wbg_setonload_69f9426b613d7bd2 = logError(function(arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
});

const __wbg_setonerror_e519d2d2cbd89b1d = logError(function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
});

const __wbg_setsrc_8742008d92b4e70e = logError(function(arg0, arg1, arg2) {
    getObject(arg0).src = getStringFromWasm0(arg1, arg2);
});

const __wbg_texParameteri_c0b2b665319f6a16 = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
});

const __wbg_texImage2D_a5dad82b8f689bbd = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
});

const __wbg_createFramebuffer_edeb035499d73077 = logError(function(arg0) {
    var ret = getObject(arg0).createFramebuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_bindFramebuffer_abbc9985c473f160 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
});

const __wbg_framebufferTexture2D_bb45b3c3d234ddcd = logError(function(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
});

const __wbg_createVertexArray_5cbff3d8bbe1c324 = logError(function(arg0) {
    var ret = getObject(arg0).createVertexArray();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_useProgram_b1cc885b00b8f52c = logError(function(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
});

const __wbg_createBuffer_4302ddbcbfc99048 = logError(function(arg0) {
    var ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_lineWidth_a2c6059f833032d4 = logError(function(arg0, arg1) {
    getObject(arg0).lineWidth(arg1);
});

const __wbg_vertexAttribPointer_3bb013e284cd07bf = logError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
});

const __wbg_enableVertexAttribArray_413ef49912a23f9e = logError(function(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
});

const __wbg_document_d8cce4c1031c64eb = logError(function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_getElementsByClassName_a5ef560ae6918226 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementsByClassName(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
});

const __wbg_getwithindex_bcf1a04b716019a9 = logError(function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_setAttribute_fb8737b4573a65f8 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
});

const __wbg_getContext_d277f710e8035242 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_instanceof_CanvasRenderingContext2d_fbca10ed951560f3 = logError(function(arg0) {
    var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
    _assertBoolean(ret);
    return ret;
});

const __wbg_scale_e0fdce059098cd1b = handleError(function(arg0, arg1, arg2) {
    getObject(arg0).scale(arg1, arg2);
});

const __wbg_texImage2D_bc294af8c1a6a435 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
});

const __wbg_log_a39f164b49616cb0 = logError(function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
});

const __wbg_newwithbyteoffsetandlength_0c7ac30665ee26f8 = logError(function(arg0, arg1, arg2) {
    var ret = new Int32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_new_c77df81d6c892c35 = logError(function(arg0) {
    var ret = new Int32Array(getObject(arg0));
    return addHeapObject(ret);
});

const __wbg_newwithbyteoffsetandlength_836859e5deb44d3f = logError(function(arg0, arg1, arg2) {
    var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_new_139e70222494b1ff = logError(function(arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
});

const __wbg_newwithbyteoffsetandlength_c274c3296a37fcb4 = logError(function(arg0, arg1, arg2) {
    var ret = new Int16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_new_5b74a8dd0c5b71ac = logError(function(arg0) {
    var ret = new Int16Array(getObject(arg0));
    return addHeapObject(ret);
});

const __wbg_vertexAttribDivisor_bf07aa5a1a9fc2d1 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).vertexAttribDivisor(arg1 >>> 0, arg2 >>> 0);
});

const __wbg_deleteBuffer_988823f4e76e697d = logError(function(arg0, arg1) {
    getObject(arg0).deleteBuffer(getObject(arg1));
});

const __wbg_createProgram_128698dd90ec070d = logError(function(arg0) {
    var ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_attachShader_5d53b7b00823cafb = logError(function(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
});

const __wbg_linkProgram_370ed11b34456c89 = logError(function(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
});

const __wbg_getProgramParameter_b949ba1d9662f6a2 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    _assertNum(ret);
    return ret;
};

const __wbg_getProgramInfoLog_f8f65be65281f691 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

const __wbg_getActiveUniform_70b770a58f551f8f = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getActiveUniform(getObject(arg1), arg2 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_name_4f3b7294acbeabad = logError(function(arg0, arg1) {
    var ret = getObject(arg1).name;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

const __wbg_getUniformLocation_472b7459010900a5 = logError(function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_createShader_26e4f959d5d64d80 = logError(function(arg0, arg1) {
    var ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_shaderSource_96ace5133c032f2f = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
});

const __wbg_compileShader_f7e245515fa1405d = logError(function(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
});

const __wbg_getShaderParameter_cced0ff8ba83f3e7 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_getShaderInfoLog_5412e8bc642139e8 = logError(function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

const __wbg_clearColor_fc22409197a5bd68 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
});

const __wbg_clear_25e035ed3961f1c6 = logError(function(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
});

const __wbg_width_d9e3643c351ff015 = logError(function(arg0) {
    var ret = getObject(arg0).width;
    _assertNum(ret);
    return ret;
});

const __wbg_height_b92a879a29e66010 = logError(function(arg0) {
    var ret = getObject(arg0).height;
    _assertNum(ret);
    return ret;
});

const __wbg_uniform1f_fa50abe89ff891ea = logError(function(arg0, arg1, arg2) {
    getObject(arg0).uniform1f(getObject(arg1), arg2);
});

const __wbg_uniform2f_ab7c909be2949448 = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
});

const __wbg_drawElementsInstanced_6a606cd25bdbafb3 = logError(function(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawElementsInstanced(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
});

const __wbg_drawElements_c109bfea7998fd99 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
});

const __wbg_uniform4f_9941fe9c32da60ea = logError(function(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
});

const __wbg_drawArrays_f6e7af9c06f4f4ae = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
});

const __wbg_uniform1i_a1e8f5ad954fa6b5 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
});

const __wbg_clearRect_620b55f817af6080 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
});

const __wbg_setfont_7d7b206c4c017729 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).font = getStringFromWasm0(arg1, arg2);
});

const __wbg_settextAlign_0ab90671be8e1137 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).textAlign = getStringFromWasm0(arg1, arg2);
});

const __wbg_setfillStyle_1b068f8d99084158 = logError(function(arg0, arg1) {
    getObject(arg0).fillStyle = getObject(arg1);
});

const __wbg_save_be2f4340f20bfe6f = logError(function(arg0) {
    getObject(arg0).save();
});

const __wbg_translate_a603cdd310297ee8 = handleError(function(arg0, arg1, arg2) {
    getObject(arg0).translate(arg1, arg2);
});

const __wbg_rotate_4ae42333a58388ed = handleError(function(arg0, arg1) {
    getObject(arg0).rotate(arg1);
});

const __wbg_fillText_aee0d6016521a3b2 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
});

const __wbg_restore_e6861230b7a8a25e = logError(function(arg0) {
    getObject(arg0).restore();
});

const __wbg_disable_827be6d0f77447e1 = logError(function(arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
});

const __wbg_parse_b89e797098b3bc7b = handleError(function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
});

const __wbg_getContext_a5ae0a2c4fe6f42b = handleError(function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

const __wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb = logError(function(arg0) {
    var ret = getObject(arg0) instanceof WebGL2RenderingContext;
    _assertBoolean(ret);
    return ret;
});

const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    _assertBoolean(ret);
    return ret;
};

const __wbindgen_json_parse = function(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

const __wbg_uniform1fv_7b33ccba8ca090e4 = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
});

const __wbg_uniformMatrix4fv_82825540b9315680 = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
});

const __wbg_uniform1iv_2c8af1d8286865f0 = logError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform1iv(getObject(arg1), getArrayI32FromWasm0(arg2, arg3));
});

const __wbg_uniform3f_a7c04d3d1c2b18aa = logError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
});

const __wbg_isArray_3320300beb1837ab = logError(function(arg0) {
    var ret = Array.isArray(getObject(arg0));
    _assertBoolean(ret);
    return ret;
});

const __wbg_length_8f15bbb4ecbf7e33 = logError(function(arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
});

const __wbg_get_40375c2067f479fc = logError(function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
});

const __wbg_response_c70a68323728a385 = handleError(function(arg0) {
    var ret = getObject(arg0).response;
    return addHeapObject(ret);
});

const __wbg_length_2cfa674c2a529bc1 = logError(function(arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
});

const __wbg_set_d771848e3c7935bb = logError(function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
});

const __wbg_new_59cb74e423758ede = logError(function() {
    var ret = new Error();
    return addHeapObject(ret);
});

const __wbg_stack_558ba5917b466edd = logError(function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

const __wbg_error_4bb6c2a97407129a = logError(function(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](arg0, arg1);
    }
});

const __wbg_deleteVertexArray_3f37aabaae61ca26 = logError(function(arg0, arg1) {
    getObject(arg0).deleteVertexArray(getObject(arg1));
});

const __wbg_disableVertexAttribArray_1401ee870505cf02 = logError(function(arg0, arg1) {
    getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
});

const __wbg_deleteTexture_cf22af6782ebc54f = logError(function(arg0, arg1) {
    getObject(arg0).deleteTexture(getObject(arg1));
});

const __wbg_self_eeabd9085c04fc17 = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

const __wbg_window_f110c13310da2c8f = handleError(function() {
    var ret = window.window;
    return addHeapObject(ret);
});

const __wbg_globalThis_a2669bee93faee43 = handleError(function() {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
});

const __wbg_global_a5584d717f4d6761 = handleError(function() {
    var ret = global.global;
    return addHeapObject(ret);
});

const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

const __wbg_newnoargs_179d393e4626fcf7 = logError(function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
});

const __wbg_call_8487a9f580e47219 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
});

const __wbg_newwithbyteoffsetandlength_a20c8edf0fedac40 = logError(function(arg0, arg1, arg2) {
    var ret = new Uint16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbg_newwithbyteoffsetandlength_7b9a415096aef9c1 = logError(function(arg0, arg1, arg2) {
    var ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
});

const __wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"], _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_realloc"]);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

const __wbg_blendFunc_8bd5998b54c12fd3 = logError(function(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
});

const __wbg_open_c1608202d44b7d1c = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), arg5 !== 0);
});

const __wbg_setonload_c71ccab98777e104 = logError(function(arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
});

const __wbg_setonerror_1bea8ceda68d0d63 = logError(function(arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
});

const __wbg_send_3e459af287bba919 = handleError(function(arg0) {
    getObject(arg0).send();
});

const __wbg_texSubImage2D_43d09711529aa698 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, getObject(arg7));
});

const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

const __wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

const __wbg_instanceof_Window_fa4595281eb5ba83 = logError(function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    _assertBoolean(ret);
    return ret;
});

const __wbindgen_closure_wrapper166 = logError(function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 5, __wbg_adapter_28);
    return addHeapObject(ret);
});

const __wbindgen_closure_wrapper170 = logError(function(arg0, arg1, arg2) {
    var ret = makeClosure(arg0, arg1, 5, __wbg_adapter_31);
    return addHeapObject(ret);
});


/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! text-encoding */ "./node_modules/text-encoding/index.js")["TextEncoder"], __webpack_require__(/*! ./../../../node_modules/webpack/buildin/harmony-module.js */ "./node_modules/webpack/buildin/harmony-module.js")(module), __webpack_require__(/*! text-encoding */ "./node_modules/text-encoding/index.js")["TextDecoder"], __webpack_require__(/*! ./../../../node_modules/webpack/buildin/global.js */ "./node_modules/webpack/buildin/global.js")))

/***/ }),

/***/ "./src/core/pkg/index_bg.wasm":
/*!************************************!*\
  !*** ./src/core/pkg/index_bg.wasm ***!
  \************************************/
/*! exports provided: memory, __wbg_webclient_free, webclient_new, webclient_update, webclient_resize, webclient_render, webclient_setProjection, webclient_setLongitudeReversed, webclient_isReady, webclient_setImageSurveys, webclient_moveImageSurveysLayerForward, webclient_setOpacityLayer, webclient_setGridColor, webclient_enableGrid, webclient_disableGrid, webclient_hideGridLabels, webclient_showGridLabels, webclient_cooSystem, webclient_setCooSystem, webclient_J20002Gal, webclient_Gal2J2000, webclient_getFieldOfView, webclient_setFieldOfView, webclient_setRotationAroundCenter, webclient_getRotationAroundCenter, webclient_getMaxFieldOfView, webclient_getClipZoomFactor, webclient_setCenter, webclient_getCenter, webclient_moveToLocation, webclient_goFromTo, webclient_worldToScreen, webclient_worldToScreenVec, webclient_screenToWorld, webclient_releaseLeftButtonMouse, webclient_pressLeftMouseButton, webclient_registerWheelEvent, webclient_addCatalog, webclient_isCatalogLoaded, webclient_setCatalogColormap, webclient_setCatalogOpacity, webclient_setCatalogKernelStrength, webclient_projectLine, webclient_getAvailableColormapList, GALCooSys, ICRSJ2000CooSys, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_export_2, _dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h47dd8a349271e080, _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h342357a9a09a4a44, __wbindgen_add_to_stack_pointer, __wbindgen_free, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.i];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name != "__webpack_init__") exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./index_bg.js */ "./src/core/pkg/index_bg.js");


// exec wasm module
wasmExports["__webpack_init__"]()

/***/ })

}]);
//# sourceMappingURL=1.aladin.js.map