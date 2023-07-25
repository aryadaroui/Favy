interface MyObject {
	name: string;
	field: string;
}

const my_map = new Map<number, MyObject>();
my_map.set(1, { name: "Object 1", field: "Value 1" });
my_map.set(2, { name: "Object 2", field: "Value 2" });

const obj = my_map.get(1);
if (obj) {
	obj.field = "Updated Value";
	// my_map.set(1, obj);
}

console.log(my_map.get(1)); // Output: { name: "Object 1", field: "Updated Value" }
