
const area = document.getElementById("area");
const add = document.getElementById("add");
const text = document.getElementById("text");
const arr = [];

//添加
add.addEventListener('click', (e) => {
    e.preventDefault();
    if (text.value) {
        const obj = {
            personid: arr.length + 1,
            username: "和光予序",
            content: text.value
        }
        arr.push(obj);
        render();
        text.value = '';
    }
})
//回车输入，但是有bug
// text.onkeyup = (e) => {
//     if (e.keycode == 13) {
//         e.preventDefault();
//         if (text.value) {
//             const obj = {
//                 personid: arr.length + 1,
//                 username: "和光予序",
//                 content: text.value
//             }
//             arr.push(obj);
//             render();
// text.innerHTML = '';
//         }
//     }
// }

//删除
area.addEventListener('click', (e) => {
    if (e.target.value) {
        for (let i = e.target.id - 1; i < arr.length; i++) {
            arr[i] = arr[i + 1];
        }
        arr.pop();
        render();
    }
})

//渲染
function render() {
    area.innerHTML = '';
    for (let i = 0; i < arr.length; i++) {
        const tr = document.createElement("tr");
        tr.innerHTML = `
            <td class="td">
                <img src="../Img/莉莉丝.jpg" alt="userimg" class="userimg">
                <span class="username">${arr[i].username}</span>
                <p class="content">${arr[i].content}</p>
                <div class="delete">
                    <ul>
                        <li>...</li>
                        <li name="delete" id="${arr[i].personid}" value="1">删除</li>
                    </ul>
                </div>
            </td>
        `
        area.appendChild(tr);
    }
}