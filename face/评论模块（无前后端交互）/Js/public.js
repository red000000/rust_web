//基本变量
const area = document.getElementById("area");
const add = document.getElementById("add");
const text = document.getElementById("text");
const form = document.getElementById("form");
const arr = []; //存储消息对象

//添加
add.addEventListener('click', (e) => {
    e.preventDefault();
    if (text.value) {
        // 读取、设置消息对象
        const obj = {
            personid: arr.length + 1,
            username: "和光予序",
            content: text.value
        }
        arr.push(obj); // 向数组末尾添加对象
        render(); // 再次渲染
        text.value = ''; // 清空输入框
    }
})

form.addEventListener('submit', (e) => {
    e.preventDefault();

    if (text.value) { // 检查输入框的值是否为空
        const obj = {
            personid: arr.length + 1,
            username: "和光予序",
            content: text.value
        }
        arr.push(obj);
        render();
        text.value = ''; // 清空输入框
    }
})

//删除
area.addEventListener('click', (e) => {
    if (e.target.value) {
        // 将需要删除的对象替换为下一个对象
        for (let i = e.target.id - 1; i < arr.length; i++) {
            arr[i] = arr[i + 1];
        }
        arr.pop(); // 删除数组末尾对象
        for (let j = 0; j < arr.length; j++) {
            arr[j].personid = j + 1;
        }
        render(); // 再次渲染
    }
})

//渲染
function render() {
    area.innerHTML = ''; // 渲染前清空
    for (let i = 0; i < arr.length; i++) {
        const tr = document.createElement("tr");
        // 渲染对象的格式
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
        // 渲染对象
        area.appendChild(tr);
    }
}