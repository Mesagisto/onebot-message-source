#[tokio::main]
async fn main() {
  let mut nb = nonebot_rs::Nonebot::new(); // 新建 Nonebot

  let mut matchers = nonebot_rs::Matchers::new_empty(); // 新建空 Matchers Plugin
  matchers
      .add_message_matcher(nonebot_rs::builtin::echo::echo())  // 注册 echo Matcher
      .add_message_matcher(nonebot_rs::builtin::rcnb::rcnb()); // 注册 rcnb Matcher
  // nb.add_plugin(scheduler); // 添加 Plugin

  nb.async_run().await; // 运行 Nonebot
}
