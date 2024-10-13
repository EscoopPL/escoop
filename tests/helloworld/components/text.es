// Comments in Escoop can be Single-line
/*
	Or Multi-line
*/

identifier helloworld components text-comp // Every file in Escoop must start with an identifier statement, which is the identifier other files in the program will use to reference this file.
class text-comp extends component // Every file in Escoop must have a file declaration after the identifier statement. This is a class declaration, which tells Escoop that this file is a class. Specifically, it tells Escoop that this file contains a class called text-comp, which inherits from the string-component class. Classes can inherit from the object class, component class, a user-defined class, or a standard library class.

string value // This is a variable declaration of type "string" and name "value"

construct string value is // This is a constructor declaration that takes a string called "value". "is" starts a block.
	self value = value // This assigns the value variable to the value parameter
end // This ends the block started by "is".

compfunc string get-value is // This defines a component function called get-value that returns a string and has no arguments.
	return value // This will return the value variable.
end // This ends the block started by "is".