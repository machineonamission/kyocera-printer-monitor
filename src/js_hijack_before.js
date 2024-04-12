// this is some extremely jank JS code that is put before printer returned JS to essentially trick it into running
// the code without the stupid dependencies it normally has
var ModelFactory = {};
ModelFactory.type = {
    MODEL: 1,
};
ModelFactory.create = function (a, b) {
    return b.prototype.constructor = b,
        b.prototype.name = "",
        e = new b;
}
// gpp is where the properties are saved to
var gpp = {properties: {}};
var Model = {};
Model.apply = function () {
    return {pp: function () {return gpp}};
}