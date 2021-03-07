
function getValue() {  /* function to get the value in the input*/ 


    var usersName = document.getElementById("usersName").value;
    var language = document.getElementById("language").value;
    var nbMonths = document.getElementById("nbMonths").value;

    changeThings(usersName,language,nbMonths);  /*Call the function who will do the main work here.*/
}



function createURLpage(name,language){   /*  Creer le lien pour la page wikipedia qu'on souhaite */
        name2 = name.split(' ').join('_');
        var url = "https://"+language+".wikipedia.org/wiki/"+name2;
        var complet = "<a href="+url+ " class = "+ '"text-white">'+name+"</a>";
        
        return complet;
    }

function createRow(PagesEdit,DaysEdit,DeltaEdit,CategoriesEdit,language,i){  /* Creates the row off the tab */
    var URLpage = createURLpage(PagesEdit,language);
    /*document.getElementById("innerTab").innerHTML += "<div class='row'> <div class='col bg-info' >"+URLpage+"</div> <div class='col bg-info' >"+DaysEdit+"  </div> <div class='col bg-info' > "+DeltaEdit+" </div> <div class='col bg-info' > "+CategoriesEdit+" </div></div>"; */
   
    document.getElementById("innerTab").innerHTML += "<div class='row'> <div class='col-2 bg-info  border' >"+URLpage+"</div> <div class='col-2 bg-info  border' >"+DaysEdit+"  </div> <div class='col-1 bg-info  border' > "+DeltaEdit+" </div> <div class='col-6 bg-info  border' > <div class='row' id = "+((i+1)*1000+1)+" hidden = true >"+CategoriesEdit+" </div><div class= 'row'><button type='button' class ='btn btn-success mb-2' onclick= "+'" displayValue(+'+((i+1)*1000+1)+','+10+');"'+">See categories </button></div></div></div>";
}

function createRows(usersC,L,language,plateau,separation){/* Go through all the queries the users made (usersC), and display in a tab the differentes queries with relevant information  */ 
    a = 0;
    for (var u in usersC){  
        PagesEdit = usersC[u].title;
        DaysEdit = usersC[u].timestamp ;
        DeltaEdit = usersC[u].sizediff;
        CategoriesEdit = L[usersC[u].title];
        createRow(PagesEdit,DaysEdit,DeltaEdit,CategoriesEdit,language,u);
        }
        a+=1;
    }


function findPage(usersC,id){  /*Do exaustif search to find the query matching the id i*/

    L = [];
    for (var u in usersC){
        if (usersC[u].pageid == id){     
            return usersC[u];
        }
    }
    return L;
}

function fillPage(usersC,language,L,n,D,j,nbMainCat){/* Will display on screen the main information about the categories the users edit*/ 
    a = 0;
    
    for (i=0;i<n;i++){/*  In this example we sort by the number D[i][2]. As the list is assumed to be already sorted, we just go through it to display the first 10 */
        var idpage = D[i][0];               /*id of the page*/
        findPAGE = findPage(usersC,idpage);
        var PagesEdit = findPAGE.title;
        var DaysEdit = findPAGE.timestamp ;
        var DeltaEdit = findPAGE.sizediff;
        var CategoriesEdit = L[PagesEdit];
        var Numberofview = D[i][2];
        var urlPage = createURLpage(PagesEdit,language);
        createLineinCategories(j,urlPage,DeltaEdit,CategoriesEdit,DaysEdit,Numberofview,i,nbMainCat);
    }
}


function createTimestemps(date,D){ /* D est le nombre de mois */

    var timestamps = "2021-02-01T14:56:00Z";
    var m1 = parseInt(timestamps[6]);
    var m2 = parseInt(timestamps[5]);
    var m3 = parseInt(timestamps[3]);
    var m4 = parseInt(timestamps[2]);
    var L = [];
    for (i = 0; i <D; i++){
        if (m2 ==1) {
            if(m1 == 0) {
                m1 = 9;
                m2 = 0;
            }
            else {
                m1 = m1 -1;
            }
        }
        else {
            if(m1== 1){
                m1 = 2;
                m2 = 1;
                if (m3>0){
                    m3 = m3 -1;
                }
                else {
                    m3 = 9;
                    m4 = m4-1;
                }   
            }
            else {
                m1 = m1-1;
            }
        }
        timestamps2 = "20"+m4.toString() + m3.toString() + "-"+ m2.toString()+ m1.toString() +"-01T14:56:00Z";
        L.push(timestamps2); 
    }
    return L;

}


function createLineinCategories(i,URLpage,DeltaEdit,CategoriesEdit,DaysEdit,Numberofview,line,nbMainCat){

    
    var istring = i.toString();
    
    document.getElementById(istring).innerHTML += "<div class='row'> <div class='col-2 bg-info  border' >"+URLpage+"</div> <div class='col-2 bg-info  border' >"+DaysEdit+"  </div> <div class='col-1 bg-info  border' > "+DeltaEdit+" </div><div class='col-1 bg-info  border' > "+Numberofview+" </div> <div class='col-6 bg-info  border' > <div class='row' id = "+((line+1)*10+i)+" hidden = true >"+CategoriesEdit+" </div><div class= 'row'><button type='button' class ='btn btn-success mb-2' onclick= "+'" displayValue(+'+((line+1)*10+i)+','+10+');"'+">See categories </button></div></div></div>";
    
}

function createBlock(n,D){
    document.getElementById("titleInfo").innerHTML = "";
    document.getElementById("infoQueries").innerHTML = "";
    for (i=0;i<n;i++){
        if (i <n){
            document.getElementById("titleInfo").innerHTML += "<div class='col bg-info border' ><h3>"+D[i]+" </h3> <button type='button' class ='btn btn-success mb-2' onclick= "+'" displayValue(+'+i+','+n+');"'+">Display main queries </button>  </div>";
            document.getElementById("infoQueries").innerHTML += "<div class='col bg-info' hidden = true id =" +i+ "  ></div>"
        }
    }
}

 function displayValue(i,n){   /*enables the users to one value or anothe*/

    if (i ===parseInt(i,10)){
        var istring = i.toString();
    }
    else {
        var istring = i.id;
    }

    if (n < 10 ){

        for (j = 0; j <n; j++){

            if (j == i){
                if (document.getElementById(istring).hidden == false){
                    document.getElementById(istring).hidden = true;
                }
                else {
                    document.getElementById(istring).hidden = false;
                }
            } else
            document.getElementById(j).hidden = true;
        }

    }
    else{
        if (document.getElementById(istring).hidden == false){
            document.getElementById(istring).hidden = true;
            }
        else {
            document.getElementById(istring).hidden = false;
        }

    }
    
}



function getAverage(L,K){  /*computes the average*/
    x = 0;
    i = 0;
    for (key in K){
        x = x + L[K[key]];
        i++;
    }
    return Math.round(x /i*100)/100;
}


function createURLdifferentTime(lTimeStamp,language,usersName,Size){

    var L = [];
    for (var i=0; i<=Size;i++){
        var url = "https://"+ language+ ".wikipedia.org/w/api.php"; 
        var params = {
            action: "query",
            list: "usercontribs",
            ucuser: usersName,
            ucprop: "ids|title|comment|size|sizediff|timestamp|tags",
            format: "json",
            ucshow:"!minor",
            ucend:lTimeStamp[i+1],
            ucstart:lTimeStamp[i],
            uclimit:500,
            ucnamespace:"0"

        };
        url = url + "?origin=*";
        Object.keys(params).forEach(function(key){url += "&" + key + "=" + params[key];}); /*Constructing the url*/
        L.push(url)
    }
    return L;
}

function createTitles(usercontribs,L,borne){
    /* We create here the Titles for the different queries to the url. The list L contains the different part of userscontribs we have to use.
    If borne is not 0, it means that we are only interested in the pages of on single element of usercontribs, which have more than 50 element, and thus, that we have split in different part.*/

    var m = "";
    var compteur = 0;
    for (var i in L){
        for (var u in usercontribs[L[i]]){
            if (compteur <50 && (parseInt(u)+borne)<usercontribs[L[i]].length){
               m += "|"+usercontribs[L[i]][parseInt(u)+borne].title;
               compteur += 1; 
            }
        }
    }
    m = m.substring(1,m.length);
    return m;

}

function createURLCategories(titles,language){

    var L = [];
    
    var url = "https://"+ language+ ".wikipedia.org/w/api.php"; 
    var params = {
            action: "query",
            format: "json",
            prop: "categories|pageviews|links|linkshere",
            clshow: "!hidden",
            cllimit:500,
            titles: titles,
            pllimit : 500
        };
    url = url + "?origin=*";
    Object.keys(params).forEach(function(key){url += "&" + key + "=" + params[key];}); /*Constructing the url*/
    L.push(url)
    return L;
}


function getClass(n,L,separation){

    var b = 0;
    for ( var i in L){
        if (n < L[separation-1-i]){
            b = separation-1-i
        }
    }
    return b;
}


function getURLindexpage(language,L){

    /* pour chaque nom de L, on appel le server pour avoir l'id*/
    var listURL = [];
    for (var u in L){

        var url = "https://"+ language+ ".wikipedia.org/w/api.php"; 
                var params = {
                        action: "query",
                        format: "json",
                        prop: "info",
                        titles:L[u]
                        
                    };
                url = url + "?origin=*";
                Object.keys(params).forEach(function(key){url += "&" + key + "=" + params[key];}); /*Constructing the url*/
                listURL .push(url)
                


    }
    return listURL ;
    
}
