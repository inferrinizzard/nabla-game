import katex from 'katex';

export const js_render_katex = str => {
	let element = document.createElement('div');
	element.className = 'katex-item';

	katex.render(str, element, { throwOnError: false, displayMode: true });
	document.getElementById('katex').appendChild(element);

	return element;
};

export const js_render_katex_element = (str, id) => {
	let element = document.getElementById(id);
	if (!element) {
		element = document.createElement('div');
		element.className += 'katex-item';
		element.id = id;
		document.getElementById('katex').appendChild(element);
	}

	katex.render(str, element, { throwOnError: false, displayMode: true });
	return element;
};
