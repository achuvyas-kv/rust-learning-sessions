// JavaScript: Garbage Collection
function main() {
	let numbers = [0, 10, 20, 30, 40];

	// Use the memory
	for (let i = 0; i < numbers.length; i++) {
		console.log(`numbers[${i}] = ${numbers[i]}`);
	}

	// Modify the array
	numbers = numbers.map(num => num * 2);
	console.log("After modification:", numbers);

	// Create a new object
	let obj = { data: numbers };
	console.log("Object created:", obj);

	// Reassign - old memory becomes eligible for garbage collection
	obj = null;  // Explicitly mark for GC (optional)
	numbers = null;  // Old array becomes eligible for GC




	// Garbage collector will automatically free memory when no references remain
	// No manual cleanup needed!
}

main();

// Memory is automatically managed:
// - Allocated when variables are created
// - Freed when no references remain
// - No manual malloc/free needed
// - No ownership rules to follow 
