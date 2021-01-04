(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./node_modules/@fxpineau/healpix/healpix.js":
/*!***************************************************!*\
  !*** ./node_modules/@fxpineau/healpix/healpix.js ***!
  \***************************************************/
/*! exports provided: getOrderMax, lonlatToNested, multiLonlatToNested, nestedCenter, nestedVertices, nestedNeighbours, nestedQueryConeBMOC, nestedQueryCone, nestedQueryPolygonBMOC, nestedQueryPolygon, BMOC, Cell, Neighbours, PolygonMap, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./healpix_bg.wasm */ "./node_modules/@fxpineau/healpix/healpix_bg.wasm");
/* harmony import */ var _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./healpix_bg.js */ "./node_modules/@fxpineau/healpix/healpix_bg.js");
/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "getOrderMax", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["getOrderMax"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "lonlatToNested", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["lonlatToNested"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "multiLonlatToNested", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["multiLonlatToNested"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedCenter", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedCenter"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedVertices", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedVertices"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedNeighbours", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedNeighbours"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedQueryConeBMOC", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedQueryConeBMOC"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedQueryCone", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedQueryCone"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedQueryPolygonBMOC", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedQueryPolygonBMOC"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "nestedQueryPolygon", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["nestedQueryPolygon"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "BMOC", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["BMOC"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "Cell", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["Cell"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "Neighbours", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["Neighbours"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "PolygonMap", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["PolygonMap"]; });

/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return _healpix_bg_js__WEBPACK_IMPORTED_MODULE_1__["__wbindgen_throw"]; });




/***/ }),

/***/ "./node_modules/@fxpineau/healpix/healpix_bg.js":
/*!******************************************************!*\
  !*** ./node_modules/@fxpineau/healpix/healpix_bg.js ***!
  \******************************************************/
/*! exports provided: getOrderMax, lonlatToNested, multiLonlatToNested, nestedCenter, nestedVertices, nestedNeighbours, nestedQueryConeBMOC, nestedQueryCone, nestedQueryPolygonBMOC, nestedQueryPolygon, BMOC, Cell, Neighbours, PolygonMap, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
__webpack_require__.r(__webpack_exports__);
/* WEBPACK VAR INJECTION */(function(TextDecoder, module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "getOrderMax", function() { return getOrderMax; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "lonlatToNested", function() { return lonlatToNested; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "multiLonlatToNested", function() { return multiLonlatToNested; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedCenter", function() { return nestedCenter; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedVertices", function() { return nestedVertices; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedNeighbours", function() { return nestedNeighbours; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedQueryConeBMOC", function() { return nestedQueryConeBMOC; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedQueryCone", function() { return nestedQueryCone; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedQueryPolygonBMOC", function() { return nestedQueryPolygonBMOC; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "nestedQueryPolygon", function() { return nestedQueryPolygon; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "BMOC", function() { return BMOC; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Cell", function() { return Cell; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "Neighbours", function() { return Neighbours; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "PolygonMap", function() { return PolygonMap; });
/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, "__wbindgen_throw", function() { return __wbindgen_throw; });
/* harmony import */ var _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./healpix_bg.wasm */ "./node_modules/@fxpineau/healpix/healpix_bg.wasm");


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetUint8Memory0 = new Uint8Array(_healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* Returns the max order one can use in Javascipts.
* Equals 24 since we store ou HEALPix indices on a Javascript number which is a double.
* A double has a mantissa of 52 bits = 24 * 2 (2 bits per level) + 4 (4 bits to code base cells).
* @returns {number}
*/
function getOrderMax() {
    var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["getOrderMax"]();
    return ret;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetFloat64Memory0 = new Float64Array(_healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetFloat64Memory0;
}

let WASM_VECTOR_LEN = 0;

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8);
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetInt32Memory0 = new Int32Array(_healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetInt32Memory0;
}

let cachegetUint32Memory0 = null;
function getUint32Memory0() {
    if (cachegetUint32Memory0 === null || cachegetUint32Memory0.buffer !== _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer) {
        cachegetUint32Memory0 = new Uint32Array(_healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["memory"].buffer);
    }
    return cachegetUint32Memory0;
}

function getArrayU32FromWasm0(ptr, len) {
    return getUint32Memory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
* Returns the cell number (hash value) in the NESTED scheme associated with the given position
* on the unit sphere
* # Inputs
* - `order`: the order of the HEALPix hash we want in output
* - `lon`: longitude in degrees, support reasonably large positive and negative values
*          producing accurate results with a naive range reduction like modulo 360
*          (i.e. without having to resort on Cody-Waite or Payne Hanek range reduction).
* - `lat`: latitude in degrees, must be in `[-90, 90]`
* # Output
* - the cell number (hash value) associated with the given position on the unit sphere,
*   in `[0, 12*nside^2[`
* # Panics
*   If `lat` **not in** `[-90, 90]`, this method panics.
* @param {number} depth
* @param {number} lon
* @param {number} lat
* @returns {number}
*/
function lonlatToNested(depth, lon, lat) {
    var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["lonlatToNested"](depth, lon, lat);
    return ret;
}

function getArrayF64FromWasm0(ptr, len) {
    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
* Returns the cell numbers (hash value) in the NESTED scheme associated with the given positions
* on the unit sphere
* # Inputs
* - `order`: the order of the HEALPix hashes we want in output
* - `coords`: an array storing consecutively the coordinates of the positions we look for the
*             cell numbers: `[lon_1, lat_1, lon_2, lat_2, ..., lon_n, lat_n]`
*   - `lon`: longitude in degrees, support reasonably large positive and negative values
*            producing accurate results with a naive range reduction like modulo 2*pi
*            (i.e. without having to resort on Cody-Waite or Payne Hanek range reduction).
*   - `lat`: latitude in degrees, must be in `[-90, 90]`
* # Output
* - the cell number (hash value) associated with the given position on the unit sphere,
*   in `[0, 12*nside^2[`
* # Panics
*   If `lat` **not in** `[-90, 90]`, this method panics.
* @param {number} depth
* @param {Float64Array} coords
* @returns {Float64Array}
*/
function multiLonlatToNested(depth, coords) {
    var ptr0 = passArrayF64ToWasm0(coords, _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
    var len0 = WASM_VECTOR_LEN;
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["multiLonlatToNested"](8, depth, ptr0, len0);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v1 = getArrayF64FromWasm0(r0, r1).slice();
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
    return v1;
}

/**
* Compute the position on the unit sphere of the center (in the Euclidean projection plane)
* of the cell associated to the given cell number.
* # Input
* - `order`: the order of the cell
* - `icell`: the cell number value of the cell we look for the unprojected center, in the NESTED scheme
* # Output
* - `[lon, lat]` in degrees, the unprojected position (on the unit sphere) of the center of
*   the cell in the Euclidean plane
*   - `lon`, longitude in `[0, 360]` degrees;
*   - `lat`, latitude in `[-90, 90]` degrees.
*
* # Panics
* If the given `hash` value is not in `[0, 12*nside^2[`, this method panics.
* @param {number} depth
* @param {number} hash
* @returns {Float64Array}
*/
function nestedCenter(depth, hash) {
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedCenter"](8, depth, hash);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v0 = getArrayF64FromWasm0(r0, r1).slice();
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
    return v0;
}

/**
* Computes the location on the unit sphere of the 4 vertices of the given HEALPix cell
* (define by its depth and number).
* # Inputs
* - `order` the order of the cell we look for the vertices
* - `icell`: the cell number value of the cell we look for the unprojected center, in the NESTED scheme
* # Output
* - array containing the longitudes and latitudes (in degrees) of the vertices in the following order:
*   `[SouthLon, SouthLat, EastLon, EastLat, NoethLon, NorthLat, WestLon, WestLat]`
* @param {number} depth
* @param {number} hash
* @returns {Float64Array}
*/
function nestedVertices(depth, hash) {
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedVertices"](8, depth, hash);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v0 = getArrayF64FromWasm0(r0, r1).slice();
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
    return v0;
}

/**
* Returns the hash values of all the neighbour cells of the cell of given cell number.
* The given cell itself is included.
* # Inputs
* - `order` the order of the cell we look for the neighbours
* - `icell` the nested number of the cell we look for the neighbours
* @param {number} depth
* @param {number} hash
* @returns {Neighbours}
*/
function nestedNeighbours(depth, hash) {
    var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedNeighbours"](depth, hash);
    return Neighbours.__wrap(ret);
}

/**
* Returns the BMOC covered by the given cone.
* # Inputs
* - `order`: the maximum order of the cell in the returned BMOC
* - `cone_lon`: cone center longitude, in degrees
* - `cone_lat`: cone center latitude in degrees, must be in `[-90, 90]`
* - `cone_radius`: radius of the cone, in degrees
* # Outputs
* - the BMOC covered by the given cone
* @param {number} depth
* @param {number} cone_lon
* @param {number} cone_lat
* @param {number} cone_radius
* @returns {BMOC}
*/
function nestedQueryConeBMOC(depth, cone_lon, cone_lat, cone_radius) {
    var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedQueryConeBMOC"](depth, cone_lon, cone_lat, cone_radius);
    return BMOC.__wrap(ret);
}

/**
* Returns the flat list of HEALPix cell of given order covered by the given cone.
* # Inputs
* - `order`: the order of the returned cells
* - `cone_lon`: cone center longitude, in degrees
* - `cone_lat`: cone center latitude in degrees, must be in `[-90, 90]`
* - `cone_radius`: radius of the cone, in degrees
* # Outputs
* - the flat list of HEALPix cell of given order covered by the given cone
* @param {number} depth
* @param {number} cone_lon
* @param {number} cone_lat
* @param {number} cone_radius
* @returns {Float64Array}
*/
function nestedQueryCone(depth, cone_lon, cone_lat, cone_radius) {
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedQueryCone"](8, depth, cone_lon, cone_lat, cone_radius);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v0 = getArrayF64FromWasm0(r0, r1).slice();
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
    return v0;
}

/**
* Returns the BMOC covered by the given polygon.
* # Inputs
* - `order`: the maximum order of the cell in the returned BMOC
* - `vertices_coos`: the flat list of polygon vertices coordinates, in radians
*    `[v1.lon, v1.lat, v2.lon, v2.lat, ..., vN.lon, vN.lat]`
* # Outputs
* - the BMOC covered by the given polygon
* @param {number} depth
* @param {Float64Array} vertices_coos
* @returns {BMOC}
*/
function nestedQueryPolygonBMOC(depth, vertices_coos) {
    var ptr0 = passArrayF64ToWasm0(vertices_coos, _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
    var len0 = WASM_VECTOR_LEN;
    var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedQueryPolygonBMOC"](depth, ptr0, len0);
    return BMOC.__wrap(ret);
}

/**
* Returns the BMOC covered by the given polygon.
* # Inputs
* - `order`: the order of the returned cells
* - `vertices_coos`: the flat list of polygon vertices coordinates, in radians
*    `[v1.lon, v1.lat, v2.lon, v2.lat, ..., vN.lon, vN.lat]`
* # Outputs
* - the BMOC covered by the given polygon
* @param {number} depth
* @param {Float64Array} vertices_coos
* @returns {Float64Array}
*/
function nestedQueryPolygon(depth, vertices_coos) {
    var ptr0 = passArrayF64ToWasm0(vertices_coos, _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
    var len0 = WASM_VECTOR_LEN;
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["nestedQueryPolygon"](8, depth, ptr0, len0);
    var r0 = getInt32Memory0()[8 / 4 + 0];
    var r1 = getInt32Memory0()[8 / 4 + 1];
    var v1 = getArrayF64FromWasm0(r0, r1).slice();
    _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 8);
    return v1;
}

/**
* Represents a BMOC object resulting from a cone search or a polygon query.
* For practical reasons of data exchange between Javascript and WebAssembly, each elements
* of a cell (depth, hash, flag) are stored in separate arrays.
* (I firs wanted to return an array of Cell, but so far it is not possible with wasm-bindgen)
*/
class BMOC {

    static __wrap(ptr) {
        const obj = Object.create(BMOC.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_bmoc_free"](ptr);
    }
    /**
    * @returns {number}
    */
    get n_cells() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_bmoc_n_cells"](this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    get depth_max() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_bmoc_depth_max"](this.ptr);
        return ret;
    }
    /**
    * Returns the number of cells (of various depth) in the BMOC
    * @returns {number}
    */
    getSize() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_bmoc_n_cells"](this.ptr);
        return ret >>> 0;
    }
    /**
    * Returns the maximal depth of the BMOC
    * @returns {number}
    */
    getDepth() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_bmoc_depth_max"](this.ptr);
        return ret;
    }
    /**
    * Utility method replacing the calls to `getCellDepth`, `getCellHash` and `getCellFlag`
    * # Inputs
    * - `i` the index of the cell in the BMOC
    * @param {number} icell
    * @returns {Cell}
    */
    getCell(icell) {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["bmoc_getCell"](this.ptr, icell);
        return Cell.__wrap(ret);
    }
    /**
    * Returns the depth of cell number `i` in the BMOC
    * # Inputs
    * - `i` the index of the cell in the BMOC
    * @param {number} icell
    * @returns {number}
    */
    getCellDepth(icell) {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["bmoc_getCellDepth"](this.ptr, icell);
        return ret;
    }
    /**
    * Returns the hash value (or pixel number) of the cell number `i` in the BMOC
    * # Inputs
    * - `i` the index of the cell in the BMOC
    * @param {number} icell
    * @returns {number}
    */
    getCellHash(icell) {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["bmoc_getCellHash"](this.ptr, icell);
        return ret;
    }
    /**
    * Returns the status flag of the cell number `i` in the BMOC
    * (`true`: cell fully covered; `false`: cell partially covered)
    * # Inputs
    * - `i` the index of the cell in the BMOC
    * @param {number} icell
    * @returns {boolean}
    */
    getCellFlag(icell) {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["bmoc_getCellFlag"](this.ptr, icell);
        return ret !== 0;
    }
}
/**
* Represents a BMOC cell: its depth, number and flag telling if the cell is fully/partially covered.
*/
class Cell {

    static __wrap(ptr) {
        const obj = Object.create(Cell.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_cell_free"](ptr);
    }
    /**
    * The order of the HEALPix cell
    * @returns {number}
    */
    get order() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_cell_order"](this.ptr);
        return ret;
    }
    /**
    * The nested HEALPix cell number
    * @returns {number}
    */
    get icell() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_cell_icell"](this.ptr);
        return ret;
    }
    /**
    * The flag telling if the cell if fully or partially covered
    * @returns {boolean}
    */
    get isfull() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_cell_isfull"](this.ptr);
        return ret !== 0;
    }
}
/**
* Contains the list of the neighbours of the `center` HEALPix cell.
* In general, a cell has 8 neighbours.
* But the top and bottom corners of equatorial base cells do not have
* north and south neighbours respectively.
* The left most and right most corners of the polar caps base cells do not have
* west and east corners respectively.
*/
class Neighbours {

    static __wrap(ptr) {
        const obj = Object.create(Neighbours.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_neighbours_free"](ptr);
    }
    /**
    * @returns {number | undefined}
    */
    get south() {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_south"](8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getFloat64Memory0()[8 / 8 + 1];
        return r0 === 0 ? undefined : r1;
    }
    /**
    * @returns {number}
    */
    get southeast() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_southeast"](this.ptr);
        return ret;
    }
    /**
    * @returns {number | undefined}
    */
    get east() {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_east"](8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getFloat64Memory0()[8 / 8 + 1];
        return r0 === 0 ? undefined : r1;
    }
    /**
    * @returns {number}
    */
    get southwest() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_southwest"](this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get center() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_center"](this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    get norhteast() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_norhteast"](this.ptr);
        return ret;
    }
    /**
    * @returns {number | undefined}
    */
    get west() {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_west"](8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getFloat64Memory0()[8 / 8 + 1];
        return r0 === 0 ? undefined : r1;
    }
    /**
    * @returns {number}
    */
    get norhtwest() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_norhtwest"](this.ptr);
        return ret;
    }
    /**
    * @returns {number | undefined}
    */
    get north() {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_get_neighbours_north"](8, this.ptr);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getFloat64Memory0()[8 / 8 + 1];
        return r0 === 0 ? undefined : r1;
    }
}
/**
*/
class PolygonMap {

    static __wrap(ptr) {
        const obj = Object.create(PolygonMap.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbg_polygonmap_free"](ptr);
    }
    /**
    * @returns {PolygonMap}
    */
    static new() {
        var ret = _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["polygonmap_new"]();
        return PolygonMap.__wrap(ret);
    }
    /**
    * Add a polygon of given identifier to the map, the provided liste is:
    * [ra_v1_deg, dec_v1_deg, ra_v2_deg, de_v2_deg, ..., ra_vn_deg, de_vn_deg]
    * @param {number} id
    * @param {Float64Array} vertices_coos
    */
    addPolygon(id, vertices_coos) {
        var ptr0 = passArrayF64ToWasm0(vertices_coos, _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_malloc"]);
        var len0 = WASM_VECTOR_LEN;
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["polygonmap_addPolygon"](this.ptr, id, ptr0, len0);
    }
    /**
    * Remove the polygon of given identifier to the map.
    * @param {number} id
    */
    rmPolygon(id) {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["polygonmap_rmPolygon"](this.ptr, id);
    }
    /**
    * Returns the list of the identifiers of the polygon containing the given point.
    * @param {number} ra_deg
    * @param {number} de_deg
    * @returns {Uint32Array}
    */
    polygonContaining(ra_deg, de_deg) {
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["polygonmap_polygonContaining"](8, this.ptr, ra_deg, de_deg);
        var r0 = getInt32Memory0()[8 / 4 + 0];
        var r1 = getInt32Memory0()[8 / 4 + 1];
        var v0 = getArrayU32FromWasm0(r0, r1).slice();
        _healpix_bg_wasm__WEBPACK_IMPORTED_MODULE_0__["__wbindgen_free"](r0, r1 * 4);
        return v0;
    }
}

const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};


/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! text-encoding */ "./node_modules/text-encoding/index.js")["TextDecoder"], __webpack_require__(/*! ./../../webpack/buildin/harmony-module.js */ "./node_modules/webpack/buildin/harmony-module.js")(module)))

/***/ }),

/***/ "./node_modules/@fxpineau/healpix/healpix_bg.wasm":
/*!********************************************************!*\
  !*** ./node_modules/@fxpineau/healpix/healpix_bg.wasm ***!
  \********************************************************/
/*! exports provided: memory, getOrderMax, __wbg_polygonmap_free, polygonmap_new, polygonmap_addPolygon, polygonmap_rmPolygon, polygonmap_polygonContaining, lonlatToNested, multiLonlatToNested, nestedCenter, nestedVertices, __wbg_neighbours_free, __wbg_get_neighbours_south, __wbg_get_neighbours_southeast, __wbg_get_neighbours_east, __wbg_get_neighbours_southwest, __wbg_get_neighbours_center, __wbg_get_neighbours_norhteast, __wbg_get_neighbours_west, __wbg_get_neighbours_norhtwest, __wbg_get_neighbours_north, nestedNeighbours, __wbg_bmoc_free, __wbg_get_bmoc_n_cells, __wbg_get_bmoc_depth_max, __wbg_cell_free, __wbg_get_cell_order, __wbg_get_cell_icell, __wbg_get_cell_isfull, bmoc_getCell, bmoc_getCellDepth, bmoc_getCellHash, bmoc_getCellFlag, nestedQueryConeBMOC, nestedQueryCone, nestedQueryPolygonBMOC, nestedQueryPolygon, bmoc_getSize, bmoc_getDepth, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.i];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name != "__webpack_init__") exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./healpix_bg.js */ "./node_modules/@fxpineau/healpix/healpix_bg.js");


// exec wasm module
wasmExports["__webpack_init__"]()

/***/ })

}]);
//# sourceMappingURL=1.aladin.js.map