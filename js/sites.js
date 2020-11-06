function search(ele) {
	if(event.key === 'Enter') {
        // github
		if( ele.value.includes("git") == true ){
			document.getElementById('search').value = '';
			window.open("https://github.com", '_blank');
        }
        // unsplash
		else if(
            ele.value.includes("uns") == true ){
			document.getElementById('search').value = '';
			window.open("https://unsplash.com", '_blank');
        }
        // chess
		else if(
            ele.value.includes("che") == true ){
			document.getElementById('search').value = '';
			window.open("https://chess.com", '_blank');
        }
        // reddit
		else if(
            ele.value.includes("red") == true ){
			document.getElementById('search').value = '';
			window.open("https://reddit.com", '_blank');
        }
        // search with duckduckgo
		else if(ele.value.includes("search") == true) {
			let doSearch = ele.value.replace("search", "");
			window.open("https://duckduckgo.com/?q=" + doSearch);
			document.getElementById('search').value = '';
		}
	}
}
