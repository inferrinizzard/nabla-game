import katex from 'katex';

/**
 * Renders a KaTeX expression to a new element and appends it to the DOM
 * @param {String} str - The KaTeX expression to render
 * @returns DOMElement - The rendered element
 */
export const js_render_katex = str => {
	let element = document.createElement('div');
	element.className = 'katex-item';

	katex.render(str, element, { throwOnError: false, displayMode: true });
	document.getElementById('katex').appendChild(element);

	return element;
};

/**
 * Finds the element with id `id`, creating one if not present and renders a KaTeX expression to it
 * @param {String} str - The KaTeX expression to render
 * @param {String} id - The id of the element on which to render the expression
 * @returns DOMElement - The rendered element
 */
export const js_render_katex_element = (str, id) => {
	let element = document.getElementById(id);
	if (!element && str.length) {
		element = document.createElement('div');
		element.className += 'katex-item';
		element.id = id;
		document.getElementById('katex').appendChild(element);
	}

	katex.render(str, element, { throwOnError: false, displayMode: true });
	return element;
};
