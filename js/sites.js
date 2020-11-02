var pr = {
	0: "r",
	1: "re",
	2: 'red', 
	3: 'reddit'
};

var pg = {
	0: "g",
	1: "gi",
	2: "git",
	3: "github"
};

var pu = {
	0: "u",
	1: "un",
	2: "uns",
	3: "unsplash"
}

var pc = {
	0: "c",
	1: "ch",
	2: "ches",
	3: "chess"
}

function search(ele) {
	if(event.key === 'Enter') {
		if( 
			ele.value == pg[0] ||
			ele.value == pg[1] ||
			ele.value == pg[2] ||
			ele.value == pg[3] 
		){
			document.getElementById('search').value = '';
			window.open("https://github.com", '_blank'); } 
		else if(
			ele.value == pu[0] ||
			ele.value == pu[1] ||
			ele.value == pu[2] ||
			ele.value == pu[3] 
		){
			document.getElementById('search').value = '';
			window.open("https://unsplash.com", '_blank'); } 
		else if(
			ele.value == pc[0] ||
			ele.value == pc[1] ||
			ele.value == pc[2] ||
			ele.value == pc[3] 
		){ 
			document.getElementById('search').value = '';
			window.open("https://chess.com", '_blank'); } 
		else if(
			ele.value == pr[0] ||
			ele.value == pr[1] ||
			ele.value == pr[2] ||
			ele.value == pr[3] 
		){
			document.getElementById('search').value = '';
			window.open("https://reddit.com", '_blank'); }
	}
}

function getTime() {
	let date = new Date(),
        min = date.getMinutes(),
        hour = date.getHours();
    
        return '' + 
        	(hour < 10 ? ('0' + hour) : hour) + ':' + 
                (min < 10 ? ('0' + min) : min);
}

window.onload = () => {
	document.getElementById('clock').innerHTML = getTime();
        		
        setInterval( () => {
        	document.getElementById('clock').innerHTML = getTime();
        },100);
}

