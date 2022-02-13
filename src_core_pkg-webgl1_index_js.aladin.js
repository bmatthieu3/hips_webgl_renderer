"use strict";
(self["webpackChunkhips_webgl_renderer"] = self["webpackChunkhips_webgl_renderer"] || []).push([["src_core_pkg-webgl1_index_js"],{

/***/ "./src/core/pkg-webgl1/index.js":
/*!**************************************!*\
  !*** ./src/core/pkg-webgl1/index.js ***!
  \**************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "CooSystem": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.CooSystem),
/* harmony export */   "GALCooSys": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.GALCooSys),
/* harmony export */   "ICRSJ2000CooSys": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.ICRSJ2000CooSys),
/* harmony export */   "WebClient": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.WebClient),
/* harmony export */   "__wbg_activeTexture_74ed11a5c5d5af90": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_activeTexture_74ed11a5c5d5af90),
/* harmony export */   "__wbg_addEventListener_6bdba88519fdc1c9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_addEventListener_6bdba88519fdc1c9),
/* harmony export */   "__wbg_altKey_773e7f8151c49bb1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_altKey_773e7f8151c49bb1),
/* harmony export */   "__wbg_appendChild_3fe5090c665d3bb4": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_appendChild_3fe5090c665d3bb4),
/* harmony export */   "__wbg_arrayBuffer_8b5364ee9b393098": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_arrayBuffer_8b5364ee9b393098),
/* harmony export */   "__wbg_attachShader_55dbe770f3ee32ca": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_attachShader_55dbe770f3ee32ca),
/* harmony export */   "__wbg_bindBuffer_29d52e7bc48650c3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_bindBuffer_29d52e7bc48650c3),
/* harmony export */   "__wbg_bindFramebuffer_bd35ddd23765c7b6": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_bindFramebuffer_bd35ddd23765c7b6),
/* harmony export */   "__wbg_bindTexture_198c816345baca83": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_bindTexture_198c816345baca83),
/* harmony export */   "__wbg_blendEquation_09d56f3be6f914f5": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_blendEquation_09d56f3be6f914f5),
/* harmony export */   "__wbg_blendFuncSeparate_494b1dae028cb9a9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_blendFuncSeparate_494b1dae028cb9a9),
/* harmony export */   "__wbg_blendFunc_c8f1e0fb4467f57c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_blendFunc_c8f1e0fb4467f57c),
/* harmony export */   "__wbg_blur_2156876090506146": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_blur_2156876090506146),
/* harmony export */   "__wbg_body_7538539844356c1c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_body_7538539844356c1c),
/* harmony export */   "__wbg_bufferData_85d635f32a990208": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_bufferData_85d635f32a990208),
/* harmony export */   "__wbg_bufferSubData_3a944e1fdad0cd9a": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_bufferSubData_3a944e1fdad0cd9a),
/* harmony export */   "__wbg_buffer_5e74a88a1424a2e0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_buffer_5e74a88a1424a2e0),
/* harmony export */   "__wbg_button_a18f33eb55774d89": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_button_a18f33eb55774d89),
/* harmony export */   "__wbg_call_89558c3e96703ca1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_call_89558c3e96703ca1),
/* harmony export */   "__wbg_canvas_d0b58be124e596e3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_canvas_d0b58be124e596e3),
/* harmony export */   "__wbg_changedTouches_363278e8a9a95419": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_changedTouches_363278e8a9a95419),
/* harmony export */   "__wbg_clearColor_51c4f69c743c3252": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_clearColor_51c4f69c743c3252),
/* harmony export */   "__wbg_clear_2af1271959ec83d7": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_clear_2af1271959ec83d7),
/* harmony export */   "__wbg_clientX_849ccdf456d662ac": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_clientX_849ccdf456d662ac),
/* harmony export */   "__wbg_clientY_1aaff30fe0cd0876": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_clientY_1aaff30fe0cd0876),
/* harmony export */   "__wbg_compileShader_3b5f9ef4c67a0777": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_compileShader_3b5f9ef4c67a0777),
/* harmony export */   "__wbg_createBuffer_c40f37e1348bb91f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createBuffer_c40f37e1348bb91f),
/* harmony export */   "__wbg_createElement_d017b8d2af99bab9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createElement_d017b8d2af99bab9),
/* harmony export */   "__wbg_createFramebuffer_410b12a5cc5a8f13": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createFramebuffer_410b12a5cc5a8f13),
/* harmony export */   "__wbg_createProgram_245520da1fb9e47b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createProgram_245520da1fb9e47b),
/* harmony export */   "__wbg_createShader_4d8818a13cb825b3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createShader_4d8818a13cb825b3),
/* harmony export */   "__wbg_createTexture_f3a6a715d6bada45": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_createTexture_f3a6a715d6bada45),
/* harmony export */   "__wbg_ctrlKey_4e536bedb069129f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_ctrlKey_4e536bedb069129f),
/* harmony export */   "__wbg_ctrlKey_8c7ff99be598479e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_ctrlKey_8c7ff99be598479e),
/* harmony export */   "__wbg_cullFace_c6fb8a7309c36a38": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_cullFace_c6fb8a7309c36a38),
/* harmony export */   "__wbg_dataTransfer_bc4c0501385a0c8e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_dataTransfer_bc4c0501385a0c8e),
/* harmony export */   "__wbg_data_9562112603a9aa89": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_data_9562112603a9aa89),
/* harmony export */   "__wbg_deleteBuffer_c708688b9e1b3518": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deleteBuffer_c708688b9e1b3518),
/* harmony export */   "__wbg_deleteFramebuffer_ca006f8649d4550a": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deleteFramebuffer_ca006f8649d4550a),
/* harmony export */   "__wbg_deleteTexture_9159fb5927ed32c0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deleteTexture_9159fb5927ed32c0),
/* harmony export */   "__wbg_deltaMode_ed9d7974a0c11323": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deltaMode_ed9d7974a0c11323),
/* harmony export */   "__wbg_deltaX_df228181f4d1a561": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deltaX_df228181f4d1a561),
/* harmony export */   "__wbg_deltaY_afa6edde136e1500": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_deltaY_afa6edde136e1500),
/* harmony export */   "__wbg_devicePixelRatio_9632545370d525ae": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_devicePixelRatio_9632545370d525ae),
/* harmony export */   "__wbg_disableVertexAttribArray_aa8458b40dd08914": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_disableVertexAttribArray_aa8458b40dd08914),
/* harmony export */   "__wbg_disable_2b63b75dc6c27537": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_disable_2b63b75dc6c27537),
/* harmony export */   "__wbg_document_5edd43643d1060d9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_document_5edd43643d1060d9),
/* harmony export */   "__wbg_drawArrays_22c88d644a33fd59": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_drawArrays_22c88d644a33fd59),
/* harmony export */   "__wbg_drawElementsInstancedANGLE_e184bb1bad14df88": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_drawElementsInstancedANGLE_e184bb1bad14df88),
/* harmony export */   "__wbg_drawElements_6e26500a25ecf478": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_drawElements_6e26500a25ecf478),
/* harmony export */   "__wbg_enableVertexAttribArray_4ed5f91d0718bee1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_enableVertexAttribArray_4ed5f91d0718bee1),
/* harmony export */   "__wbg_enable_8f6dd779ccb8e1de": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_enable_8f6dd779ccb8e1de),
/* harmony export */   "__wbg_error_4bb6c2a97407129a": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_4bb6c2a97407129a),
/* harmony export */   "__wbg_error_ca520cb687b085a1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_ca520cb687b085a1),
/* harmony export */   "__wbg_fetchSurveyMetadata_3d518f6be78ba7d4": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_fetchSurveyMetadata_3d518f6be78ba7d4),
/* harmony export */   "__wbg_files_a4192b4f5967317b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_files_a4192b4f5967317b),
/* harmony export */   "__wbg_focus_4434360545ac99cf": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_focus_4434360545ac99cf),
/* harmony export */   "__wbg_force_8e51e1fec066aade": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_force_8e51e1fec066aade),
/* harmony export */   "__wbg_framebufferTexture2D_31643260e5b0b294": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_framebufferTexture2D_31643260e5b0b294),
/* harmony export */   "__wbg_getActiveUniform_3851244f8fc5db53": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getActiveUniform_3851244f8fc5db53),
/* harmony export */   "__wbg_getAttribLocation_da5df7094096113d": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getAttribLocation_da5df7094096113d),
/* harmony export */   "__wbg_getBoundingClientRect_534c1b96b6e612d3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getBoundingClientRect_534c1b96b6e612d3),
/* harmony export */   "__wbg_getContext_10d5c2a4cc0737c8": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getContext_10d5c2a4cc0737c8),
/* harmony export */   "__wbg_getElementById_b30e88aff96f66a1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getElementById_b30e88aff96f66a1),
/* harmony export */   "__wbg_getElementsByClassName_8a7d00ed3eaf1522": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getElementsByClassName_8a7d00ed3eaf1522),
/* harmony export */   "__wbg_getExtension_c6ceee3244ee7f20": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getExtension_c6ceee3244ee7f20),
/* harmony export */   "__wbg_getProgramInfoLog_c253042b64e86027": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getProgramInfoLog_c253042b64e86027),
/* harmony export */   "__wbg_getProgramParameter_4f698af0dda0a2d4": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getProgramParameter_4f698af0dda0a2d4),
/* harmony export */   "__wbg_getPropertyValue_fd6ae3726bda9d7f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getPropertyValue_fd6ae3726bda9d7f),
/* harmony export */   "__wbg_getShaderInfoLog_584794e3bcf1e19b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getShaderInfoLog_584794e3bcf1e19b),
/* harmony export */   "__wbg_getShaderParameter_64b1ffe576e5fa25": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getShaderParameter_64b1ffe576e5fa25),
/* harmony export */   "__wbg_getUniformLocation_703972f150a46500": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getUniformLocation_703972f150a46500),
/* harmony export */   "__wbg_get_1c01a7682a9775bb": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_1c01a7682a9775bb),
/* harmony export */   "__wbg_get_a765dab923455e0d": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_a765dab923455e0d),
/* harmony export */   "__wbg_get_bdec89fd60d07530": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_bdec89fd60d07530),
/* harmony export */   "__wbg_get_f45dff51f52d7222": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_f45dff51f52d7222),
/* harmony export */   "__wbg_getwithindex_5caaba1b5b3e6e18": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_getwithindex_5caaba1b5b3e6e18),
/* harmony export */   "__wbg_globalThis_d61b1f48a57191ae": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_globalThis_d61b1f48a57191ae),
/* harmony export */   "__wbg_global_e7669da72fd7f239": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_global_e7669da72fd7f239),
/* harmony export */   "__wbg_height_133772b066cfc559": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_height_133772b066cfc559),
/* harmony export */   "__wbg_height_1b399500ca683487": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_height_1b399500ca683487),
/* harmony export */   "__wbg_hidden_f7a620ec4ab18ce5": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_hidden_f7a620ec4ab18ce5),
/* harmony export */   "__wbg_id_79dca31d8297faf1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_id_79dca31d8297faf1),
/* harmony export */   "__wbg_identifier_afa8b01d4d901685": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_identifier_afa8b01d4d901685),
/* harmony export */   "__wbg_innerHeight_25d3be0d129329c3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_innerHeight_25d3be0d129329c3),
/* harmony export */   "__wbg_innerWidth_405786923c1d2641": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_innerWidth_405786923c1d2641),
/* harmony export */   "__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638),
/* harmony export */   "__wbg_instanceof_HtmlInputElement_8969541a2a0bded0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_instanceof_HtmlInputElement_8969541a2a0bded0),
/* harmony export */   "__wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362),
/* harmony export */   "__wbg_instanceof_Window_434ce1849eb4e0fc": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_instanceof_Window_434ce1849eb4e0fc),
/* harmony export */   "__wbg_isArray_8480ed76e5369634": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_isArray_8480ed76e5369634),
/* harmony export */   "__wbg_isComposing_b892666abf384da9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_isComposing_b892666abf384da9),
/* harmony export */   "__wbg_item_b192ab411bbfbb09": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_item_b192ab411bbfbb09),
/* harmony export */   "__wbg_items_d571f433ef73ee49": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_items_d571f433ef73ee49),
/* harmony export */   "__wbg_keyCode_8a05b1390fced3c8": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_keyCode_8a05b1390fced3c8),
/* harmony export */   "__wbg_key_7f10b1291a923361": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_key_7f10b1291a923361),
/* harmony export */   "__wbg_lastModified_0de23a8c5214f2fb": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_lastModified_0de23a8c5214f2fb),
/* harmony export */   "__wbg_left_0e681cb8fd277739": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_left_0e681cb8fd277739),
/* harmony export */   "__wbg_length_01a613025b5ffd74": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_01a613025b5ffd74),
/* harmony export */   "__wbg_length_30803400a8f15c59": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_30803400a8f15c59),
/* harmony export */   "__wbg_length_41b205f6892bf9d9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_41b205f6892bf9d9),
/* harmony export */   "__wbg_length_44449d3b5928d07c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_44449d3b5928d07c),
/* harmony export */   "__wbg_length_7b60f47bde714631": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_7b60f47bde714631),
/* harmony export */   "__wbg_length_a2882c668bdf6488": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_length_a2882c668bdf6488),
/* harmony export */   "__wbg_linkProgram_5fdd57237c761833": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_linkProgram_5fdd57237c761833),
/* harmony export */   "__wbg_log_a39f164b49616cb0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_a39f164b49616cb0),
/* harmony export */   "__wbg_log_fbd13631356d44e4": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_log_fbd13631356d44e4),
/* harmony export */   "__wbg_metaKey_0b396e35a4941247": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_metaKey_0b396e35a4941247),
/* harmony export */   "__wbg_metaKey_99a7d3732e1b7856": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_metaKey_99a7d3732e1b7856),
/* harmony export */   "__wbg_name_4ada8b70ffadb5c0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_name_4ada8b70ffadb5c0),
/* harmony export */   "__wbg_name_9a61dbbdbfb2d0de": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_name_9a61dbbdbfb2d0de),
/* harmony export */   "__wbg_navigator_0e0588c949560476": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_navigator_0e0588c949560476),
/* harmony export */   "__wbg_new_08dfde0f90155eb7": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_08dfde0f90155eb7),
/* harmony export */   "__wbg_new_59cb74e423758ede": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_59cb74e423758ede),
/* harmony export */   "__wbg_new_da67f111e299956e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_da67f111e299956e),
/* harmony export */   "__wbg_new_e3b800e570795b3c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_e3b800e570795b3c),
/* harmony export */   "__wbg_new_f5438c0cea22a3aa": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_f5438c0cea22a3aa),
/* harmony export */   "__wbg_newnoargs_f579424187aa1717": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newnoargs_f579424187aa1717),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_278ec7532799393a": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newwithbyteoffsetandlength_278ec7532799393a),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_bdb885cfc5e9bc43": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newwithbyteoffsetandlength_bdb885cfc5e9bc43),
/* harmony export */   "__wbg_newwithlength_5f4ce114a24dfe1e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newwithlength_5f4ce114a24dfe1e),
/* harmony export */   "__wbg_newwithlength_747b31c525d823ec": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_newwithlength_747b31c525d823ec),
/* harmony export */   "__wbg_now_5fa0ca001e042f8a": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_now_5fa0ca001e042f8a),
/* harmony export */   "__wbg_offsetLeft_be5393bf9eec5766": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_offsetLeft_be5393bf9eec5766),
/* harmony export */   "__wbg_offsetTop_45111254e7b26a1f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_offsetTop_45111254e7b26a1f),
/* harmony export */   "__wbg_offsetWidth_bc683e2f57ea2d6b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_offsetWidth_bc683e2f57ea2d6b),
/* harmony export */   "__wbg_open_67fbcd7373a90ddc": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_open_67fbcd7373a90ddc),
/* harmony export */   "__wbg_open_7190f43b39e7f488": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_open_7190f43b39e7f488),
/* harmony export */   "__wbg_pageX_e0c8221ecfdb73d0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_pageX_e0c8221ecfdb73d0),
/* harmony export */   "__wbg_pageY_32100ad7039a744e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_pageY_32100ad7039a744e),
/* harmony export */   "__wbg_parse_e3e7e590474b89d2": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_parse_e3e7e590474b89d2),
/* harmony export */   "__wbg_performance_bbca4ccfaef860b2": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_performance_bbca4ccfaef860b2),
/* harmony export */   "__wbg_preventDefault_fa00541ff125b78c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_preventDefault_fa00541ff125b78c),
/* harmony export */   "__wbg_readPixels_3692eaca9dfc7c0c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_readPixels_3692eaca9dfc7c0c),
/* harmony export */   "__wbg_requestAnimationFrame_0c71cd3c6779a371": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_requestAnimationFrame_0c71cd3c6779a371),
/* harmony export */   "__wbg_resolve_4f8f547f26b30b27": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_resolve_4f8f547f26b30b27),
/* harmony export */   "__wbg_responseURL_a3e549a88db1c1f7": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_responseURL_a3e549a88db1c1f7),
/* harmony export */   "__wbg_response_8b12ac238727ae0e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_response_8b12ac238727ae0e),
/* harmony export */   "__wbg_scissor_fb094c7db856e2a7": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_scissor_fb094c7db856e2a7),
/* harmony export */   "__wbg_scrollLeft_e8aba47a94d12290": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_scrollLeft_e8aba47a94d12290),
/* harmony export */   "__wbg_scrollTop_5ebd5c6591748d6e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_scrollTop_5ebd5c6591748d6e),
/* harmony export */   "__wbg_self_e23d74ae45fb17d1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_self_e23d74ae45fb17d1),
/* harmony export */   "__wbg_send_84c8dd943b775f78": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_send_84c8dd943b775f78),
/* harmony export */   "__wbg_setProperty_ebb06e7fa941d6a8": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setProperty_ebb06e7fa941d6a8),
/* harmony export */   "__wbg_setTimeout_1c75092906446b91": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setTimeout_1c75092906446b91),
/* harmony export */   "__wbg_set_5b8081e9d002f0df": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_5b8081e9d002f0df),
/* harmony export */   "__wbg_set_7cb6639737aebb39": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_7cb6639737aebb39),
/* harmony export */   "__wbg_setautofocus_a2ae37091dfbe4af": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setautofocus_a2ae37091dfbe4af),
/* harmony export */   "__wbg_setcrossOrigin_07e0e4935571a4c5": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setcrossOrigin_07e0e4935571a4c5),
/* harmony export */   "__wbg_setheight_28f53831182cc410": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setheight_28f53831182cc410),
/* harmony export */   "__wbg_sethidden_fdaefd7e7da7e4c0": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_sethidden_fdaefd7e7da7e4c0),
/* harmony export */   "__wbg_setid_73be37238eaa05be": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setid_73be37238eaa05be),
/* harmony export */   "__wbg_setonerror_939f617c2b40758c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setonerror_939f617c2b40758c),
/* harmony export */   "__wbg_setonerror_d665b35adb3552fb": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setonerror_d665b35adb3552fb),
/* harmony export */   "__wbg_setonload_18033df8ec5db791": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setonload_18033df8ec5db791),
/* harmony export */   "__wbg_setonload_9235de4503eb82c8": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setonload_9235de4503eb82c8),
/* harmony export */   "__wbg_setresponseType_e5326d926ee8e787": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setresponseType_e5326d926ee8e787),
/* harmony export */   "__wbg_setsize_90d1034a7a757a50": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setsize_90d1034a7a757a50),
/* harmony export */   "__wbg_setsrc_b0a1ac4dd261ae2d": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setsrc_b0a1ac4dd261ae2d),
/* harmony export */   "__wbg_setvalue_ce4a23f487065c07": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setvalue_ce4a23f487065c07),
/* harmony export */   "__wbg_setwidth_362e8db8cbadbe96": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setwidth_362e8db8cbadbe96),
/* harmony export */   "__wbg_shaderSource_173ab97288934a60": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_shaderSource_173ab97288934a60),
/* harmony export */   "__wbg_shiftKey_894b631364d8db13": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_shiftKey_894b631364d8db13),
/* harmony export */   "__wbg_size_20c167ba9040b895": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_size_20c167ba9040b895),
/* harmony export */   "__wbg_stack_558ba5917b466edd": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_558ba5917b466edd),
/* harmony export */   "__wbg_stopPropagation_da586180676fa914": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stopPropagation_da586180676fa914),
/* harmony export */   "__wbg_style_16f5dd9624687c8f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_style_16f5dd9624687c8f),
/* harmony export */   "__wbg_subarray_e729e242fb317565": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_subarray_e729e242fb317565),
/* harmony export */   "__wbg_texImage2D_b46a9b691e69d90b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_texImage2D_b46a9b691e69d90b),
/* harmony export */   "__wbg_texImage2D_d26bd916ff0956a1": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_texImage2D_d26bd916ff0956a1),
/* harmony export */   "__wbg_texParameteri_caec5468f2a850c3": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_texParameteri_caec5468f2a850c3),
/* harmony export */   "__wbg_texSubImage2D_d907a4c940fd6e41": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_texSubImage2D_d907a4c940fd6e41),
/* harmony export */   "__wbg_texSubImage2D_d9dc0ffd91998f0d": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_texSubImage2D_d9dc0ffd91998f0d),
/* harmony export */   "__wbg_then_58a04e42527f52c6": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_then_58a04e42527f52c6),
/* harmony export */   "__wbg_then_a6860c82b90816ca": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_then_a6860c82b90816ca),
/* harmony export */   "__wbg_top_a24b8b81afea659b": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_top_a24b8b81afea659b),
/* harmony export */   "__wbg_touches_7397ce4df4dceded": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_touches_7397ce4df4dceded),
/* harmony export */   "__wbg_type_8bc3e57acd2158c9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_type_8bc3e57acd2158c9),
/* harmony export */   "__wbg_type_e32f387f5584c765": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_type_e32f387f5584c765),
/* harmony export */   "__wbg_uniform1f_258478814234cf9c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform1f_258478814234cf9c),
/* harmony export */   "__wbg_uniform1fv_e6a2134edff4f2e9": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform1fv_e6a2134edff4f2e9),
/* harmony export */   "__wbg_uniform1i_a0275676828a22b6": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform1i_a0275676828a22b6),
/* harmony export */   "__wbg_uniform2f_b0af46ba675f2c0d": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform2f_b0af46ba675f2c0d),
/* harmony export */   "__wbg_uniform3f_65416973a351fbab": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform3f_65416973a351fbab),
/* harmony export */   "__wbg_uniform4f_e5d0a91bf98b35ad": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniform4f_e5d0a91bf98b35ad),
/* harmony export */   "__wbg_uniformMatrix2fv_aaa4e0f7c15bca04": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniformMatrix2fv_aaa4e0f7c15bca04),
/* harmony export */   "__wbg_uniformMatrix4fv_f07c6caf5a563616": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_uniformMatrix4fv_f07c6caf5a563616),
/* harmony export */   "__wbg_useProgram_d5898a40ebe88916": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_useProgram_d5898a40ebe88916),
/* harmony export */   "__wbg_userAgent_3f63af8b4fe2331c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_userAgent_3f63af8b4fe2331c),
/* harmony export */   "__wbg_value_fc1c354d1a0e9714": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_value_fc1c354d1a0e9714),
/* harmony export */   "__wbg_vertexAttribDivisorANGLE_2dc41a79843a435c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_vertexAttribDivisorANGLE_2dc41a79843a435c),
/* harmony export */   "__wbg_vertexAttribPointer_0d097efa33e3f45f": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_vertexAttribPointer_0d097efa33e3f45f),
/* harmony export */   "__wbg_viewport_19577064127daf83": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_viewport_19577064127daf83),
/* harmony export */   "__wbg_warn_97f10a6b0dbb8c5c": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_warn_97f10a6b0dbb8c5c),
/* harmony export */   "__wbg_width_6c4cad65073b3852": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_width_6c4cad65073b3852),
/* harmony export */   "__wbg_width_cfa982e2a6ad6297": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_width_cfa982e2a6ad6297),
/* harmony export */   "__wbg_window_b4be7f48b24ac56e": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_window_b4be7f48b24ac56e),
/* harmony export */   "__wbindgen_boolean_get": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_boolean_get),
/* harmony export */   "__wbindgen_cb_drop": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_cb_drop),
/* harmony export */   "__wbindgen_closure_wrapper1458": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper1458),
/* harmony export */   "__wbindgen_closure_wrapper1460": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper1460),
/* harmony export */   "__wbindgen_closure_wrapper2297": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2297),
/* harmony export */   "__wbindgen_closure_wrapper2298": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2298),
/* harmony export */   "__wbindgen_closure_wrapper2300": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2300),
/* harmony export */   "__wbindgen_closure_wrapper2302": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2302),
/* harmony export */   "__wbindgen_closure_wrapper2304": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2304),
/* harmony export */   "__wbindgen_closure_wrapper2306": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2306),
/* harmony export */   "__wbindgen_closure_wrapper2308": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2308),
/* harmony export */   "__wbindgen_closure_wrapper2310": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2310),
/* harmony export */   "__wbindgen_closure_wrapper2313": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2313),
/* harmony export */   "__wbindgen_closure_wrapper2456": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_closure_wrapper2456),
/* harmony export */   "__wbindgen_debug_string": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_debug_string),
/* harmony export */   "__wbindgen_is_undefined": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_is_undefined),
/* harmony export */   "__wbindgen_json_parse": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_json_parse),
/* harmony export */   "__wbindgen_json_serialize": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_json_serialize),
/* harmony export */   "__wbindgen_memory": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_memory),
/* harmony export */   "__wbindgen_number_get": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_number_get),
/* harmony export */   "__wbindgen_object_clone_ref": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_clone_ref),
/* harmony export */   "__wbindgen_object_drop_ref": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),
/* harmony export */   "__wbindgen_string_new": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_string_new),
/* harmony export */   "__wbindgen_throw": () => (/* reexport safe */ _index_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)
/* harmony export */ });
/* harmony import */ var _index_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.js */ "./src/core/pkg-webgl1/index_bg.js");



/***/ }),

/***/ "./src/core/pkg-webgl1/index_bg.js":
/*!*****************************************!*\
  !*** ./src/core/pkg-webgl1/index_bg.js ***!
  \*****************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "GALCooSys": () => (/* binding */ GALCooSys),
/* harmony export */   "ICRSJ2000CooSys": () => (/* binding */ ICRSJ2000CooSys),
/* harmony export */   "CooSystem": () => (/* binding */ CooSystem),
/* harmony export */   "WebClient": () => (/* binding */ WebClient),
/* harmony export */   "__wbindgen_string_new": () => (/* binding */ __wbindgen_string_new),
/* harmony export */   "__wbindgen_object_drop_ref": () => (/* binding */ __wbindgen_object_drop_ref),
/* harmony export */   "__wbindgen_object_clone_ref": () => (/* binding */ __wbindgen_object_clone_ref),
/* harmony export */   "__wbindgen_cb_drop": () => (/* binding */ __wbindgen_cb_drop),
/* harmony export */   "__wbindgen_number_get": () => (/* binding */ __wbindgen_number_get),
/* harmony export */   "__wbindgen_json_parse": () => (/* binding */ __wbindgen_json_parse),
/* harmony export */   "__wbindgen_json_serialize": () => (/* binding */ __wbindgen_json_serialize),
/* harmony export */   "__wbg_fetchSurveyMetadata_3d518f6be78ba7d4": () => (/* binding */ __wbg_fetchSurveyMetadata_3d518f6be78ba7d4),
/* harmony export */   "__wbindgen_boolean_get": () => (/* binding */ __wbindgen_boolean_get),
/* harmony export */   "__wbg_log_a39f164b49616cb0": () => (/* binding */ __wbg_log_a39f164b49616cb0),
/* harmony export */   "__wbg_instanceof_HtmlCanvasElement_a6157e470d06b638": () => (/* binding */ __wbg_instanceof_HtmlCanvasElement_a6157e470d06b638),
/* harmony export */   "__wbg_width_cfa982e2a6ad6297": () => (/* binding */ __wbg_width_cfa982e2a6ad6297),
/* harmony export */   "__wbg_setwidth_362e8db8cbadbe96": () => (/* binding */ __wbg_setwidth_362e8db8cbadbe96),
/* harmony export */   "__wbg_height_1b399500ca683487": () => (/* binding */ __wbg_height_1b399500ca683487),
/* harmony export */   "__wbg_setheight_28f53831182cc410": () => (/* binding */ __wbg_setheight_28f53831182cc410),
/* harmony export */   "__wbg_getContext_10d5c2a4cc0737c8": () => (/* binding */ __wbg_getContext_10d5c2a4cc0737c8),
/* harmony export */   "__wbg_touches_7397ce4df4dceded": () => (/* binding */ __wbg_touches_7397ce4df4dceded),
/* harmony export */   "__wbg_changedTouches_363278e8a9a95419": () => (/* binding */ __wbg_changedTouches_363278e8a9a95419),
/* harmony export */   "__wbg_type_e32f387f5584c765": () => (/* binding */ __wbg_type_e32f387f5584c765),
/* harmony export */   "__wbg_preventDefault_fa00541ff125b78c": () => (/* binding */ __wbg_preventDefault_fa00541ff125b78c),
/* harmony export */   "__wbg_stopPropagation_da586180676fa914": () => (/* binding */ __wbg_stopPropagation_da586180676fa914),
/* harmony export */   "__wbg_keyCode_8a05b1390fced3c8": () => (/* binding */ __wbg_keyCode_8a05b1390fced3c8),
/* harmony export */   "__wbg_altKey_773e7f8151c49bb1": () => (/* binding */ __wbg_altKey_773e7f8151c49bb1),
/* harmony export */   "__wbg_ctrlKey_8c7ff99be598479e": () => (/* binding */ __wbg_ctrlKey_8c7ff99be598479e),
/* harmony export */   "__wbg_shiftKey_894b631364d8db13": () => (/* binding */ __wbg_shiftKey_894b631364d8db13),
/* harmony export */   "__wbg_metaKey_99a7d3732e1b7856": () => (/* binding */ __wbg_metaKey_99a7d3732e1b7856),
/* harmony export */   "__wbg_isComposing_b892666abf384da9": () => (/* binding */ __wbg_isComposing_b892666abf384da9),
/* harmony export */   "__wbg_key_7f10b1291a923361": () => (/* binding */ __wbg_key_7f10b1291a923361),
/* harmony export */   "__wbg_setsrc_b0a1ac4dd261ae2d": () => (/* binding */ __wbg_setsrc_b0a1ac4dd261ae2d),
/* harmony export */   "__wbg_setcrossOrigin_07e0e4935571a4c5": () => (/* binding */ __wbg_setcrossOrigin_07e0e4935571a4c5),
/* harmony export */   "__wbg_width_6c4cad65073b3852": () => (/* binding */ __wbg_width_6c4cad65073b3852),
/* harmony export */   "__wbg_height_133772b066cfc559": () => (/* binding */ __wbg_height_133772b066cfc559),
/* harmony export */   "__wbg_new_da67f111e299956e": () => (/* binding */ __wbg_new_da67f111e299956e),
/* harmony export */   "__wbg_type_8bc3e57acd2158c9": () => (/* binding */ __wbg_type_8bc3e57acd2158c9),
/* harmony export */   "__wbg_userAgent_3f63af8b4fe2331c": () => (/* binding */ __wbg_userAgent_3f63af8b4fe2331c),
/* harmony export */   "__wbg_name_9a61dbbdbfb2d0de": () => (/* binding */ __wbg_name_9a61dbbdbfb2d0de),
/* harmony export */   "__wbg_lastModified_0de23a8c5214f2fb": () => (/* binding */ __wbg_lastModified_0de23a8c5214f2fb),
/* harmony export */   "__wbg_name_4ada8b70ffadb5c0": () => (/* binding */ __wbg_name_4ada8b70ffadb5c0),
/* harmony export */   "__wbg_error_ca520cb687b085a1": () => (/* binding */ __wbg_error_ca520cb687b085a1),
/* harmony export */   "__wbg_log_fbd13631356d44e4": () => (/* binding */ __wbg_log_fbd13631356d44e4),
/* harmony export */   "__wbg_warn_97f10a6b0dbb8c5c": () => (/* binding */ __wbg_warn_97f10a6b0dbb8c5c),
/* harmony export */   "__wbg_deltaX_df228181f4d1a561": () => (/* binding */ __wbg_deltaX_df228181f4d1a561),
/* harmony export */   "__wbg_deltaY_afa6edde136e1500": () => (/* binding */ __wbg_deltaY_afa6edde136e1500),
/* harmony export */   "__wbg_deltaMode_ed9d7974a0c11323": () => (/* binding */ __wbg_deltaMode_ed9d7974a0c11323),
/* harmony export */   "__wbg_setonerror_d665b35adb3552fb": () => (/* binding */ __wbg_setonerror_d665b35adb3552fb),
/* harmony export */   "__wbg_setonload_18033df8ec5db791": () => (/* binding */ __wbg_setonload_18033df8ec5db791),
/* harmony export */   "__wbg_addEventListener_6bdba88519fdc1c9": () => (/* binding */ __wbg_addEventListener_6bdba88519fdc1c9),
/* harmony export */   "__wbg_size_20c167ba9040b895": () => (/* binding */ __wbg_size_20c167ba9040b895),
/* harmony export */   "__wbg_arrayBuffer_8b5364ee9b393098": () => (/* binding */ __wbg_arrayBuffer_8b5364ee9b393098),
/* harmony export */   "__wbg_items_d571f433ef73ee49": () => (/* binding */ __wbg_items_d571f433ef73ee49),
/* harmony export */   "__wbg_files_a4192b4f5967317b": () => (/* binding */ __wbg_files_a4192b4f5967317b),
/* harmony export */   "__wbg_body_7538539844356c1c": () => (/* binding */ __wbg_body_7538539844356c1c),
/* harmony export */   "__wbg_createElement_d017b8d2af99bab9": () => (/* binding */ __wbg_createElement_d017b8d2af99bab9),
/* harmony export */   "__wbg_getElementById_b30e88aff96f66a1": () => (/* binding */ __wbg_getElementById_b30e88aff96f66a1),
/* harmony export */   "__wbg_data_9562112603a9aa89": () => (/* binding */ __wbg_data_9562112603a9aa89),
/* harmony export */   "__wbg_instanceof_HtmlInputElement_8969541a2a0bded0": () => (/* binding */ __wbg_instanceof_HtmlInputElement_8969541a2a0bded0),
/* harmony export */   "__wbg_setautofocus_a2ae37091dfbe4af": () => (/* binding */ __wbg_setautofocus_a2ae37091dfbe4af),
/* harmony export */   "__wbg_setsize_90d1034a7a757a50": () => (/* binding */ __wbg_setsize_90d1034a7a757a50),
/* harmony export */   "__wbg_value_fc1c354d1a0e9714": () => (/* binding */ __wbg_value_fc1c354d1a0e9714),
/* harmony export */   "__wbg_setvalue_ce4a23f487065c07": () => (/* binding */ __wbg_setvalue_ce4a23f487065c07),
/* harmony export */   "__wbg_now_5fa0ca001e042f8a": () => (/* binding */ __wbg_now_5fa0ca001e042f8a),
/* harmony export */   "__wbg_appendChild_3fe5090c665d3bb4": () => (/* binding */ __wbg_appendChild_3fe5090c665d3bb4),
/* harmony export */   "__wbg_top_a24b8b81afea659b": () => (/* binding */ __wbg_top_a24b8b81afea659b),
/* harmony export */   "__wbg_left_0e681cb8fd277739": () => (/* binding */ __wbg_left_0e681cb8fd277739),
/* harmony export */   "__wbg_identifier_afa8b01d4d901685": () => (/* binding */ __wbg_identifier_afa8b01d4d901685),
/* harmony export */   "__wbg_pageX_e0c8221ecfdb73d0": () => (/* binding */ __wbg_pageX_e0c8221ecfdb73d0),
/* harmony export */   "__wbg_pageY_32100ad7039a744e": () => (/* binding */ __wbg_pageY_32100ad7039a744e),
/* harmony export */   "__wbg_force_8e51e1fec066aade": () => (/* binding */ __wbg_force_8e51e1fec066aade),
/* harmony export */   "__wbg_getPropertyValue_fd6ae3726bda9d7f": () => (/* binding */ __wbg_getPropertyValue_fd6ae3726bda9d7f),
/* harmony export */   "__wbg_setProperty_ebb06e7fa941d6a8": () => (/* binding */ __wbg_setProperty_ebb06e7fa941d6a8),
/* harmony export */   "__wbg_getwithindex_5caaba1b5b3e6e18": () => (/* binding */ __wbg_getwithindex_5caaba1b5b3e6e18),
/* harmony export */   "__wbg_instanceof_Window_434ce1849eb4e0fc": () => (/* binding */ __wbg_instanceof_Window_434ce1849eb4e0fc),
/* harmony export */   "__wbg_document_5edd43643d1060d9": () => (/* binding */ __wbg_document_5edd43643d1060d9),
/* harmony export */   "__wbg_navigator_0e0588c949560476": () => (/* binding */ __wbg_navigator_0e0588c949560476),
/* harmony export */   "__wbg_innerWidth_405786923c1d2641": () => (/* binding */ __wbg_innerWidth_405786923c1d2641),
/* harmony export */   "__wbg_innerHeight_25d3be0d129329c3": () => (/* binding */ __wbg_innerHeight_25d3be0d129329c3),
/* harmony export */   "__wbg_devicePixelRatio_9632545370d525ae": () => (/* binding */ __wbg_devicePixelRatio_9632545370d525ae),
/* harmony export */   "__wbg_performance_bbca4ccfaef860b2": () => (/* binding */ __wbg_performance_bbca4ccfaef860b2),
/* harmony export */   "__wbg_open_67fbcd7373a90ddc": () => (/* binding */ __wbg_open_67fbcd7373a90ddc),
/* harmony export */   "__wbg_requestAnimationFrame_0c71cd3c6779a371": () => (/* binding */ __wbg_requestAnimationFrame_0c71cd3c6779a371),
/* harmony export */   "__wbg_setTimeout_1c75092906446b91": () => (/* binding */ __wbg_setTimeout_1c75092906446b91),
/* harmony export */   "__wbg_length_01a613025b5ffd74": () => (/* binding */ __wbg_length_01a613025b5ffd74),
/* harmony export */   "__wbg_item_b192ab411bbfbb09": () => (/* binding */ __wbg_item_b192ab411bbfbb09),
/* harmony export */   "__wbg_get_a765dab923455e0d": () => (/* binding */ __wbg_get_a765dab923455e0d),
/* harmony export */   "__wbg_length_a2882c668bdf6488": () => (/* binding */ __wbg_length_a2882c668bdf6488),
/* harmony export */   "__wbg_get_1c01a7682a9775bb": () => (/* binding */ __wbg_get_1c01a7682a9775bb),
/* harmony export */   "__wbg_id_79dca31d8297faf1": () => (/* binding */ __wbg_id_79dca31d8297faf1),
/* harmony export */   "__wbg_setid_73be37238eaa05be": () => (/* binding */ __wbg_setid_73be37238eaa05be),
/* harmony export */   "__wbg_scrollLeft_e8aba47a94d12290": () => (/* binding */ __wbg_scrollLeft_e8aba47a94d12290),
/* harmony export */   "__wbg_getBoundingClientRect_534c1b96b6e612d3": () => (/* binding */ __wbg_getBoundingClientRect_534c1b96b6e612d3),
/* harmony export */   "__wbg_getElementsByClassName_8a7d00ed3eaf1522": () => (/* binding */ __wbg_getElementsByClassName_8a7d00ed3eaf1522),
/* harmony export */   "__wbg_length_41b205f6892bf9d9": () => (/* binding */ __wbg_length_41b205f6892bf9d9),
/* harmony export */   "__wbg_get_bdec89fd60d07530": () => (/* binding */ __wbg_get_bdec89fd60d07530),
/* harmony export */   "__wbg_dataTransfer_bc4c0501385a0c8e": () => (/* binding */ __wbg_dataTransfer_bc4c0501385a0c8e),
/* harmony export */   "__wbg_drawElementsInstancedANGLE_e184bb1bad14df88": () => (/* binding */ __wbg_drawElementsInstancedANGLE_e184bb1bad14df88),
/* harmony export */   "__wbg_vertexAttribDivisorANGLE_2dc41a79843a435c": () => (/* binding */ __wbg_vertexAttribDivisorANGLE_2dc41a79843a435c),
/* harmony export */   "__wbg_scrollTop_5ebd5c6591748d6e": () => (/* binding */ __wbg_scrollTop_5ebd5c6591748d6e),
/* harmony export */   "__wbg_hidden_f7a620ec4ab18ce5": () => (/* binding */ __wbg_hidden_f7a620ec4ab18ce5),
/* harmony export */   "__wbg_sethidden_fdaefd7e7da7e4c0": () => (/* binding */ __wbg_sethidden_fdaefd7e7da7e4c0),
/* harmony export */   "__wbg_style_16f5dd9624687c8f": () => (/* binding */ __wbg_style_16f5dd9624687c8f),
/* harmony export */   "__wbg_offsetTop_45111254e7b26a1f": () => (/* binding */ __wbg_offsetTop_45111254e7b26a1f),
/* harmony export */   "__wbg_offsetLeft_be5393bf9eec5766": () => (/* binding */ __wbg_offsetLeft_be5393bf9eec5766),
/* harmony export */   "__wbg_offsetWidth_bc683e2f57ea2d6b": () => (/* binding */ __wbg_offsetWidth_bc683e2f57ea2d6b),
/* harmony export */   "__wbg_setonload_9235de4503eb82c8": () => (/* binding */ __wbg_setonload_9235de4503eb82c8),
/* harmony export */   "__wbg_setonerror_939f617c2b40758c": () => (/* binding */ __wbg_setonerror_939f617c2b40758c),
/* harmony export */   "__wbg_blur_2156876090506146": () => (/* binding */ __wbg_blur_2156876090506146),
/* harmony export */   "__wbg_focus_4434360545ac99cf": () => (/* binding */ __wbg_focus_4434360545ac99cf),
/* harmony export */   "__wbg_responseURL_a3e549a88db1c1f7": () => (/* binding */ __wbg_responseURL_a3e549a88db1c1f7),
/* harmony export */   "__wbg_setresponseType_e5326d926ee8e787": () => (/* binding */ __wbg_setresponseType_e5326d926ee8e787),
/* harmony export */   "__wbg_response_8b12ac238727ae0e": () => (/* binding */ __wbg_response_8b12ac238727ae0e),
/* harmony export */   "__wbg_new_08dfde0f90155eb7": () => (/* binding */ __wbg_new_08dfde0f90155eb7),
/* harmony export */   "__wbg_open_7190f43b39e7f488": () => (/* binding */ __wbg_open_7190f43b39e7f488),
/* harmony export */   "__wbg_send_84c8dd943b775f78": () => (/* binding */ __wbg_send_84c8dd943b775f78),
/* harmony export */   "__wbg_clientX_849ccdf456d662ac": () => (/* binding */ __wbg_clientX_849ccdf456d662ac),
/* harmony export */   "__wbg_clientY_1aaff30fe0cd0876": () => (/* binding */ __wbg_clientY_1aaff30fe0cd0876),
/* harmony export */   "__wbg_ctrlKey_4e536bedb069129f": () => (/* binding */ __wbg_ctrlKey_4e536bedb069129f),
/* harmony export */   "__wbg_metaKey_0b396e35a4941247": () => (/* binding */ __wbg_metaKey_0b396e35a4941247),
/* harmony export */   "__wbg_button_a18f33eb55774d89": () => (/* binding */ __wbg_button_a18f33eb55774d89),
/* harmony export */   "__wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362": () => (/* binding */ __wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362),
/* harmony export */   "__wbg_canvas_d0b58be124e596e3": () => (/* binding */ __wbg_canvas_d0b58be124e596e3),
/* harmony export */   "__wbg_bufferData_85d635f32a990208": () => (/* binding */ __wbg_bufferData_85d635f32a990208),
/* harmony export */   "__wbg_bufferSubData_3a944e1fdad0cd9a": () => (/* binding */ __wbg_bufferSubData_3a944e1fdad0cd9a),
/* harmony export */   "__wbg_readPixels_3692eaca9dfc7c0c": () => (/* binding */ __wbg_readPixels_3692eaca9dfc7c0c),
/* harmony export */   "__wbg_texImage2D_d26bd916ff0956a1": () => (/* binding */ __wbg_texImage2D_d26bd916ff0956a1),
/* harmony export */   "__wbg_texImage2D_b46a9b691e69d90b": () => (/* binding */ __wbg_texImage2D_b46a9b691e69d90b),
/* harmony export */   "__wbg_texSubImage2D_d907a4c940fd6e41": () => (/* binding */ __wbg_texSubImage2D_d907a4c940fd6e41),
/* harmony export */   "__wbg_texSubImage2D_d9dc0ffd91998f0d": () => (/* binding */ __wbg_texSubImage2D_d9dc0ffd91998f0d),
/* harmony export */   "__wbg_uniform1fv_e6a2134edff4f2e9": () => (/* binding */ __wbg_uniform1fv_e6a2134edff4f2e9),
/* harmony export */   "__wbg_uniformMatrix2fv_aaa4e0f7c15bca04": () => (/* binding */ __wbg_uniformMatrix2fv_aaa4e0f7c15bca04),
/* harmony export */   "__wbg_uniformMatrix4fv_f07c6caf5a563616": () => (/* binding */ __wbg_uniformMatrix4fv_f07c6caf5a563616),
/* harmony export */   "__wbg_activeTexture_74ed11a5c5d5af90": () => (/* binding */ __wbg_activeTexture_74ed11a5c5d5af90),
/* harmony export */   "__wbg_attachShader_55dbe770f3ee32ca": () => (/* binding */ __wbg_attachShader_55dbe770f3ee32ca),
/* harmony export */   "__wbg_bindBuffer_29d52e7bc48650c3": () => (/* binding */ __wbg_bindBuffer_29d52e7bc48650c3),
/* harmony export */   "__wbg_bindFramebuffer_bd35ddd23765c7b6": () => (/* binding */ __wbg_bindFramebuffer_bd35ddd23765c7b6),
/* harmony export */   "__wbg_bindTexture_198c816345baca83": () => (/* binding */ __wbg_bindTexture_198c816345baca83),
/* harmony export */   "__wbg_blendEquation_09d56f3be6f914f5": () => (/* binding */ __wbg_blendEquation_09d56f3be6f914f5),
/* harmony export */   "__wbg_blendFunc_c8f1e0fb4467f57c": () => (/* binding */ __wbg_blendFunc_c8f1e0fb4467f57c),
/* harmony export */   "__wbg_blendFuncSeparate_494b1dae028cb9a9": () => (/* binding */ __wbg_blendFuncSeparate_494b1dae028cb9a9),
/* harmony export */   "__wbg_clear_2af1271959ec83d7": () => (/* binding */ __wbg_clear_2af1271959ec83d7),
/* harmony export */   "__wbg_clearColor_51c4f69c743c3252": () => (/* binding */ __wbg_clearColor_51c4f69c743c3252),
/* harmony export */   "__wbg_compileShader_3b5f9ef4c67a0777": () => (/* binding */ __wbg_compileShader_3b5f9ef4c67a0777),
/* harmony export */   "__wbg_createBuffer_c40f37e1348bb91f": () => (/* binding */ __wbg_createBuffer_c40f37e1348bb91f),
/* harmony export */   "__wbg_createFramebuffer_410b12a5cc5a8f13": () => (/* binding */ __wbg_createFramebuffer_410b12a5cc5a8f13),
/* harmony export */   "__wbg_createProgram_245520da1fb9e47b": () => (/* binding */ __wbg_createProgram_245520da1fb9e47b),
/* harmony export */   "__wbg_createShader_4d8818a13cb825b3": () => (/* binding */ __wbg_createShader_4d8818a13cb825b3),
/* harmony export */   "__wbg_createTexture_f3a6a715d6bada45": () => (/* binding */ __wbg_createTexture_f3a6a715d6bada45),
/* harmony export */   "__wbg_cullFace_c6fb8a7309c36a38": () => (/* binding */ __wbg_cullFace_c6fb8a7309c36a38),
/* harmony export */   "__wbg_deleteBuffer_c708688b9e1b3518": () => (/* binding */ __wbg_deleteBuffer_c708688b9e1b3518),
/* harmony export */   "__wbg_deleteFramebuffer_ca006f8649d4550a": () => (/* binding */ __wbg_deleteFramebuffer_ca006f8649d4550a),
/* harmony export */   "__wbg_deleteTexture_9159fb5927ed32c0": () => (/* binding */ __wbg_deleteTexture_9159fb5927ed32c0),
/* harmony export */   "__wbg_disable_2b63b75dc6c27537": () => (/* binding */ __wbg_disable_2b63b75dc6c27537),
/* harmony export */   "__wbg_disableVertexAttribArray_aa8458b40dd08914": () => (/* binding */ __wbg_disableVertexAttribArray_aa8458b40dd08914),
/* harmony export */   "__wbg_drawArrays_22c88d644a33fd59": () => (/* binding */ __wbg_drawArrays_22c88d644a33fd59),
/* harmony export */   "__wbg_drawElements_6e26500a25ecf478": () => (/* binding */ __wbg_drawElements_6e26500a25ecf478),
/* harmony export */   "__wbg_enable_8f6dd779ccb8e1de": () => (/* binding */ __wbg_enable_8f6dd779ccb8e1de),
/* harmony export */   "__wbg_enableVertexAttribArray_4ed5f91d0718bee1": () => (/* binding */ __wbg_enableVertexAttribArray_4ed5f91d0718bee1),
/* harmony export */   "__wbg_framebufferTexture2D_31643260e5b0b294": () => (/* binding */ __wbg_framebufferTexture2D_31643260e5b0b294),
/* harmony export */   "__wbg_getActiveUniform_3851244f8fc5db53": () => (/* binding */ __wbg_getActiveUniform_3851244f8fc5db53),
/* harmony export */   "__wbg_getAttribLocation_da5df7094096113d": () => (/* binding */ __wbg_getAttribLocation_da5df7094096113d),
/* harmony export */   "__wbg_getExtension_c6ceee3244ee7f20": () => (/* binding */ __wbg_getExtension_c6ceee3244ee7f20),
/* harmony export */   "__wbg_getProgramInfoLog_c253042b64e86027": () => (/* binding */ __wbg_getProgramInfoLog_c253042b64e86027),
/* harmony export */   "__wbg_getProgramParameter_4f698af0dda0a2d4": () => (/* binding */ __wbg_getProgramParameter_4f698af0dda0a2d4),
/* harmony export */   "__wbg_getShaderInfoLog_584794e3bcf1e19b": () => (/* binding */ __wbg_getShaderInfoLog_584794e3bcf1e19b),
/* harmony export */   "__wbg_getShaderParameter_64b1ffe576e5fa25": () => (/* binding */ __wbg_getShaderParameter_64b1ffe576e5fa25),
/* harmony export */   "__wbg_getUniformLocation_703972f150a46500": () => (/* binding */ __wbg_getUniformLocation_703972f150a46500),
/* harmony export */   "__wbg_linkProgram_5fdd57237c761833": () => (/* binding */ __wbg_linkProgram_5fdd57237c761833),
/* harmony export */   "__wbg_scissor_fb094c7db856e2a7": () => (/* binding */ __wbg_scissor_fb094c7db856e2a7),
/* harmony export */   "__wbg_shaderSource_173ab97288934a60": () => (/* binding */ __wbg_shaderSource_173ab97288934a60),
/* harmony export */   "__wbg_texParameteri_caec5468f2a850c3": () => (/* binding */ __wbg_texParameteri_caec5468f2a850c3),
/* harmony export */   "__wbg_uniform1f_258478814234cf9c": () => (/* binding */ __wbg_uniform1f_258478814234cf9c),
/* harmony export */   "__wbg_uniform1i_a0275676828a22b6": () => (/* binding */ __wbg_uniform1i_a0275676828a22b6),
/* harmony export */   "__wbg_uniform2f_b0af46ba675f2c0d": () => (/* binding */ __wbg_uniform2f_b0af46ba675f2c0d),
/* harmony export */   "__wbg_uniform3f_65416973a351fbab": () => (/* binding */ __wbg_uniform3f_65416973a351fbab),
/* harmony export */   "__wbg_uniform4f_e5d0a91bf98b35ad": () => (/* binding */ __wbg_uniform4f_e5d0a91bf98b35ad),
/* harmony export */   "__wbg_useProgram_d5898a40ebe88916": () => (/* binding */ __wbg_useProgram_d5898a40ebe88916),
/* harmony export */   "__wbg_vertexAttribPointer_0d097efa33e3f45f": () => (/* binding */ __wbg_vertexAttribPointer_0d097efa33e3f45f),
/* harmony export */   "__wbg_viewport_19577064127daf83": () => (/* binding */ __wbg_viewport_19577064127daf83),
/* harmony export */   "__wbindgen_is_undefined": () => (/* binding */ __wbindgen_is_undefined),
/* harmony export */   "__wbg_buffer_5e74a88a1424a2e0": () => (/* binding */ __wbg_buffer_5e74a88a1424a2e0),
/* harmony export */   "__wbg_get_f45dff51f52d7222": () => (/* binding */ __wbg_get_f45dff51f52d7222),
/* harmony export */   "__wbg_length_7b60f47bde714631": () => (/* binding */ __wbg_length_7b60f47bde714631),
/* harmony export */   "__wbg_newnoargs_f579424187aa1717": () => (/* binding */ __wbg_newnoargs_f579424187aa1717),
/* harmony export */   "__wbg_call_89558c3e96703ca1": () => (/* binding */ __wbg_call_89558c3e96703ca1),
/* harmony export */   "__wbg_isArray_8480ed76e5369634": () => (/* binding */ __wbg_isArray_8480ed76e5369634),
/* harmony export */   "__wbg_resolve_4f8f547f26b30b27": () => (/* binding */ __wbg_resolve_4f8f547f26b30b27),
/* harmony export */   "__wbg_then_a6860c82b90816ca": () => (/* binding */ __wbg_then_a6860c82b90816ca),
/* harmony export */   "__wbg_then_58a04e42527f52c6": () => (/* binding */ __wbg_then_58a04e42527f52c6),
/* harmony export */   "__wbg_self_e23d74ae45fb17d1": () => (/* binding */ __wbg_self_e23d74ae45fb17d1),
/* harmony export */   "__wbg_window_b4be7f48b24ac56e": () => (/* binding */ __wbg_window_b4be7f48b24ac56e),
/* harmony export */   "__wbg_globalThis_d61b1f48a57191ae": () => (/* binding */ __wbg_globalThis_d61b1f48a57191ae),
/* harmony export */   "__wbg_global_e7669da72fd7f239": () => (/* binding */ __wbg_global_e7669da72fd7f239),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_278ec7532799393a": () => (/* binding */ __wbg_newwithbyteoffsetandlength_278ec7532799393a),
/* harmony export */   "__wbg_new_e3b800e570795b3c": () => (/* binding */ __wbg_new_e3b800e570795b3c),
/* harmony export */   "__wbg_set_5b8081e9d002f0df": () => (/* binding */ __wbg_set_5b8081e9d002f0df),
/* harmony export */   "__wbg_length_30803400a8f15c59": () => (/* binding */ __wbg_length_30803400a8f15c59),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_bdb885cfc5e9bc43": () => (/* binding */ __wbg_newwithbyteoffsetandlength_bdb885cfc5e9bc43),
/* harmony export */   "__wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f": () => (/* binding */ __wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f),
/* harmony export */   "__wbg_new_f5438c0cea22a3aa": () => (/* binding */ __wbg_new_f5438c0cea22a3aa),
/* harmony export */   "__wbg_set_7cb6639737aebb39": () => (/* binding */ __wbg_set_7cb6639737aebb39),
/* harmony export */   "__wbg_length_44449d3b5928d07c": () => (/* binding */ __wbg_length_44449d3b5928d07c),
/* harmony export */   "__wbg_newwithlength_5f4ce114a24dfe1e": () => (/* binding */ __wbg_newwithlength_5f4ce114a24dfe1e),
/* harmony export */   "__wbg_newwithlength_747b31c525d823ec": () => (/* binding */ __wbg_newwithlength_747b31c525d823ec),
/* harmony export */   "__wbg_subarray_e729e242fb317565": () => (/* binding */ __wbg_subarray_e729e242fb317565),
/* harmony export */   "__wbg_parse_e3e7e590474b89d2": () => (/* binding */ __wbg_parse_e3e7e590474b89d2),
/* harmony export */   "__wbg_new_59cb74e423758ede": () => (/* binding */ __wbg_new_59cb74e423758ede),
/* harmony export */   "__wbg_stack_558ba5917b466edd": () => (/* binding */ __wbg_stack_558ba5917b466edd),
/* harmony export */   "__wbg_error_4bb6c2a97407129a": () => (/* binding */ __wbg_error_4bb6c2a97407129a),
/* harmony export */   "__wbindgen_debug_string": () => (/* binding */ __wbindgen_debug_string),
/* harmony export */   "__wbindgen_throw": () => (/* binding */ __wbindgen_throw),
/* harmony export */   "__wbindgen_memory": () => (/* binding */ __wbindgen_memory),
/* harmony export */   "__wbindgen_closure_wrapper1458": () => (/* binding */ __wbindgen_closure_wrapper1458),
/* harmony export */   "__wbindgen_closure_wrapper1460": () => (/* binding */ __wbindgen_closure_wrapper1460),
/* harmony export */   "__wbindgen_closure_wrapper2297": () => (/* binding */ __wbindgen_closure_wrapper2297),
/* harmony export */   "__wbindgen_closure_wrapper2298": () => (/* binding */ __wbindgen_closure_wrapper2298),
/* harmony export */   "__wbindgen_closure_wrapper2300": () => (/* binding */ __wbindgen_closure_wrapper2300),
/* harmony export */   "__wbindgen_closure_wrapper2302": () => (/* binding */ __wbindgen_closure_wrapper2302),
/* harmony export */   "__wbindgen_closure_wrapper2304": () => (/* binding */ __wbindgen_closure_wrapper2304),
/* harmony export */   "__wbindgen_closure_wrapper2306": () => (/* binding */ __wbindgen_closure_wrapper2306),
/* harmony export */   "__wbindgen_closure_wrapper2308": () => (/* binding */ __wbindgen_closure_wrapper2308),
/* harmony export */   "__wbindgen_closure_wrapper2310": () => (/* binding */ __wbindgen_closure_wrapper2310),
/* harmony export */   "__wbindgen_closure_wrapper2313": () => (/* binding */ __wbindgen_closure_wrapper2313),
/* harmony export */   "__wbindgen_closure_wrapper2456": () => (/* binding */ __wbindgen_closure_wrapper2456)
/* harmony export */ });
/* harmony import */ var _snippets_al_ui_d4d455bd9e91f451_js_hpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./snippets/al-ui-d4d455bd9e91f451/js/hpxImageSurvey.js */ "./src/core/pkg-webgl1/snippets/al-ui-d4d455bd9e91f451/js/hpxImageSurvey.js");
/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ "./src/core/pkg-webgl1/index_bg.wasm");
/* module decorator */ module = __webpack_require__.hmd(module);
/* provided dependency */ var TextDecoder = __webpack_require__(/*! text-encoding */ "./node_modules/text-encoding/index.js")["TextDecoder"];
/* provided dependency */ var TextEncoder = __webpack_require__(/*! text-encoding */ "./node_modules/text-encoding/index.js")["TextEncoder"];



const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let WASM_VECTOR_LEN = 0;

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
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
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
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_26(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2bda815aee23e04f(arg0, arg1, addBorrowedObject(arg2));
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
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_29(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2bc122081057ca5c(arg0, arg1);
}

function __wbg_adapter_32(arg0, arg1) {
    try {
        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _assertNum(arg0);
        _assertNum(arg1);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc7b86a4a9972026b(retptr, arg0, arg1);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
    }
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h036bd29ba9039c35(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_38(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4a1efbed06081ce8(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_41(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h97b4cca87a4f442d(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_44(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9948bd0462ccf34b(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_47(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h2c4060d5b89984c6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_50(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcd20bd4572ae6841(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_53(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7069be39e0f7ccf8(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_56(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h00c05509b8d9a6e1(arg0, arg1);
}

function __wbg_adapter_59(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4266fc9359039d5a(arg0, arg1, addHeapObject(arg2));
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetUint32Memory0 = new Uint32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
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
    try {
        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.GALCooSys(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
            throw takeObject(r1);
        }
        return r0 >>> 0;
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @returns {number}
*/
function ICRSJ2000CooSys() {
    try {
        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.ICRSJ2000CooSys(retptr);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
            throw takeObject(r1);
        }
        return r0 >>> 0;
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachegetFloat32Memory0 = null;
function getFloat32Memory0() {
    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetFloat32Memory0 = new Float32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetFloat32Memory0;
}

function getArrayF32FromWasm0(ptr, len) {
    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);
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
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_webclient_free(ptr);
    }
    /**
    * Create the Aladin Lite webgl backend
    *
    * # Arguments
    *
    * * `aladin_div_name` - The name of the div where aladin is created
    * * `shaders` - The list of shader objects containing the GLSL code source
    * * `resources` - Image resource files
    * @param {string} aladin_div_name
    * @param {any} shaders
    * @param {any} resources
    */
    constructor(aladin_div_name, shaders, resources) {
        try {
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            var ptr0 = passStringToWasm0(aladin_div_name, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_new(retptr, ptr0, len0, addBorrowedObject(shaders), addBorrowedObject(resources));
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WebClient.__wrap(r0);
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
            heap[stack_pointer++] = undefined;
            heap[stack_pointer++] = undefined;
        }
    }
    /**
    * Update the view
    *
    * # Arguments
    *
    * * `dt` - The time elapsed from the last frame update
    * * `force` - This parameter ensures to force the update of some elements
    *   even if the camera has not moved
    * @param {number} dt
    * @param {boolean} force
    */
    update(dt, force) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _assertBoolean(force);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_update(retptr, this.ptr, dt, force);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Resize the window
    *
    * # Arguments
    *
    * * `width` - The width in pixels of the view
    * * `height` - The height in pixels of the view
    * @param {number} width
    * @param {number} height
    */
    resize(width, height) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_resize(retptr, this.ptr, width, height);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Render the frame to the canvas
    *
    * The rendering does not redraw the scene if nothing has changed
    *
    * # Arguments
    *
    * * `force` - Force the rendering of the frame
    * @param {boolean} force
    */
    render(force) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _assertBoolean(force);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_render(retptr, this.ptr, force);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the type of projections
    *
    * # Arguments
    *
    * * `name` - Can be aitoff, mollweide, arc, sinus, tan or mercator
    * @param {string} projection
    * @returns {WebClient}
    */
    setProjection(projection) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const ptr = this.__destroy_into_raw();
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(ptr);
            var ptr0 = passStringToWasm0(projection, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setProjection(retptr, ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return WebClient.__wrap(r0);
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Reverse the longitude axis
    *
    * # Arguments
    *
    * * `reversed` - set it to `false` for planetary surveys, `true` for spatial ones
    * @param {boolean} reversed
    */
    setLongitudeReversed(reversed) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _assertBoolean(reversed);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setLongitudeReversed(retptr, this.ptr, reversed);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Check whether the app is ready
    *
    * Aladin Lite is in a good state when the root tiles of the
    * HiPS chosen have all been retrieved and accessible for the GPU
    *
    * Surveys can be changed only if Aladin Lite is ready
    * @returns {boolean}
    */
    isReady() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_isReady(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set new image surveys
    *
    * Send the image surveys to render inside the Aladin Lite view
    *
    * # Arguments
    *
    * * `surveys` - A list/array of survey. A survey is a javascript object
    * having the specific form. Please check the file in core/src/hips.rs to see
    * the different semantics accepted.
    *
    * # Examples
    *
    * ```javascript
    * let al = new Aladin.wasmLibs.webgl.WebClient(...);
    * const panstarrs = {
    *     properties: {
    *         url: "http://alasky.u-strasbg.fr/Pan-STARRS/DR1/r",
    *
    *         maxOrder: 11,
    *         frame: { label: "J2000", system: "J2000" },
    *         tileSize: 512,
    *         format: {
    *             FITSImage: {
    *                 bitpix: 16,
    *             }
    *         },
    *         minCutout: -0.15,
    *         maxCutout: 5,
    *     },
    *     color: {
    *         Grayscale2Colormap: {
    *             colormap: "RedTemperature",
    *             transfer: "asinh",
    *             reversed: false,
    *         }
    *     },
    * };
    * al.setImageSurveys([panstarrs]);
    * ```
    *
    * # Panics
    *
    * * If the surveys do not match SimpleHiPS type
    * * If the number of surveys is greater than 4. For the moment, due to the limitations
    *   of WebGL2 texture units on some architectures, the total number of surveys rendered is
    *   limited to 4.
    * @param {any[]} surveys
    */
    setImageSurveys(surveys) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passArrayJsValueToWasm0(surveys, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setImageSurveys(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Move a layer forward the other ones
    *
    * # Arguments
    *
    * * `layer_name` - The name of the layer to move
    *
    * # Panics
    *
    * * If the layer specified is not found
    * @param {string} survey_url
    */
    moveImageSurveysLayerForward(survey_url) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(survey_url, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_moveImageSurveysLayerForward(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the opacity of a layer
    *
    * # Arguments
    *
    * * `opacity` - Set an opacity value (between 0.0 and 1.0)
    * * `layer_name` - The name of the layer to move
    *
    * # Panics
    *
    * * If the layer specified is not found
    * @param {number} opacity
    * @param {string} survey_url
    */
    setOpacityLayer(opacity, survey_url) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(survey_url, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setOpacityLayer(retptr, this.ptr, opacity, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the equatorial grid color
    *
    * # Arguments
    *
    * * `red` - Red amount (between 0.0 and 1.0)
    * * `green` - Green amount (between 0.0 and 1.0)
    * * `blue` - Blue amount (between 0.0 and 1.0)
    * * `alpha` - Alpha amount (between 0.0 and 1.0)
    * @param {number} red
    * @param {number} green
    * @param {number} blue
    * @param {number} alpha
    */
    setGridColor(red, green, blue, alpha) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setGridColor(retptr, this.ptr, red, green, blue, alpha);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Enable the rendering of the equatorial grid
    */
    enableGrid() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_enableGrid(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Disable the rendering of the equatorial grid
    */
    disableGrid() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_disableGrid(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Enable the rendering of the equatorial grid labels
    */
    hideGridLabels() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_hideGridLabels(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Disable the rendering of the equatorial grid labels
    */
    showGridLabels() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_showGridLabels(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the coordinate system of the view
    *
    * Returns either ICRSJ2000 or GAL
    * @returns {number}
    */
    cooSystem() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_cooSystem(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 >>> 0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the coordinate system for the view
    *
    * # Arguments
    *
    * * `coo_system` - The coordinate system
    * @param {number} coo_system
    */
    setCooSystem(coo_system) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _assertNum(coo_system);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setCooSystem(retptr, this.ptr, coo_system);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Convert a j2000 lonlat to a galactic one
    *
    * # Arguments
    *
    * * `lon` - A longitude in degrees
    * * `lat` - A latitude in degrees
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    J20002Gal(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_J20002Gal(retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Convert a galactic lonlat to a j2000 one
    *
    * # Arguments
    *
    * * `lon` - A longitude in degrees
    * * `lat` - A latitude in degrees
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    Gal2J2000(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_Gal2J2000(retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the field of the view in degrees
    * @returns {number}
    */
    getFieldOfView() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getFieldOfView(retptr, this.ptr);
            var r0 = getFloat64Memory0()[retptr / 8 + 0];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            return r0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the field of view
    *
    * # Arguments
    *
    * * `fov` - The field of view in degrees
    * @param {number} fov
    */
    setFieldOfView(fov) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setFieldOfView(retptr, this.ptr, fov);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the absolute orientation of the view
    *
    * # Arguments
    *
    * * `theta` - The rotation angle in degrees
    * @param {number} theta
    */
    setRotationAroundCenter(theta) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setRotationAroundCenter(retptr, this.ptr, theta);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the absolute orientation angle of the view
    * @returns {number}
    */
    getRotationAroundCenter() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getRotationAroundCenter(retptr, this.ptr);
            var r0 = getFloat64Memory0()[retptr / 8 + 0];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            return r0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the field of view angle value when the view is zoomed out to its maximum
    *
    * This method is dependent of the projection currently set.
    * All sky projections should return 360 degrees whereas
    * the sinus would be 180 degrees.
    * @returns {number}
    */
    getMaxFieldOfView() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getMaxFieldOfView(this.ptr);
        return ret;
    }
    /**
    * Get the clip zoom factor of the view
    *
    * This factor is deduced from the field of view angle.
    * It is a constant which when multiplied to the screen coordinates
    * gives the coordinates in clipping space.
    * @returns {number}
    */
    getClipZoomFactor() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getClipZoomFactor(retptr, this.ptr);
            var r0 = getFloat64Memory0()[retptr / 8 + 0];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            return r0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the center of the view
    *
    * The core works in ICRS system so
    * the location must be given in this system
    *
    * # Arguments
    *
    * * `lon` - A longitude in degrees
    * * `lat` - A latitude in degrees
    * @param {number} lon
    * @param {number} lat
    */
    setCenter(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setCenter(retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the center of the view
    *
    * This returns a javascript array of size 2.
    * The first component is the longitude, the second one is the latitude.
    * The angles are given in degrees.
    * @returns {Float64Array}
    */
    getCenter() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getCenter(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Rest the north pole orientation to the top of the screen
    */
    resetNorthOrientation() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_resetNorthOrientation(this.ptr);
    }
    /**
    * Move to a location
    *
    * The core works in ICRS system so
    * the location must be given in this system
    *
    * # Arguments
    *
    * * `lon` - A longitude in degrees
    * * `lat` - A latitude in degrees
    * @param {number} lon
    * @param {number} lat
    */
    moveToLocation(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_moveToLocation(retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Go from a location to another one
    *
    * # Arguments
    *
    * * `s1x` - The x screen coordinate in pixels of the starting point
    * * `s1y` - The y screen coordinate in pixels of the starting point
    * * `s2x` - The x screen coordinate in pixels of the goal point
    * * `s2y` - The y screen coordinate in pixels of the goal point
    * @param {number} s1x
    * @param {number} s1y
    * @param {number} s2x
    * @param {number} s2y
    */
    goFromTo(s1x, s1y, s2x, s2y) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_goFromTo(retptr, this.ptr, s1x, s1y, s2x, s2y);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * World to screen projection
    *
    * Coordinates must be given in the ICRS coo system
    *
    * # Arguments
    *
    * * `lon` - A longitude in degrees
    * * `lat` - A latitude in degrees
    * @param {number} lon
    * @param {number} lat
    * @returns {Float64Array | undefined}
    */
    worldToScreen(lon, lat) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_worldToScreen(retptr, this.ptr, lon, lat);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * World to screen projection of a list of sources
    *
    * Coordinates must be given in the ICRS coo system
    *
    * # Arguments
    *
    * * `sources` - An array of sources
    * @param {any[]} sources
    * @returns {Float64Array}
    */
    worldToScreenVec(sources) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passArrayJsValueToWasm0(sources, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_worldToScreenVec(retptr, this.ptr, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            return v1;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Screen to world unprojection
    *
    * # Arguments
    *
    * * `pos_x` - The x screen coordinate in pixels
    * * `pos_y` - The y screen coordinate in pixels
    * @param {number} pos_x
    * @param {number} pos_y
    * @returns {Float64Array | undefined}
    */
    screenToWorld(pos_x, pos_y) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_screenToWorld(retptr, this.ptr, pos_x, pos_y);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            let v0;
            if (r0 !== 0) {
                v0 = getArrayF64FromWasm0(r0, r1).slice();
                _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            }
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Signal the backend when the left mouse button has been released.
    *
    * This is useful for beginning inerting.
    * @param {number} sx
    * @param {number} sy
    */
    releaseLeftButtonMouse(sx, sy) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_releaseLeftButtonMouse(retptr, this.ptr, sx, sy);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Signal the backend when the left mouse button has been pressed.
    * @param {number} sx
    * @param {number} sy
    */
    pressLeftMouseButton(sx, sy) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_pressLeftMouseButton(retptr, this.ptr, sx, sy);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Signal the backend when a wheel event has been registered
    *
    * The field of view is changed accordingly
    *
    * # Arguments
    *
    * * `delta` - The delta coming from the wheel event. This is
    *   used to know if we are zooming or not.
    * @param {number} delta
    */
    registerWheelEvent(delta) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_registerWheelEvent(retptr, this.ptr, delta);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Signal the backend when a wheel event has been registered
    *
    * The field of view is changed accordingly
    *
    * # Arguments
    *
    * * `delta` - The delta coming from the wheel event. This is
    *   used to know if we are zooming or not.
    * @returns {boolean}
    */
    posOnUi() {
        if (this.ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.ptr);
        var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_posOnUi(this.ptr);
        return ret !== 0;
    }
    /**
    * Add a catalog rendered as a heatmap.
    *
    * # Arguments
    *
    * * `name_catalog` - The name of the catalog
    * * `data` - The list of the catalog sources.
    * * `colormap` - The name of the colormap. Check out the list of possible colormaps names `getAvailableColormapList`.
    * @param {string} name_catalog
    * @param {any} data
    * @param {string} colormap
    */
    addCatalog(name_catalog, data, colormap) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(colormap, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_addCatalog(retptr, this.ptr, ptr0, len0, addHeapObject(data), ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the catalog heatmap colormap
    *
    * # Arguments
    *
    * * `name_catalog` - The name of the catalog to apply this change to
    * * `colormap` - The name of the colormap. Check out the list of possible colormaps names `getAvailableColormapList`.
    *
    * # Panics
    *
    * If the catalog has not been found
    * @returns {boolean}
    */
    isCatalogLoaded() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_isCatalogLoaded(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return r0 !== 0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the catalog heatmap colormap
    *
    * # Arguments
    *
    * * `name_catalog` - The name of the catalog to apply this change to
    * * `colormap` - The name of the colormap. Check out the list of possible colormaps names `getAvailableColormapList`.
    *
    * # Panics
    *
    * If the catalog has not been found
    * @param {string} name_catalog
    * @param {string} colormap
    */
    setCatalogColormap(name_catalog, colormap) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            var ptr1 = passStringToWasm0(colormap, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setCatalogColormap(retptr, this.ptr, ptr0, len0, ptr1, len1);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the catalog heatmap opacity
    *
    * # Arguments
    *
    * * `name_catalog` - The name of the catalog to apply this change to
    * * `opacity` - The opacity factor (between 0.0 and 1.0)
    *
    * # Panics
    *
    * If the catalog has not been found
    * @param {string} name_catalog
    * @param {number} opacity
    */
    setCatalogOpacity(name_catalog, opacity) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setCatalogOpacity(retptr, this.ptr, ptr0, len0, opacity);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Set the kernel strength for the catalog heatmap rendering
    *
    * # Arguments
    *
    * * `name_catalog` - The name of the catalog to apply this change to
    * * `strength` - The strength of the kernel
    *
    * # Panics
    *
    * If the catalog has not been found
    * @param {string} name_catalog
    * @param {number} strength
    */
    setCatalogKernelStrength(name_catalog, strength) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(name_catalog, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_setCatalogKernelStrength(retptr, this.ptr, ptr0, len0, strength);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            if (r1) {
                throw takeObject(r0);
            }
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Project a line to the screen
    *
    * # Returns
    *
    * A list of xy screen coordinates defining the projected line.
    * The algorithm involved is recursive and can return different number of
    * control points depending on the projection used and therefore
    * the deformation of the line.
    *
    * # Arguments
    *
    * * `lon1` - The longitude in degrees of the starting line point
    * * `lat1` - The latitude in degrees of the starting line point
    * * `lon2` - The longitude in degrees of the ending line point
    * * `lat2` - The latitude in degrees of the ending line point
    * @param {number} lon1
    * @param {number} lat1
    * @param {number} lon2
    * @param {number} lat2
    * @returns {Float64Array}
    */
    projectLine(lon1, lat1, lon2, lat2) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_projectLine(retptr, this.ptr, lon1, lat1, lon2, lat2);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            var r3 = getInt32Memory0()[retptr / 4 + 3];
            if (r3) {
                throw takeObject(r2);
            }
            var v0 = getArrayF64FromWasm0(r0, r1).slice();
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(r0, r1 * 8);
            return v0;
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the list of colormap supported
    *
    * This list must be updated whenever a new colormap is added
    * in core/img/colormaps/colormaps.png
    * @returns {any}
    */
    getAvailableColormapList() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_getAvailableColormapList(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Get the image canvas where the webgl rendering is done
    * @returns {object | undefined}
    */
    canvas() {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_canvas(retptr, this.ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * Read the pixel value
    *
    * The current implementation only returns the pixel value
    * of the first survey of the `layer` specified.
    *
    * # Returns
    *
    * - An array of 3 items (rgb) for JPG tiles
    * - An array of 4 items (rgba) for PNG tiles
    * - A single value for FITS tiles
    *
    * # Arguments
    *
    * * `x` - The x screen coordinate in pixels
    * * `y` - The y screen coordinate in pixels
    * * `base_url` - The base url of the survey identifying it
    * @param {number} x
    * @param {number} y
    * @param {string} base_url
    * @returns {any}
    */
    readPixel(x, y, base_url) {
        try {
            if (this.ptr == 0) throw new Error('Attempt to use a moved value');
            const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
            _assertNum(this.ptr);
            var ptr0 = passStringToWasm0(base_url, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.webclient_readPixel(retptr, this.ptr, x, y, ptr0, len0);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var r2 = getInt32Memory0()[retptr / 4 + 2];
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        }
    }
}

function __wbindgen_string_new(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

function __wbindgen_object_clone_ref(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

function __wbindgen_cb_drop(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    _assertBoolean(ret);
    return ret;
};

function __wbindgen_number_get(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'number' ? obj : undefined;
    if (!isLikeNone(ret)) {
        _assertNum(ret);
    }
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

function __wbindgen_json_parse(arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

function __wbindgen_json_serialize(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = JSON.stringify(obj === undefined ? null : obj);
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbg_fetchSurveyMetadata_3d518f6be78ba7d4() { return handleError(function (arg0, arg1) {
    try {
        var ret = (0,_snippets_al_ui_d4d455bd9e91f451_js_hpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_1__.fetchSurveyMetadata)(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);
    }
}, arguments) };

function __wbindgen_boolean_get(arg0) {
    const v = getObject(arg0);
    var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    _assertNum(ret);
    return ret;
};

function __wbg_log_a39f164b49616cb0() { return logError(function (arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
}, arguments) };

function __wbg_instanceof_HtmlCanvasElement_a6157e470d06b638() { return logError(function (arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_width_cfa982e2a6ad6297() { return logError(function (arg0) {
    var ret = getObject(arg0).width;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_setwidth_362e8db8cbadbe96() { return logError(function (arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
}, arguments) };

function __wbg_height_1b399500ca683487() { return logError(function (arg0) {
    var ret = getObject(arg0).height;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_setheight_28f53831182cc410() { return logError(function (arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
}, arguments) };

function __wbg_getContext_10d5c2a4cc0737c8() { return handleError(function (arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2), getObject(arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_touches_7397ce4df4dceded() { return logError(function (arg0) {
    var ret = getObject(arg0).touches;
    return addHeapObject(ret);
}, arguments) };

function __wbg_changedTouches_363278e8a9a95419() { return logError(function (arg0) {
    var ret = getObject(arg0).changedTouches;
    return addHeapObject(ret);
}, arguments) };

function __wbg_type_e32f387f5584c765() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_preventDefault_fa00541ff125b78c() { return logError(function (arg0) {
    getObject(arg0).preventDefault();
}, arguments) };

function __wbg_stopPropagation_da586180676fa914() { return logError(function (arg0) {
    getObject(arg0).stopPropagation();
}, arguments) };

function __wbg_keyCode_8a05b1390fced3c8() { return logError(function (arg0) {
    var ret = getObject(arg0).keyCode;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_altKey_773e7f8151c49bb1() { return logError(function (arg0) {
    var ret = getObject(arg0).altKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_ctrlKey_8c7ff99be598479e() { return logError(function (arg0) {
    var ret = getObject(arg0).ctrlKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_shiftKey_894b631364d8db13() { return logError(function (arg0) {
    var ret = getObject(arg0).shiftKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_metaKey_99a7d3732e1b7856() { return logError(function (arg0) {
    var ret = getObject(arg0).metaKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_isComposing_b892666abf384da9() { return logError(function (arg0) {
    var ret = getObject(arg0).isComposing;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_key_7f10b1291a923361() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_setsrc_b0a1ac4dd261ae2d() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).src = getStringFromWasm0(arg1, arg2);
}, arguments) };

function __wbg_setcrossOrigin_07e0e4935571a4c5() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).crossOrigin = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
}, arguments) };

function __wbg_width_6c4cad65073b3852() { return logError(function (arg0) {
    var ret = getObject(arg0).width;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_height_133772b066cfc559() { return logError(function (arg0) {
    var ret = getObject(arg0).height;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_new_da67f111e299956e() { return handleError(function () {
    var ret = new Image();
    return addHeapObject(ret);
}, arguments) };

function __wbg_type_8bc3e57acd2158c9() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_userAgent_3f63af8b4fe2331c() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg1).userAgent;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_name_9a61dbbdbfb2d0de() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).name;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_lastModified_0de23a8c5214f2fb() { return logError(function (arg0) {
    var ret = getObject(arg0).lastModified;
    return ret;
}, arguments) };

function __wbg_name_4ada8b70ffadb5c0() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).name;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_error_ca520cb687b085a1() { return logError(function (arg0) {
    console.error(getObject(arg0));
}, arguments) };

function __wbg_log_fbd13631356d44e4() { return logError(function (arg0) {
    console.log(getObject(arg0));
}, arguments) };

function __wbg_warn_97f10a6b0dbb8c5c() { return logError(function (arg0) {
    console.warn(getObject(arg0));
}, arguments) };

function __wbg_deltaX_df228181f4d1a561() { return logError(function (arg0) {
    var ret = getObject(arg0).deltaX;
    return ret;
}, arguments) };

function __wbg_deltaY_afa6edde136e1500() { return logError(function (arg0) {
    var ret = getObject(arg0).deltaY;
    return ret;
}, arguments) };

function __wbg_deltaMode_ed9d7974a0c11323() { return logError(function (arg0) {
    var ret = getObject(arg0).deltaMode;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_setonerror_d665b35adb3552fb() { return logError(function (arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
}, arguments) };

function __wbg_setonload_18033df8ec5db791() { return logError(function (arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
}, arguments) };

function __wbg_addEventListener_6bdba88519fdc1c9() { return handleError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
}, arguments) };

function __wbg_size_20c167ba9040b895() { return logError(function (arg0) {
    var ret = getObject(arg0).size;
    return ret;
}, arguments) };

function __wbg_arrayBuffer_8b5364ee9b393098() { return logError(function (arg0) {
    var ret = getObject(arg0).arrayBuffer();
    return addHeapObject(ret);
}, arguments) };

function __wbg_items_d571f433ef73ee49() { return logError(function (arg0) {
    var ret = getObject(arg0).items;
    return addHeapObject(ret);
}, arguments) };

function __wbg_files_a4192b4f5967317b() { return logError(function (arg0) {
    var ret = getObject(arg0).files;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_body_7538539844356c1c() { return logError(function (arg0) {
    var ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_createElement_d017b8d2af99bab9() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

function __wbg_getElementById_b30e88aff96f66a1() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_data_9562112603a9aa89() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_instanceof_HtmlInputElement_8969541a2a0bded0() { return logError(function (arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_setautofocus_a2ae37091dfbe4af() { return logError(function (arg0, arg1) {
    getObject(arg0).autofocus = arg1 !== 0;
}, arguments) };

function __wbg_setsize_90d1034a7a757a50() { return logError(function (arg0, arg1) {
    getObject(arg0).size = arg1 >>> 0;
}, arguments) };

function __wbg_value_fc1c354d1a0e9714() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_setvalue_ce4a23f487065c07() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).value = getStringFromWasm0(arg1, arg2);
}, arguments) };

function __wbg_now_5fa0ca001e042f8a() { return logError(function (arg0) {
    var ret = getObject(arg0).now();
    return ret;
}, arguments) };

function __wbg_appendChild_3fe5090c665d3bb4() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

function __wbg_top_a24b8b81afea659b() { return logError(function (arg0) {
    var ret = getObject(arg0).top;
    return ret;
}, arguments) };

function __wbg_left_0e681cb8fd277739() { return logError(function (arg0) {
    var ret = getObject(arg0).left;
    return ret;
}, arguments) };

function __wbg_identifier_afa8b01d4d901685() { return logError(function (arg0) {
    var ret = getObject(arg0).identifier;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_pageX_e0c8221ecfdb73d0() { return logError(function (arg0) {
    var ret = getObject(arg0).pageX;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_pageY_32100ad7039a744e() { return logError(function (arg0) {
    var ret = getObject(arg0).pageY;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_force_8e51e1fec066aade() { return logError(function (arg0) {
    var ret = getObject(arg0).force;
    return ret;
}, arguments) };

function __wbg_getPropertyValue_fd6ae3726bda9d7f() { return handleError(function (arg0, arg1, arg2, arg3) {
    var ret = getObject(arg1).getPropertyValue(getStringFromWasm0(arg2, arg3));
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_setProperty_ebb06e7fa941d6a8() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
}, arguments) };

function __wbg_getwithindex_5caaba1b5b3e6e18() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_instanceof_Window_434ce1849eb4e0fc() { return logError(function (arg0) {
    var ret = getObject(arg0) instanceof Window;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_document_5edd43643d1060d9() { return logError(function (arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_navigator_0e0588c949560476() { return logError(function (arg0) {
    var ret = getObject(arg0).navigator;
    return addHeapObject(ret);
}, arguments) };

function __wbg_innerWidth_405786923c1d2641() { return handleError(function (arg0) {
    var ret = getObject(arg0).innerWidth;
    return addHeapObject(ret);
}, arguments) };

function __wbg_innerHeight_25d3be0d129329c3() { return handleError(function (arg0) {
    var ret = getObject(arg0).innerHeight;
    return addHeapObject(ret);
}, arguments) };

function __wbg_devicePixelRatio_9632545370d525ae() { return logError(function (arg0) {
    var ret = getObject(arg0).devicePixelRatio;
    return ret;
}, arguments) };

function __wbg_performance_bbca4ccfaef860b2() { return logError(function (arg0) {
    var ret = getObject(arg0).performance;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_open_67fbcd7373a90ddc() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_requestAnimationFrame_0c71cd3c6779a371() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_setTimeout_1c75092906446b91() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_length_01a613025b5ffd74() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_item_b192ab411bbfbb09() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0).item(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_get_a765dab923455e0d() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_length_a2882c668bdf6488() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_get_1c01a7682a9775bb() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_id_79dca31d8297faf1() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).id;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_setid_73be37238eaa05be() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).id = getStringFromWasm0(arg1, arg2);
}, arguments) };

function __wbg_scrollLeft_e8aba47a94d12290() { return logError(function (arg0) {
    var ret = getObject(arg0).scrollLeft;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_getBoundingClientRect_534c1b96b6e612d3() { return logError(function (arg0) {
    var ret = getObject(arg0).getBoundingClientRect();
    return addHeapObject(ret);
}, arguments) };

function __wbg_getElementsByClassName_8a7d00ed3eaf1522() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementsByClassName(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

function __wbg_length_41b205f6892bf9d9() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_get_bdec89fd60d07530() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_dataTransfer_bc4c0501385a0c8e() { return logError(function (arg0) {
    var ret = getObject(arg0).dataTransfer;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_drawElementsInstancedANGLE_e184bb1bad14df88() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawElementsInstancedANGLE(arg1 >>> 0, arg2, arg3 >>> 0, arg4, arg5);
}, arguments) };

function __wbg_vertexAttribDivisorANGLE_2dc41a79843a435c() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).vertexAttribDivisorANGLE(arg1 >>> 0, arg2 >>> 0);
}, arguments) };

function __wbg_scrollTop_5ebd5c6591748d6e() { return logError(function (arg0) {
    var ret = getObject(arg0).scrollTop;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_hidden_f7a620ec4ab18ce5() { return logError(function (arg0) {
    var ret = getObject(arg0).hidden;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_sethidden_fdaefd7e7da7e4c0() { return logError(function (arg0, arg1) {
    getObject(arg0).hidden = arg1 !== 0;
}, arguments) };

function __wbg_style_16f5dd9624687c8f() { return logError(function (arg0) {
    var ret = getObject(arg0).style;
    return addHeapObject(ret);
}, arguments) };

function __wbg_offsetTop_45111254e7b26a1f() { return logError(function (arg0) {
    var ret = getObject(arg0).offsetTop;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_offsetLeft_be5393bf9eec5766() { return logError(function (arg0) {
    var ret = getObject(arg0).offsetLeft;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_offsetWidth_bc683e2f57ea2d6b() { return logError(function (arg0) {
    var ret = getObject(arg0).offsetWidth;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_setonload_9235de4503eb82c8() { return logError(function (arg0, arg1) {
    getObject(arg0).onload = getObject(arg1);
}, arguments) };

function __wbg_setonerror_939f617c2b40758c() { return logError(function (arg0, arg1) {
    getObject(arg0).onerror = getObject(arg1);
}, arguments) };

function __wbg_blur_2156876090506146() { return handleError(function (arg0) {
    getObject(arg0).blur();
}, arguments) };

function __wbg_focus_4434360545ac99cf() { return handleError(function (arg0) {
    getObject(arg0).focus();
}, arguments) };

function __wbg_responseURL_a3e549a88db1c1f7() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).responseURL;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_setresponseType_e5326d926ee8e787() { return logError(function (arg0, arg1) {
    getObject(arg0).responseType = takeObject(arg1);
}, arguments) };

function __wbg_response_8b12ac238727ae0e() { return handleError(function (arg0) {
    var ret = getObject(arg0).response;
    return addHeapObject(ret);
}, arguments) };

function __wbg_new_08dfde0f90155eb7() { return handleError(function () {
    var ret = new XMLHttpRequest();
    return addHeapObject(ret);
}, arguments) };

function __wbg_open_7190f43b39e7f488() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4), arg5 !== 0);
}, arguments) };

function __wbg_send_84c8dd943b775f78() { return handleError(function (arg0) {
    getObject(arg0).send();
}, arguments) };

function __wbg_clientX_849ccdf456d662ac() { return logError(function (arg0) {
    var ret = getObject(arg0).clientX;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_clientY_1aaff30fe0cd0876() { return logError(function (arg0) {
    var ret = getObject(arg0).clientY;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_ctrlKey_4e536bedb069129f() { return logError(function (arg0) {
    var ret = getObject(arg0).ctrlKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_metaKey_0b396e35a4941247() { return logError(function (arg0) {
    var ret = getObject(arg0).metaKey;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_button_a18f33eb55774d89() { return logError(function (arg0) {
    var ret = getObject(arg0).button;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_instanceof_WebGlRenderingContext_2be4c068bf5f8362() { return logError(function (arg0) {
    var ret = getObject(arg0) instanceof WebGLRenderingContext;
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_canvas_d0b58be124e596e3() { return logError(function (arg0) {
    var ret = getObject(arg0).canvas;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_bufferData_85d635f32a990208() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
}, arguments) };

function __wbg_bufferSubData_3a944e1fdad0cd9a() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferSubData(arg1 >>> 0, arg2, getObject(arg3));
}, arguments) };

function __wbg_readPixels_3692eaca9dfc7c0c() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).readPixels(arg1, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, getObject(arg7));
}, arguments) };

function __wbg_texImage2D_d26bd916ff0956a1() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
}, arguments) };

function __wbg_texImage2D_b46a9b691e69d90b() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4 >>> 0, arg5 >>> 0, getObject(arg6));
}, arguments) };

function __wbg_texSubImage2D_d907a4c940fd6e41() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, getObject(arg9));
}, arguments) };

function __wbg_texSubImage2D_d9dc0ffd91998f0d() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    getObject(arg0).texSubImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5 >>> 0, arg6 >>> 0, getObject(arg7));
}, arguments) };

function __wbg_uniform1fv_e6a2134edff4f2e9() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform1fv(getObject(arg1), getArrayF32FromWasm0(arg2, arg3));
}, arguments) };

function __wbg_uniformMatrix2fv_aaa4e0f7c15bca04() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix2fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}, arguments) };

function __wbg_uniformMatrix4fv_f07c6caf5a563616() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniformMatrix4fv(getObject(arg1), arg2 !== 0, getArrayF32FromWasm0(arg3, arg4));
}, arguments) };

function __wbg_activeTexture_74ed11a5c5d5af90() { return logError(function (arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
}, arguments) };

function __wbg_attachShader_55dbe770f3ee32ca() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}, arguments) };

function __wbg_bindBuffer_29d52e7bc48650c3() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
}, arguments) };

function __wbg_bindFramebuffer_bd35ddd23765c7b6() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).bindFramebuffer(arg1 >>> 0, getObject(arg2));
}, arguments) };

function __wbg_bindTexture_198c816345baca83() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
}, arguments) };

function __wbg_blendEquation_09d56f3be6f914f5() { return logError(function (arg0, arg1) {
    getObject(arg0).blendEquation(arg1 >>> 0);
}, arguments) };

function __wbg_blendFunc_c8f1e0fb4467f57c() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
}, arguments) };

function __wbg_blendFuncSeparate_494b1dae028cb9a9() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).blendFuncSeparate(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4 >>> 0);
}, arguments) };

function __wbg_clear_2af1271959ec83d7() { return logError(function (arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}, arguments) };

function __wbg_clearColor_51c4f69c743c3252() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
}, arguments) };

function __wbg_compileShader_3b5f9ef4c67a0777() { return logError(function (arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}, arguments) };

function __wbg_createBuffer_c40f37e1348bb91f() { return logError(function (arg0) {
    var ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_createFramebuffer_410b12a5cc5a8f13() { return logError(function (arg0) {
    var ret = getObject(arg0).createFramebuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_createProgram_245520da1fb9e47b() { return logError(function (arg0) {
    var ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_createShader_4d8818a13cb825b3() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_createTexture_f3a6a715d6bada45() { return logError(function (arg0) {
    var ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_cullFace_c6fb8a7309c36a38() { return logError(function (arg0, arg1) {
    getObject(arg0).cullFace(arg1 >>> 0);
}, arguments) };

function __wbg_deleteBuffer_c708688b9e1b3518() { return logError(function (arg0, arg1) {
    getObject(arg0).deleteBuffer(getObject(arg1));
}, arguments) };

function __wbg_deleteFramebuffer_ca006f8649d4550a() { return logError(function (arg0, arg1) {
    getObject(arg0).deleteFramebuffer(getObject(arg1));
}, arguments) };

function __wbg_deleteTexture_9159fb5927ed32c0() { return logError(function (arg0, arg1) {
    getObject(arg0).deleteTexture(getObject(arg1));
}, arguments) };

function __wbg_disable_2b63b75dc6c27537() { return logError(function (arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
}, arguments) };

function __wbg_disableVertexAttribArray_aa8458b40dd08914() { return logError(function (arg0, arg1) {
    getObject(arg0).disableVertexAttribArray(arg1 >>> 0);
}, arguments) };

function __wbg_drawArrays_22c88d644a33fd59() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}, arguments) };

function __wbg_drawElements_6e26500a25ecf478() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
}, arguments) };

function __wbg_enable_8f6dd779ccb8e1de() { return logError(function (arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
}, arguments) };

function __wbg_enableVertexAttribArray_4ed5f91d0718bee1() { return logError(function (arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
}, arguments) };

function __wbg_framebufferTexture2D_31643260e5b0b294() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, getObject(arg4), arg5);
}, arguments) };

function __wbg_getActiveUniform_3851244f8fc5db53() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getActiveUniform(getObject(arg1), arg2 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_getAttribLocation_da5df7094096113d() { return logError(function (arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_getExtension_c6ceee3244ee7f20() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getExtension(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_getProgramInfoLog_c253042b64e86027() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_getProgramParameter_4f698af0dda0a2d4() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_getShaderInfoLog_584794e3bcf1e19b() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_getShaderParameter_64b1ffe576e5fa25() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_getUniformLocation_703972f150a46500() { return logError(function (arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
}, arguments) };

function __wbg_linkProgram_5fdd57237c761833() { return logError(function (arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}, arguments) };

function __wbg_scissor_fb094c7db856e2a7() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
}, arguments) };

function __wbg_shaderSource_173ab97288934a60() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
}, arguments) };

function __wbg_texParameteri_caec5468f2a850c3() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}, arguments) };

function __wbg_uniform1f_258478814234cf9c() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).uniform1f(getObject(arg1), arg2);
}, arguments) };

function __wbg_uniform1i_a0275676828a22b6() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
}, arguments) };

function __wbg_uniform2f_b0af46ba675f2c0d() { return logError(function (arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
}, arguments) };

function __wbg_uniform3f_65416973a351fbab() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).uniform3f(getObject(arg1), arg2, arg3, arg4);
}, arguments) };

function __wbg_uniform4f_e5d0a91bf98b35ad() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
}, arguments) };

function __wbg_useProgram_d5898a40ebe88916() { return logError(function (arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}, arguments) };

function __wbg_vertexAttribPointer_0d097efa33e3f45f() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}, arguments) };

function __wbg_viewport_19577064127daf83() { return logError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
}, arguments) };

function __wbindgen_is_undefined(arg0) {
    var ret = getObject(arg0) === undefined;
    _assertBoolean(ret);
    return ret;
};

function __wbg_buffer_5e74a88a1424a2e0() { return logError(function (arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
}, arguments) };

function __wbg_get_f45dff51f52d7222() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
}, arguments) };

function __wbg_length_7b60f47bde714631() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_newnoargs_f579424187aa1717() { return logError(function (arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

function __wbg_call_89558c3e96703ca1() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

function __wbg_isArray_8480ed76e5369634() { return logError(function (arg0) {
    var ret = Array.isArray(getObject(arg0));
    _assertBoolean(ret);
    return ret;
}, arguments) };

function __wbg_resolve_4f8f547f26b30b27() { return logError(function (arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

function __wbg_then_a6860c82b90816ca() { return logError(function (arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

function __wbg_then_58a04e42527f52c6() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

function __wbg_self_e23d74ae45fb17d1() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };

function __wbg_window_b4be7f48b24ac56e() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };

function __wbg_globalThis_d61b1f48a57191ae() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

function __wbg_global_e7669da72fd7f239() { return handleError(function () {
    var ret = __webpack_require__.g.global;
    return addHeapObject(ret);
}, arguments) };

function __wbg_newwithbyteoffsetandlength_278ec7532799393a() { return logError(function (arg0, arg1, arg2) {
    var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_new_e3b800e570795b3c() { return logError(function (arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

function __wbg_set_5b8081e9d002f0df() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
}, arguments) };

function __wbg_length_30803400a8f15c59() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_newwithbyteoffsetandlength_bdb885cfc5e9bc43() { return logError(function (arg0, arg1, arg2) {
    var ret = new Uint16Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_newwithbyteoffsetandlength_ad2916c6fa7d4c6f() { return logError(function (arg0, arg1, arg2) {
    var ret = new Float32Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_new_f5438c0cea22a3aa() { return logError(function (arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
}, arguments) };

function __wbg_set_7cb6639737aebb39() { return logError(function (arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
}, arguments) };

function __wbg_length_44449d3b5928d07c() { return logError(function (arg0) {
    var ret = getObject(arg0).length;
    _assertNum(ret);
    return ret;
}, arguments) };

function __wbg_newwithlength_5f4ce114a24dfe1e() { return logError(function (arg0) {
    var ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_newwithlength_747b31c525d823ec() { return logError(function (arg0) {
    var ret = new Float32Array(arg0 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_subarray_e729e242fb317565() { return logError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
}, arguments) };

function __wbg_parse_e3e7e590474b89d2() { return handleError(function (arg0, arg1) {
    var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

function __wbg_new_59cb74e423758ede() { return logError(function () {
    var ret = new Error();
    return addHeapObject(ret);
}, arguments) };

function __wbg_stack_558ba5917b466edd() { return logError(function (arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
}, arguments) };

function __wbg_error_4bb6c2a97407129a() { return logError(function (arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);
    }
}, arguments) };

function __wbindgen_debug_string(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

function __wbindgen_memory() {
    var ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory;
    return addHeapObject(ret);
};

function __wbindgen_closure_wrapper1458() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 235, __wbg_adapter_26);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper1460() { return logError(function (arg0, arg1, arg2) {
    var ret = makeClosure(arg0, arg1, 232, __wbg_adapter_29);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2297() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 476, __wbg_adapter_32);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2298() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 490, __wbg_adapter_35);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2300() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 480, __wbg_adapter_38);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2302() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 482, __wbg_adapter_41);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2304() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 484, __wbg_adapter_44);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2306() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 486, __wbg_adapter_47);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2308() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 488, __wbg_adapter_50);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2310() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 492, __wbg_adapter_53);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2313() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 478, __wbg_adapter_56);
    return addHeapObject(ret);
}, arguments) };

function __wbindgen_closure_wrapper2456() { return logError(function (arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 502, __wbg_adapter_59);
    return addHeapObject(ret);
}, arguments) };



/***/ }),

/***/ "./src/core/pkg-webgl1/snippets/al-ui-d4d455bd9e91f451/js/hpxImageSurvey.js":
/*!**********************************************************************************!*\
  !*** ./src/core/pkg-webgl1/snippets/al-ui-d4d455bd9e91f451/js/hpxImageSurvey.js ***!
  \**********************************************************************************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "fetchSurveyMetadata": () => (/* binding */ fetchSurveyMetadata)
/* harmony export */ });
/* harmony import */ var _js_HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ../../../../../js/HiPSDefinition.js */ "./src/js/HiPSDefinition.js");
/* harmony import */ var _js_HpxImageSurvey_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ../../../../../js/HpxImageSurvey.js */ "./src/js/HpxImageSurvey.js");
/* harmony import */ var _js_Utils_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ../../../../../js/Utils.js */ "./src/js/Utils.js");





async function fetchSurveyMetadata(rootURL) {
    // Use the url for retrieving the HiPS properties
    // remove final slash
    if (rootURL.slice(-1) === '/') {
        rootURL = rootURL.substr(0, rootURL.length-1);
    }

    // make URL absolute
    rootURL = _js_Utils_js__WEBPACK_IMPORTED_MODULE_2__.Utils.getAbsoluteURL(rootURL);

    // fast fix for HTTPS support --> will work for all HiPS served by CDS
    if (_js_Utils_js__WEBPACK_IMPORTED_MODULE_2__.Utils.isHttpsContext() && ( /u-strasbg.fr/i.test(rootURL) || /unistra.fr/i.test(rootURL)  ) ) {
        rootURL = rootURL.replace('http://', 'https://');
    }

    const url = rootURL + '/properties';
    console.log("properties url", url);
    let metadata = await fetch(url)
        .then((response) => response.text());
    // We get the property here
    metadata = _js_HiPSDefinition_js__WEBPACK_IMPORTED_MODULE_0__.HiPSDefinition.parseHiPSProperties(metadata);

    // 1. Ensure there is exactly one survey matching
    if (!metadata) {
        throw 'no surveys matching';
    }
    console.log(metadata)
    return metadata;
}


/***/ }),

/***/ "./src/core/pkg-webgl1/index_bg.wasm":
/*!*******************************************!*\
  !*** ./src/core/pkg-webgl1/index_bg.wasm ***!
  \*******************************************/
/***/ ((module, exports, __webpack_require__) => {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.id];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name) exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./index_bg.js */ "./src/core/pkg-webgl1/index_bg.js");


// exec wasm module
wasmExports[""]()

/***/ })

}]);
//# sourceMappingURL=src_core_pkg-webgl1_index_js.aladin.js.map