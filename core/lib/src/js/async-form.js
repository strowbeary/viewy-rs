window.addEventListener("load", () => {
   const forms = document.querySelectorAll("form[data-async]");
   forms.forEach(form => {
       console.log(form);
       form.addEventListener('submit', e => {
           e.preventDefault();
           const formData = new FormData(form);
           const jsonFormData = {};
           formData.forEach((value, key) => {
               // Reflect.has in favor of: object.hasOwnProperty(key)
               if(!Reflect.has(jsonFormData, key)){
                   jsonFormData[key] = value;
                   return;
               }
               if(!Array.isArray(jsonFormData[key])){
                   jsonFormData[key] = [jsonFormData[key]];
               }
               jsonFormData[key].push(value);
           });
           fetch(form.getAttribute("action"), {
               method: "POST",
               headers: {
                   "Content-Type": "application/json",
               },
               body: JSON.stringify(jsonFormData)
           })
               .then()
       });
   })
});