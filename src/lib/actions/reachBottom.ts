export function reachingBottom(node: HTMLElement) {
	let reachedBottom = false;

	const handleScroll = () => {
		const isEnd = node.scrollTop + node.offsetHeight >= node.scrollHeight;
		if (!reachedBottom && isEnd) {
			node.dispatchEvent(new CustomEvent('reachingbottom'));
		}
		reachedBottom = isEnd;
	};

	node.addEventListener('scroll', handleScroll);

	return {
		destroy() {
			node.removeEventListener('scroll', handleScroll);
		}
	};
}
