mergeInto(LibraryManager.library, {
	aussie_time: function() {
		const str = new Date().toLocaleString("en-AU", {timeZone: "Australia/Sydney"});
		const intArray = Module.intArrayFromString(str)
		const ptr = Module._malloc(intArray.length)
		Module.writeArrayToMemory(intArray, ptr)

		return ptr
	}
})