use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    execute_query("index.html", Some(&3), None, None, None,
    HashMap::from([("brand", Some("pureflix")), ("env", Some("stg"))]));
}

fn execute_query(index: &str, n_recommendations: Option<&u8>, fields_to_return: Option<Vec<&str>>,
                     sort: Option<Vec<&str>>, blacklist: Option<Vec<&str>>,
                     where_clauses: HashMap<&str,Option<&str>>) -> Vec<String> {

    let fields_to_return: Option<Vec<&str>> = match fields_to_return {
        None => Some(vec!["contentId", "rawCatalogue", "rank"]),
        Some(fields) => {
            if fields == vec!["all"] {None} else { Some(fields) }
        }
    };

    let sort = sort.unwrap_or(vec!["rank:asc"]);

    let where_clauses: HashMap<&str,&str> = where_clauses.iter()
        .filter(|tuple| tuple.1.is_some())
        .map(|tuple| (*tuple.0, tuple.1.unwrap()))
        .collect();

    let filter_statement: Vec<HashMap<&str, HashMap<&str, &str>>> = where_clauses.iter()
        .map(|tuple| HashMap::from([("match", HashMap::from([(*tuple.0, *tuple.1)]))]))
        .collect();

    // if self.blacklist or extra_blacklist:
    //     blacklist = tuple(set(self.blacklist + extra_blacklist)) if extra_blacklist else tuple(self.blacklist)
    let must_not_statement = HashMap::from([("terms", HashMap::from([("contentId", blacklist)]))]);

    // let query = HashMap::from([("bool", HashMap::from([("must", filter_statement),
    //     ("must_not", must_not_statement)]))]);
    // Lo dejo así a falta de mirar cómo se consulta ES de verdad
    let query = HashMap::from([("bool", HashMap::from([("must", filter_statement)]))]);

    println!("Query ==> {:?}", query);


    // recommendations = self._query(index=index,
    //                               query=query,
    //                               n_recommendations=n_recommendations,
    //                               sort=sort,
    //                               fields_to_return=fields_to_return)
    let recommendations = vec![String::from("Avatar"), String::from("FF Advent Children"), String::from("A realm reborn")];

    // if recommendations.len() > 0 {
    //     let recommendations = recommendations.get("hits").get("hits").iter()
    //         .map(|recommendation| recommendation.get("_source")).collect()
    // }

    println!("recommendations ==> {:?}", recommendations);
    recommendations
    }



