




function changeThings(usersName,language,nbMonths){ /*To update with the value that we get from the input*/

    /* We construct the first url , to get the info about the user*/

    /*document.getElementById("name").innerHTML ="<strong>Users Name </strong>";
    document.getElementById("groups").innerHTML ="<strong>User's groups </strong> ";
    document.getElementById("editcount").innerHTML =" <strong>Edits count </strong> ";
    document.getElementById("registration").innerHTML ="<strong> Registration date </strong>";
    document.getElementById("emailable").innerHTML ="<strong> Email, if informed </strong> ";
*/
    document.getElementById("Graphic").innerHTML = '<div id="container"  style ="height: 400px;"></div>';
    /*document.getElementById("container")*/
    document.getElementById("Loading").hidden = false;
    var url = "https://"+ language+ ".wikipedia.org/w/api.php"; 
    var params = {
        action: "query",
        list: "users",
        ususers: usersName,
        usprop: "blockinfo|groups|editcount|registration|emailable|gender",
        format: "json"
    };

    url = url + "?origin=*";
    Object.keys(params).forEach(function(key){url += "&" + key + "=" + params[key];});   
    fetch(url)
        .then(function(response){return response.json();})
        .then(function(response) {
            /*Define all the variables usefull to show after*/
            console.log(response)
            var users = response.query.users;
            var xname = users[0].name;
            var xblockinfo = users[0].blockinfo;
            var xgroups = users[0].groups;
            var xeditcount = users[0].editcount;
            var xregistration = users[0].registration;
            var xemailable = users[0].emailable;
            var xgender = users[0].gender;
            /*Show the different value*/
            console.log(xname);
            document.getElementById("name").innerHTML =xname;
            /*document.getElementById("blockinfo").innerHTML =+xblockinfo;*/
            document.getElementById("groups").innerHTML =xgroups;
            document.getElementById("editcount").innerHTML =xeditcount;
            document.getElementById("registration").innerHTML =xregistration;
            document.getElementById("emailable").innerHTML =xemailable;

        })

    /*Second request to get the different queries of that user*/
    /* On va essayer de request le plus de queries possible, en utilisant different fetch. Ça va être assez sympa mais compliqué.
    attention les yeux*/

    /* on va commencer par une liste d'URL je pense.*/

    var listURLuserQueries = [];
    /* Le probleme c'est qu'on ne sait pas forcément comment découper en fonction du temps. Néamoins on peut se donner quelque chose d'assez simple. on va esasyer de lui demander de nous donner tout les queries qu,il a fait dans un mois à chaque fois.  Avec un peu de chance il n'aura pas fait plus de 500 queries dans le mois. 
    On n'a plus qu'a après balayer quelque choses genre 5 ans */
    timestampEx = "2021-03-01T14:56:00Z"

    
    var numberMonths = nbMonths;  /* Nombre de mois donné par l'utilisateur*/
    var lTimeStamp = createTimestemps("0000",numberMonths+1);
    listURLuserQueries = createURLdifferentTime(lTimeStamp,language,usersName,numberMonths-1);
    Promise.all(listURLuserQueries.map(url =>
        fetch(url).then(resp => resp.json())
    )).then(resp => {

         document.getElementById("tab").innerHTML = "<div class='row'><button type='button' class ='btn btn-success mb-2' onclick= "+'" displayValue(innerTab);"'+">Display all the last queries of the users </button> </div>";
        document.getElementById("tab").innerHTML += "<div class='row' > <div class='col-2 bg-primary' > <strong>Page</strong> </div> <div class='col-2 bg-primary' > <strong>Jour de l'édit </strong> </div> <div class='col-1 bg-primary' ><strong> DeltaSize </strong> </div> <div class='col-7 bg-primary' > <strong>Categories</strong>  </div></div><div class = 'container' id ='innerTab' hidden = true></div>";
        /*On va maintenant compter combien de queries on a*/
        var usercontribs = {};
        var NumberQueries = 0;
        var Partition = [];
        var V = [];
        var countInter = 0;
        var plateau = [];
        var usersC = [];
        var number = 0;
        var actionByMonths = [];
        var data = {}
        var actionByMonths2 = [];
        /* In this part I group and divide the different pages in blocks to be send to the server. Each of them have to be of a size < 50 to be accepted. */

        for (i=0;i<numberMonths;i++){
            K = resp[i].query.usercontribs;
            usercontribs[i] = K;
            var temp = {};

            actionByMonths.push([lTimeStamp[i].substring(0,10), K.length])
            /*temp["x"] = lTimeStamp[i].substring(0,10);
            temp["y"] =  K.length;
            actionByMonths.push(temp);*/
            actionByMonths2.push(K.length)
;                for (var u in K){

                usersC[number] = K[u];
                number+=1;
            }

            if (K.length > 50){
                
                Partition.push(V);
                V = new Array();
                plateau.push(NumberQueries);
                var repeat = Math.floor(K.length/50);
                
                for (j =0; j<repeat;j++){
                
                    V.push(i);
                    Partition.push(V);
                    V = new Array();
                    NumberQueries += 50;
                    plateau.push(NumberQueries)
                }
                V.push(i);
                Partition.push(V);
                V = new Array();
                NumberQueries += (K.length-50*(repeat-1));
                plateau.push(NumberQueries);
                countInter = 0;
            }
            else {
            
                NumberQueries += K.length;
                countInter = countInter + K.length;
                if (countInter > 50  ){
                    countInter = K.length;
                    Partition.push(V);
                    V = [];
                    plateau.push(NumberQueries);
                    

                }
                V.push(i);
               
                
            }
        }
        Partition.push(V);
        plateau.push(NumberQueries);
        var separation = Partition .length;


        console.log(actionByMonths2)

        /*var data = [
                {x: 'Amazon', y: 120},
                {x: 'DZone', y: 60},
                {x: 'Gizmodo', y: 30},
                {x: 'StackOverFlow', y: 80},
                {x: 'CNET', y: 50}
            ];
        */
        var chart = anychart.column();
        chart.title("Evolution of the quantity of modification made by the users");
        chart.xAxis().title("Months");
        chart.yAxis().title("Nomber of modification");
        var series = chart.column(actionByMonths);
        chart.container("container");
        chart.draw();
      

        console.log(actionByMonths);

        var usersC = usersC.sort(function(a,b){   /*We sort the contribution by the absolute value of delta size. */
                if (Math.abs(a.sizediff) < Math.abs(b.sizediff) ){ return 1; } 
                if (Math.abs(a.sizediff) > Math.abs(b.sizediff) ){ return -1;}
                return 0; 
            })

        

        /* Now we desgin the url to get the categories associated.*/


        listURLCategories = [];
        var borne = 0;
        for (var e in Partition ){
            if (Partition[e-1] == Partition[e]){
                borne += 1;
            }
            else {
                borne = 0;
            }
            var title = createTitles(usercontribs,Partition[e],borne*50);   /* Each Url will get a different "Title", wich is the titles of the different pages we saerch. There are at most 50 pages in the block.*/
            var Url = createURLCategories(title,language);
            listURLCategories.push(Url);
        }

        console.log(listURLCategories);


        Promise.all(listURLCategories.map(url =>  /* We fetch all the url */
            fetch(url).then(resp2 => resp2.json())
        )).then(resp2 => {

            
            var L = {};
            var pageViews = [];
            var links = [];
            var linkshere = [];

            var jsonServeur = [];
            var group ={};
            var correspondance = []
            
            /*var dictLengthCategoriesDifferentLanguage = {

                als : 10;
                br : 7;
                en : 9;
                it : 10;
                oc:10;
                pcd: 10;
                pt: 10;
                fr:10;
                eo: 10;
                de:10
            }*/
            console.log(resp2);

            for (var i= 0;i<separation; i++){  /* For each fetchs… there is 10 fetchs*/ 
                
                var pages = resp2[i].query.pages;
                for (var p in pages) {
                    let categ = "";  /*Variable that will contains all the categories of these page*/
                    try {
                        for (var cat of pages[p].categories) {


                            var index = cat.title.indexOf(":");
                            var mot = cat.title.substring(index+1,cat.title.length);
                            categ += mot + " </br>" ; /* We add the categories*/

                            correspondance.push(cat.title);
                            blockCategories.push(p);

                        }
                        L[pages[p].title]=categ;
                    } catch(TypeError) {
                        categ = "";
                    }
                    try {
                        key = Object.keys(pages[p].pageviews);
                        LpageV = pages[p].pageviews;
                        var A = getAverage(LpageV,key);
                        pageViews.push([p,pages[p].title,A]);
                        links.push([pages[p].title,pages[p].links]);
                        linkshere.push([pages[p].title,pages[p].linkshere]);
                    }

                    catch(TypeError){
                        var doNothing = 0;
                        
                    }                                    
                }
            }

            
            URLindexListe = getURLindexpage(language,correspondance);
            console.log(URLindexListe);


            Promise.all(URLindexListe.map(url =>  /* We fetch all the url */
                fetch(url).then(resp3 => resp3.json())
            )).then(resp3 => {

                    console.log("YOUUUUUUUUU");
                    console.log(resp3);

                    correspondanceIdNom = {}
                    var blockCategories = [];


                    for (var u in resp3){

                        page = resp3[u].query.pages
                        for (var i in page){
                            correspondanceIdNom[page[i].title]=page[i].pageid;
                            blockCategories.push(page[i].pageid);
                        }
                    }

                    console.log(correspondanceIdNom);


                    group["categories"] = blockCategories;
                    group["weight"] = 1;  /* By default. For now*/
                    jsonServeur.push(group);
                    console.log(group);
                    console.log(correspondance);
                    





                    /* ON APPEL LE SERVER ET ON REÇOIT NOTRE DEMANDE */

                    console.log(jsonServeur);

                    var limit = 3;
                    
                    var xhr = new XMLHttpRequest();
                    xhr.open("POST", 'http://scaling-potatoes.ml/api/'+language+'/category?limit='+limit, true);

                    //Envoie les informations du header adaptées avec la requête
                    xhr.setRequestHeader("Content-Type", "application/json");

                    xhr.onreadystatechange = function() { //Appelle une fonction au changement d'état.
                        if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {


                            console.log("WHOA");
                    // Requête finie, traitement ici.
                        }
                    }
                    xhr.send(jsonServeur);
                    // xhr.send(new Int8Array());
                    // xhr.send(document);
                        
















                    var sortedpageViews = pageViews.sort(function(a,b){   /*We sort the contribution by the absolute value of delta size. */
                        if (a[2] < b[2] ){ return 1; } 
                        if (b[2] < a[2] ) { return -1;}
                        return 0; 
                    })

                    createRows(usersC,L,language,plateau);
                    
                    L1 = sortedpageViews.slice(0,10);
                    L2 = sortedpageViews.slice(10,20);
                    L3 = sortedpageViews.slice(20,30);
                    LCat = [L1,L2,L3];

                    nbMainCat = 3;  /* Maincat number between 2 and 5 ? */
                    Cat = ["Mathématique","Informatique","Politque"];
                    createBlock(nbMainCat,Cat);
                    document.getElementById("Titre").innerHTML = "<h1>Principale topics the users edits : </h1>";
                    for (y=0; y<nbMainCat;y++){
                        fillPage(usersC,language,L,10,LCat[y],y,nbMainCat);
                    }
                    document.getElementById("Loading").hidden = true;



                })
        })

    })
    
}

