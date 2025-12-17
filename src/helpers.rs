use crate::structs;
use crate::structs::Fleet;
use gtfs_realtime::FeedEntity;

// A helper function that takes a FeedEntity and splits it into a Vec of Vecs of all vehicles
// separated by their type according to the Fleet enum. 
pub fn get_collection(entities: Vec<FeedEntity>) -> Vec<structs::FeedCategory> {
    let mut category: Vec<structs::FeedCategory> = Vec::new();

    let partition1: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(entities, 
        Fleet::NFIDE60LF2008);
    let nfide60lf2008 = partition1.0;
    category.push(nfide60lf2008);
    let partition2: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition1.1,
        Fleet::NFIDE60LF2009);
    let nfide60lf2009 = partition2.0;
    category.push(nfide60lf2009);
    let partition3: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition2.1,
        Fleet::NFIDE60LFR2010);
    let nfide60lfr2010 = partition3.0;
    category.push(nfide60lfr2010);
    let partition4: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition3.1,
        Fleet::NFIDE60LFR2011);   
    let nfide60lfr2011 = partition4.0;
    category.push(nfide60lfr2011);
    let partition5: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition4.1,
        Fleet::NFIDE60LFR2012);  
    let nfide60lfr2012 = partition5.0;
    category.push(nfide60lfr2012);
    let partition6: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition5.1,
        Fleet::NFIDE60LFR2013);  
    let nfide60lfr2013 = partition6.0;
    category.push(nfide60lfr2013);
    let partition7: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition6.1,
        Fleet::NFIDE60LFA2009); 
    let nfide60lfa2009 = partition7.0;
    category.push(nfide60lfa2009);
    let partition8: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition7.1,
        Fleet::NFIXDE352014); 
    let nfixde352014 = partition8.0;
    category.push(nfixde352014);
    let partition9: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition8.1,
        Fleet::NFIXE402021); 
    let nfixe402021 = partition9.0;
    category.push(nfixe402021);
    let partition10: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition9.1,
        Fleet::NFIXE602021); 
    let nfixe602021 = partition10.0;
    category.push(nfixe602021);
    let partition11: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition10.1,
        Fleet::NFIXDE602015); 
    let nfixde602015 = partition11.0;
    category.push(nfixde602015);
    let partition12: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition11.1,
        Fleet::NFIXDE602018); 
    let nfixde602018 = partition12.0;
    category.push(nfixde602018);
    let partition13: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition12.1,
        Fleet::NFIXDE602019); 
    let nfixde602019 = partition13.0;
    category.push(nfixde602019);
    let partition14: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition13.1,
        Fleet::NFIXDE602023); 
    let nfixde602023 = partition14.0;
    category.push(nfixde602023);
    let partition15: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition14.1,
        Fleet::NFIXDE602014); 
    let nfixde602014 = partition15.0;
    category.push(nfixde602014);
    let partition16: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition15.1,
        Fleet::OBIVII2010); 
    let obivii2010 = partition16.0;
    category.push(obivii2010);
    let partition17: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition16.1,
        Fleet::NFIXDE602016); 
    let nfixde602016 = partition17.0;
    category.push(nfixde602016);
    let partition18: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition17.1,
        Fleet::NFIXDE602017);
    let nfixde602017 = partition18.0;
    category.push(nfixde602017);
    let partition19: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition18.1,
        Fleet::NFIXDE402017); 
    let nfixde402017 = partition19.0;
    category.push(nfixde402017);
    let partition20: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition19.1,
        Fleet::NFIXT402014); 
    let nfixt402014 = partition20.0;
    category.push(nfixt402014);
    let partition21: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition20.1,
        Fleet::NFIXT602015);    
    let nfixt602015 = partition21.0;
    category.push(nfixt602015);
    let partition22: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition21.1,
        Fleet::OBIVII2011);   
    let obivii2011 = partition22.0;
    category.push(obivii2011);
    let partition23: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition22.1,
        Fleet::GILLIGHEV402017);    
    let gillighev402017 = partition23.0;
    category.push(gillighev402017);
    let partition24: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition23.1,
        Fleet::GILLIGHEV402018);    
    let gillighev402018 = partition24.0;
    category.push(gillighev402018);
    let partition25: (structs::FeedCategory, Vec<FeedEntity>) = fleet_helper(partition24.1,
        Fleet::GILLIGHEV402019);    
    let gillighev402019 = partition25.0;
    category.push(gillighev402019);

    category
}

// Helper function, which performs the partitioning done in fleet_handler(). 
pub fn fleet_helper(vector: Vec<FeedEntity>, enumeration: Fleet) -> (structs::FeedCategory, Vec<FeedEntity>){
        let ret: (Vec<FeedEntity>, Vec<FeedEntity>) = vector.into_iter().partition(|x| {
            Fleet::which(
                x.vehicle
                    .as_ref()
                    .unwrap()
                    .vehicle
                    .as_ref()
                    .unwrap()
                    .id
                    .as_ref()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            ) == enumeration 
        });

        let string = match enumeration {
            Fleet::NFIXDE352014 => "nfixde352014".to_string(),
            Fleet::NFIXE402021 => "nfixe402021".to_string(),
            Fleet::NFIXE602021 => "nfixe602021".to_string(),
            Fleet::NFIDE60LFA2009 => "nfide60lfa2009".to_string(),
            Fleet::NFIDE60LFR2011 => "nfide60lfr2011".to_string(),
            Fleet::NFIDE60LFR2013 => "nfide60lfr2013".to_string(),
            Fleet::NFIXDE602015 => "nfixde602015".to_string(),
            Fleet::NFIXDE602018 => "nfixde602018".to_string(),
            Fleet::NFIXDE602019 => "nfixde602019".to_string(),
            Fleet::NFIXDE602023 => "nfixde602013".to_string(),
            Fleet::NFIDE60LF2008 => "nfide60lf2008".to_string(),
            Fleet::NFIDE60LF2009 => "nfide60lf2009".to_string(),
            Fleet::NFIDE60LFR2010 => "nfide60lfr2010".to_string(),
            Fleet::NFIDE60LFR2012 => "nfide60lfr2012".to_string(),
            Fleet::OBIVII2010 => "obivii2010".to_string(),
            Fleet::OBIVII2011 => "obivii2011".to_string(),
            Fleet::NFIXDE402014 => "nfixde402014".to_string(),
            Fleet::GILLIGHEV402017 => "gillighev402017".to_string(),
            Fleet::GILLIGHEV402018 => "gillighev402018".to_string(),
            Fleet::GILLIGHEV402019 => "gillighev402019".to_string(),
            Fleet::NFIXDE602014 => "nfixde602014".to_string(),
            Fleet::NFIXDE602016 => "nfixde602016".to_string(),
            Fleet::NFIXDE602017 => "nfixde602017".to_string(),
            Fleet::NFIXDE402017 => "nfixde402017".to_string(),
            Fleet::NFIXT402014 => "nfixt402014".to_string(),
            Fleet::NFIXT602015 => "nfixt602015".to_string(),
            Fleet::NoneFleet => "".to_string()
        };
        
        let ret: (structs::FeedCategory, Vec<FeedEntity>) = (structs::FeedCategory { list: ret.0, name: string}, ret.1);
        ret

}


pub fn collection_split_html(vector: Vec<FeedEntity>) {
   
}
