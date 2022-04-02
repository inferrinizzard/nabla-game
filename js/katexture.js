import katex from 'katex';

const remToPx = rem => {
	return parseFloat(rem) * parseFloat(getComputedStyle(document.documentElement).fontSize);
};

let textureCanvas = document.getElementById('textureCanvas');
let textureContext = textureCanvas.getContext('2d');
let staging = document.getElementById('staging');

const sizeBase = 9;
const scale = 2;
const playerHeightBase = sizeBase + 'rem';
const playerHeight = remToPx(playerHeightBase) * scale;

const playerWidth = playerHeight * 0.75;
const gutter = playerWidth / 4.0;

textureCanvas.setAttribute('height', playerHeight * 4);
textureCanvas.setAttribute('width', playerWidth * 6);

staging.style.height = playerHeight + 'px';
staging.style.width = playerWidth + 'px';
staging.style.left = playerWidth + 'px';

const cornerElement = '\\boldsymbol{\\in}';
const cornerFunction = '\\Bbb{F}';
const cardsSmall = [
	'0', // BasisCard::Zero,
	'1', // BasisCard::One,
	'x', // BasisCard::X,
	'x^{2}', // BasisCard::X2,
	'e^{x}', // BasisCard::E,
	'\\div', // AlgebraicCard::Div,
	'\\times', // AlgebraicCard::Mult,
	'\\sqrt{}', // AlgebraicCard::Sqrt,
	'\\nabla', // DerivativeCard::Laplacian,
	'\\Delta', // DerivativeCard::Nabla,
];
const cardsStandard = [
	'\\cos(x)', // BasisCard::Cos,
	'\\sin(x)', // BasisCard::Sin,
	'f^{-1}', // AlgebraicCard::Inverse,
	'\\ln', // AlgebraicCard::Log,
	'\\log', // AlgebraicCard::Log,
	'\\frac{d}{dx}', // DerivativeCard::Derivative,
	'\\int', // DerivativeCard::Integral,
	'\\lim\\limits_{x\\rightarrow+\\infty}', // LimitCard::LimPosInf,
	'\\lim\\limits_{x\\rightarrow-\\infty}', // LimitCard::LimNegInf,
	'\\lim\\limits_{x\\rightarrow0}', // LimitCard::Lim0,
	'\\liminf\\limits_{x\\rightarrow+\\infty}', // LimitCard::Liminf,
	'\\limsup\\limits_{x\\rightarrow+\\infty}', // LimitCard::Limsup,
];

const renderKatex = str => {
	let element = document.createElement('div');
	element.className = 'katex-item';

	katex.render(str, element, { throwOnError: false, displayMode: true });

	return element;
};

(async () => {
	let i = 0;
	for (let corner of [cornerElement, cornerFunction]) {
		let topLeft = renderKatex(corner);
		staging.appendChild(topLeft);
		topLeft.style.top = gutter * 0.75 + 'px';
		topLeft.style.left = gutter * 0.75 + 'px';
		topLeft.style.fontSize = 1.2 * scale + 'rem';

		let bottomRight = renderKatex(corner);
		staging.appendChild(bottomRight);
		bottomRight.style.top = playerHeight - gutter * 0.75 + 'px';
		bottomRight.style.left = playerWidth - gutter * 0.75 + 'px';
		bottomRight.style.fontSize = 1.2 * scale + 'rem';
		bottomRight.style.transform = 'translate(-50%, -50%) rotate(180deg)';

		await html2canvas(staging, {
			width: playerWidth,
			height: playerHeight,
		}).then(canvas => {
			textureContext.drawImage(canvas, (i % 6) * playerWidth, Math.floor(i / 6) * playerHeight);
		});

		topLeft.remove();
		bottomRight.remove();

		i++;
	}
	for (let card of cardsSmall) {
		let element = renderKatex(card);
		staging.appendChild(element);
		element.style.top = playerHeight / 2.0 + 'px';
		element.style.left = playerWidth / 2.0 + 'px';
		element.style.fontSize = 2.15 * scale + 'rem';

		await html2canvas(staging, {
			width: playerWidth,
			height: playerHeight,
		}).then(canvas => {
			textureContext.drawImage(canvas, (i % 6) * playerWidth, Math.floor(i / 6) * playerHeight);
		});

		element.remove();

		i++;
	}

	for (let card of cardsStandard) {
		let element = renderKatex(card);
		staging.appendChild(element);
		element.style.top = playerHeight / 2.0 + 'px';
		element.style.left = playerWidth / 2.0 + 'px';
		element.style.fontSize = 1.75 * scale + 'rem';

		await html2canvas(staging, {
			width: playerWidth,
			height: playerHeight,
		}).then(canvas => {
			textureContext.drawImage(canvas, (i % 6) * playerWidth, Math.floor(i / 6) * playerHeight);
		});

		element.remove();

		i++;
	}
})();
