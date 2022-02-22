import katex from 'katex';

export function test_katex(str) {
	let latex = katex.renderToString(str, { throwOnError: false, displayMode: true });

	let x = document.createElement('div');
	x.innerHTML = latex;
	document.body.appendChild(x);
	console.log(x);

	let y = document.createElement('div');
	katex.render('x = \\int_{-\\infty}^\\infty\\xi\\,e^{2 \\pi i \\xi x}\\,d\\xi', y, {
		throwOnError: false,
		displayMode: true,
	});
	document.body.appendChild(y);
	console.log(y);
}
