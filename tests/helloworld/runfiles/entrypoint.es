identifier helloworld runfiles entrypoint
runfile entrypoint

import helloworld components text-comp
import helloworld objects printer

func void run array string args is
	array text-comp text = bulk text-comp args
	printer text-printer = new printer
	bulk text-printer text add-comp text
	text-printer print-contents
end