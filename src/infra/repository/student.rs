use crate::api::primitive::students::StuNo;
use crate::api::res::PageRes;
use crate::ddd::core::{DomainModel, Id, PageQuery};
use crate::domain::cmd::student_cmd::StudentQuery;
use crate::domain::entity::student::Student;
use crate::domain::repo::student::StudentRepository;

use crate::ddd::repo::Repository;
use crate::infra::data::student::StudentDO;
use crate::infra::repository::DbRes;
use crate::pool;
use axum::extract::FromRef;
use rbatis::{Page, PageRequest, RBatis};


pub struct StudentRepositoryImpl<'a> {
    rb: &'a RBatis,
}


impl<'a> StudentRepositoryImpl<'a> {
    pub fn new() -> Self {
        Self {
            rb: pool!()
        }
    }
}

impl Repository<Id<i64>, Student> for StudentRepositoryImpl<'_> {
    async fn attach(&self, aggr: Student) {
        todo!()
    }

    async fn detach(&self, aggr: Student) {
        todo!()
    }

    async fn find(&self, id: Id<i64>) -> Result<Student, String> {
        let DbRes(res) = StudentDO::select_by_id(self.rb, id.0).await.into();
        match res {
            Ok(stu) => {
                match stu {
                    Some(student) => Student::new(&student),
                    None => Err("查询数据为空".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn save(&self, stu: Student) -> Result<(), String> {
        let data: StudentDO = stu.into();
        let res = StudentDO::insert(self.rb, &data).await;
        match res {
            Ok(_) => {
                Ok(())
            },
            Err(e) => Err(e.to_string())
        }
    }

    async fn remove(&self, aggr: Student) -> Result<(), String> {
        todo!()
    }
}

impl StudentRepository for StudentRepositoryImpl<'_> {
    async fn find_one(&self, stu_no: StuNo) -> Result<Student, String> {
        let DbRes(res)  = 
            StudentDO::select_by_stu_no(self.rb, stu_no.0).await.into();
        match res {
            Ok(stu) => {
                match stu {
                    Some(student) => Student::new(&student),
                    None => Err("查询数据为空".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn find_page(&self, page_query: PageQuery<StudentQuery>) -> Result<PageRes<Student>, String> {
        let page: PageRequest = PageRequest::from_ref(&page_query);
        let stu = page_query.query.map(|q| q.into());
        let DbRes(db_res) =  
            DbRes::<Page<StudentDO>>::from(StudentDO::select_page(self.rb, &page, &stu).await);
        match db_res { 
            Ok(res) => Ok(PageRes::<Student>::try_from(res)?),
            Err(e) => Err(e)
        }
    }

    async fn count(&self, query: StudentQuery) -> Result<u64, String> {
        todo!()
    }
}