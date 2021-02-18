// #[test]
// fn parallel() -> () {
// 	let categories = Vec::from_par_iter(
// 		Extractor::extract_par_iter_file::<Category>(
// 			File::open("test_samples/nrm/nrmwiki-20210201-page.sql")
// 				.expect("Something went wrong reading the file category")));
// 	let catcat = CategoryCategoryVec::from_par_iter(
// 		Extractor::extract_par_iter_file::<CategoryCategorySql>(
// 			File::open("test_samples/nrm/nrmwiki-20210201-categorylinks.sql")
// 				.expect("Something went wrong reading the file categorylinks")));
//
// 	let v =
// 		Vec::from_iter(merge_categories_links_triplets(&categories, &catcat));
//
// 	// println!("assert_eq!({},v.len());",v.len());
// 	// v[5..13].iter().for_each(|nzc|println!("{},",nzc.serialize()))
//
// 	// for i in 5..15 {
// 	// 	println!("assert_eq!(v[{}],{});", i, v[i].serialize())
// 	// }
//
// 	assert_eq!(145, v.len());
// 	assert_eq!(v[5], NonZeroCoeff::new(10160, 2171, 6));
// 	assert_eq!(v[6], NonZeroCoeff::new(10252, 2171, 6));
// 	assert_eq!(v[7], NonZeroCoeff::new(10340, 2171, 6));
// 	assert_eq!(v[8], NonZeroCoeff::new(6187, 2456, 1));
// 	assert_eq!(v[9], NonZeroCoeff::new(3133, 3075, 1));
// 	assert_eq!(v[10], NonZeroCoeff::new(3721, 3082, 10));
// 	assert_eq!(v[11], NonZeroCoeff::new(6106, 3082, 10));
// 	assert_eq!(v[12], NonZeroCoeff::new(6107, 3082, 10));
// 	assert_eq!(v[13], NonZeroCoeff::new(6108, 3082, 10));
// 	assert_eq!(v[14], NonZeroCoeff::new(6109, 3082, 10));
// }
