function func_on_collap(event) {
    var tag_content = this.parentElement.querySelector('.collap-content');
    var sum = this.querySelector('summary');
    if (tag_content.style.display === 'none') {
        tag_content.style.display = '';
        sum.style.listStyleType = 'disclosure-open';
    } else {
        tag_content.style.display = 'none';
        sum.style.listStyleType = 'disclosure-closed';
    }
}

function handle_collap() {
    var elements = document.getElementsByClassName('collap-header');
    for (let i = 0; i < elements.length; i++) {
        var ele = elements[i];
        ele.onclick = func_on_collap;
    }
}
