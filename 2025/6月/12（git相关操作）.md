今天弄了半天，基本是弄明白了，这里整理一下工作流程

首先从主线仓库fork一个到自己的账号，然后clone到本地

如果已经有了一个fork，注意更新

```rust
git remote add upstream(这个是用来标识上游仓库的，个人习惯是把主线记为upstream，然后要拉取的下游仓库记为oscomp之类的) https://github.com/original-owner/original-repo.git

git remote -v(查看远程仓库)

上述两条命令是第一次在本地添加上游仓库才用，之后可以不输入

git fetch upstream(拉取上游仓库的内容)
git checkout main(切换到自己的分支)
git merge upstream/main
```
上面是拉取上游仓库的内容，然后合并到自己的仓库中

然后是单独拉取下游仓库单独测试的步骤
```rust
git checkout main

git checkout -b test-pr19-only（创建一个新的分支用于测试）

git remote add oscomp https://github.com/oscomp/arceos
git fetch oscomp
git fetch oscomp pull/19/head:pr-19-fix(这个是拉取pr的内容，注意这里的19是pr的id，pr-19-fix是分支名，自己可以随意命名)
git checkout test-pr19-only
git cherry-pick c043211   # 第一个提交
git cherry-pick 9a16a9e   # 第二个提交
git cherry-pick 1c72371   # 第三个提交（注意顺序）

若遇到冲突，选择传入的更改
git add .
git cherry-pick --continue
然后进行测试

git branch -d 分支名 本地删分支
git push origin --delete 分支名(删除远程分支)
```
