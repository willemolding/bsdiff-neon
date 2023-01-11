const assert = require("assert");
const crypto = require("crypto");

const bsdiff = require("..");

describe('Bsdiff', function () {
    it('should be able to calculate a diff and then apply using patch', function () {
        const oldData = crypto.randomBytes(1024).buffer;
        const newData = crypto.randomBytes(1024).buffer;
        
        const diff = bsdiff.diff(oldData, newData);
        const newRecovered = bsdiff.patch(oldData, diff, 1024);
        
        assert.deepEqual(newData, newRecovered);
    });
});
