this.blogs;

function getBlogs() {
  this.getJson("http://localhost:3030/blogs")
  .then((blogs) => {
    this.blogs = blogs;
    console.table(this.blogs);
  }).then(() => {
    for (let i = 0; i < this.blogs.length; i++) {
      this.appendElement("blogs_list", 
        `<li 
        onclick="goToQuiz('${this.blogs[i].title}')" 
        class="normal__list-item rounded_corners" 
        >
        ${this.blogs[i].title}
        </li>`
      );
    }
  });
}

function createBlog() {
  this.postJson("http://localhost:3030/blogs", { title: "create", content: "this is a test" })
}

this.getBlogs();
this.createBlog();

function getJson(from) {
  return fetch(from)
  .then((response) => response.json())
  .then((json) => { 
    return json;
  })
}

function postJson(url, object) {
 fetch(url, {
  method: "POST",
  body: JSON.stringify({
     title: "create", content: "this is a test" 
  }),
  headers: {
    "Content-type": "application/json; charset=UTF-8"
  }
}); 
}

function setElement(elementId, elementText) {
    const element = document.getElementById(elementId);

    if (element){
      element.innerHTML = elementText;
    } else {
      window.onload = () => {
        const element = document.getElementById(elementId);
        if(element) {
          element.innerHTML = elementText;
        }
      }
  }
}

function appendElement(elementId, elementText) {
    const element = document.getElementById(elementId);

    if (element){
      element.innerHTML += elementText;
    } else {
      window.onload = () => {
        const element = document.getElementById(elementId);
        if(element) {
          element.innerHTML += elementText;
        }
      }
  }
}

function changeBackgroundOfElement(elementId, color) {
     const element = document.getElementById(elementId);
     element.style.background = color;
}


function goToUrl(path) {
    window.location.assign(path);
}

function saveDataToStorage(key, value) {
  sessionStorage.setItem(key, value);
}

function getDataFromStorage(key) {
  return sessionStorage.getItem(key);
}

function removeDataFromStorage(key) {
  sessionStorage.removeItem(key);
}

function clearStorage() {
  sessionStorage.clear();
}

function shuffleArray(list) {
  return list.map(value => ({value, sort: Math.random()}))
  .sort((a, b) => a.sort - b.sort)
  .map(({ value }) => value)
}

