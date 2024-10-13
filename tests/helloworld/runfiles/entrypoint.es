// Comments in Escoop can be Single-line
/*
	Or Multi-line
*/

identifier helloworld runfiles entrypoint // Every file in Escoop must start with an identifier statement, this is the identifier other files in the program will use to reference this file.
runfile entrypoint // This is a runfile declaration, and every file must contain either a runfile declaration, or a class declaration.

import helloworld components text-comp as text // This is importing the text-comp component, but instead of using the default name of text-comp, it's renamed to text. "helloworld components text-comp" is it's identifier.
import helloworld objects printer // This is importing the printer object, and "helloworld objects printer" is it's identifier.

func void run array string args is // This is the run function declaration. "func" means that a function is being declared. "void" is the functions return type. "run" is the name of the function. "array string args" is one of the arguments to the function. "is" is the start of a block, along with "then" and "do". A run function declaration must be present in every runfile, and it must be "func void run array string args is."
	array text messages = call bulk text args // This is a variable declaration. "array text" is the variable type, "name" is the value, and "call bulk text args" is the value. "args" is referencing the "args" parameter to the run function, "text" is a constructor for the text-comp component, "call" is a keyword used to call text, and "bulk" is a keyword.
	/*
		The bulk keyword targets a function, and turns it's arguments and return type into an array.

		array text messages = call bulk text args

		is the same as

		array text messages
		foreach i args do
			messages append call text i
		end

		text is a constructor that has a string input, and a text output, but by using the bulk keyword, those can be changed to and array of strings as input, and an array of texts as output.
	*/
	printer text-printer = call printer // This is another variable declaration, where "printer" is the type, "text-printer" is the name, and "call printer" is the value. "printer" is a constructor for the printer object, which is being called.
	/*
		Escoop has first-class functions, but there aren't parentheses to use to call functions, so the call keyword is used instead.

		call function

		is the same as

		function()

		in other languages, however

		function

		is the same as

		function

		in languages with first-class functions.

		however, in this case, "printer" is technically a class that is being called.
	*/
	call bulk text-printer text add-component messages // This is a function call to "text-printer text add-component" this means to look at "text-printer", and find the "text" member, and call the add-component function within it. "text" is a component-list, so add-component is built in to it.
	call text-printer print-contents // This is a function call to "text-printer print-contents" which is a function.
end // This ends the block.