function getJson(from) {
  return fetch(from)
  .then((response) => response.json())
  .then((json) => { 
    return json;
  })
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

