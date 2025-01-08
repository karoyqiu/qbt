use futures::FutureExt;
use ormlite::{
  sqlite::{Sqlite, SqliteConnection, SqliteRow},
  BoxFuture, FromRow, Model, TableMeta,
};

use crate::error::{Error, IntoResult, Result};

/// 数据库状态接口
pub trait DbStateTrait {
  /// 获取数据库连接
  fn connection(&mut self) -> &mut Option<SqliteConnection>;

  /// 直接进行数据库操作
  async fn db<F, R>(&mut self, operation: F) -> R
  where
    F: for<'r> FnOnce(&'r mut SqliteConnection) -> BoxFuture<'r, R>,
  {
    let db = self.connection().as_mut().expect("Database not intialized");

    operation(db).await
  }

  // /// 按主键查询数据
  // async fn query_one<T, K>(&mut self, id: K) -> Result<T>
  // where
  //   T: Model<Sqlite> + for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin + 'static,
  //   K: Send
  //     + ormlite::types::Type<ormlite::sqlite::Sqlite>
  //     + for<'k> ormlite::Encode<'k, ormlite::sqlite::Sqlite>
  //     + 'static,
  // {
  //   self
  //     .db(|db| async move { T::select().where_("id = ?").bind(id).fetch_one(db).await }.boxed())
  //     .await
  //     .into_result()
  // }

  // /// 查询指定类型的所有数据
  // async fn query<T>(&mut self) -> Result<Vec<T>>
  // where
  //   T: Model<Sqlite> + for<'r> FromRow<'r, SqliteRow> + Send + Sync + Unpin + 'static,
  // {
  //   self
  //     .db(|db| {
  //       async move {
  //         let items = T::select().fetch_all(db).await.into_result()?;
  //         Ok::<Vec<T>, Error>(items)
  //       }
  //       .boxed()
  //     })
  //     .await
  // }

  // /// 更新
  // async fn update<T>(&mut self, doc: T) -> Result<()>
  // where
  //   T: Model<Sqlite> + TableMeta + Send + 'static,
  // {
  //   self
  //     .db(|db| {
  //       async move {
  //         doc.update_all_fields(db).await.into_result()?;

  //         Ok::<(), Error>(())
  //       }
  //       .boxed()
  //     })
  //     .await

  //   // // 通知数据库变动
  //   // notify_change::<T>(&app);
  // }

  // /// 按主键删除一个
  // async fn remove_one<T, K>(&mut self, id: K) -> Result<()>
  // where
  //   T: Model<Sqlite> + TableMeta + Send + 'static,
  //   K: Send
  //     + ormlite::types::Type<ormlite::sqlite::Sqlite>
  //     + for<'k> ormlite::Encode<'k, ormlite::sqlite::Sqlite>
  //     + 'static,
  // {
  //   self
  //     .db(|db| {
  //       async move {
  //         let sql = format!("DELETE FROM {} WHERE id = ?", T::table_name());
  //         ormlite::query(sql.as_str())
  //           .bind(id)
  //           .fetch_optional(db)
  //           .await
  //           .into_result()?;

  //         Ok::<(), Error>(())
  //       }
  //       .boxed()
  //     })
  //     .await
  // }
}
