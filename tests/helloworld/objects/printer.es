// Comments in Escoop can be Single-line
/*
	Or Multi-line
*/

identifier helloworld objects printer // Every file in Escoop must start with an identifier statement, which is the identifier other files in the program will use to reference this file.
class printer extends object // Every file in Escoop must have a file declaration after the identifier statement. This is a class declaration, which tells Escoop that this file is a class. Specifically, it tells Escoop that this file contains a class called printer, which inherits from the object class. Classes can inherit from the object class, component class, a user-defined class, or a standard library class.

import helloworld components text-comp // This imports the string-component class found in the "escoop component" package.

component-list text-comp text // This defines a variable of type "component-list" with contents of "string-component" with a name of "text."

func void print-contents is // "func void print-contents" defines a function that returns nothing called print-contents. "is" is one of 3 keywords used to start a block, the others being "do", and "then."
	call bulk print call text get-value // "print" is a function that takes a string and prints it onto the terminal. However, "text get-values" is a function that returns an array of strings.
	/*
		However, the bulk keyword targets a function, and turns it's arguments and return type into an array.

		call bulk print text call text get-values

		is the same as

		foreach i call text get-values do
			call print i
		end

		text is a constructor that has a string input, and a text output, but by using the bulk keyword, those can be changed to and array of strings as input, and an array of texts as output.
	*/
end // "end" ends the block started by "is."