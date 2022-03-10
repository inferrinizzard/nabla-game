export const remToPx = rem => {
	return parseFloat(rem) * parseFloat(getComputedStyle(document.documentElement).fontSize);
};
