整理一下目前的任务

1、dora移植

2、lwext4e合并

3、docker训练营筹备

下午由于要和腾讯开会，跑一遍他们的大纲

这里整理一下docker大纲中一些指令
```docker
docker image ls # 查看镜像列表
docker ps # 列出所有容器
docker ps -a # 列出所有容器，包括停止的
docker run # 运行一个容器
docker run -d # 后台运行一个容器
docker run -it # 交互式运行一个容器
docker run -p # 端口映射
docker run -v # 卷映射
docker run --name # 容器命名
docker run -e # 环境变量
docker run --restart # 重启策略
docker run --network # 网络模式
docker run --ip # 指定ip
docker run --dns # 指定dns
docker run --cap-add # 添加权限
docker run --device # 添加设备
docker run --log-driver # 日志驱动
docker run --log-opt # 日志选项
docker run --ulimit # ulimit限制
docker run --security-opt # 安全选项
docker run --rm # 退出后删除容器
docker exec # 在容器内执行命令
docker pull 镜像名# 拉取镜像
docker push # 推送镜像
docker stop # 停止容器
docker start # 启动容器
docker restart # 重启容器
docker rm # 删除容器
docker rmi # 删除镜像
docker network # 管理网络
docker volume # 管理卷
docker compose # 编排多个容器
docker attach <container_id> # 打开正在后台运行的容器
```
然后是一些关于dockerfile创建镜像的内容
```dockerfile
# 一般的创建方式：打开了容器，安装了相关软件比如figlet之类的
# 然后docker commit <container_id>, 这个id可以用docker ps -a获取
# 然后docker image ls可以查看镜像列表
# 得到镜像id之后，docker tag <image_id> <image_name>:<tag> 可以给镜像添加标签

# dockerfile的创建方式
docker build -t <image_name>:<tag> <Dockerfile路径>
# 其中Dockerfile路径可以是当前目录，也可以是其他目录
# 比如当前目录有一个Dockerfile，那么路径直接使用 .
# 如果是其他目录，比如/home/user/dockerfile，那么路径就是/home/user/dockerfile
# 例子如下
docker build -t alpine-figlet-from-dockerfile . 
#（当前目录下的dockerfile）
docker build -t jupyter-sample jupyter_sample/  
#（当前目录下的jupyter_sample目录下的dockerfile）


下午开完会了，先把文件系统这个搞了

github又碰到2FA，推荐说用microsoft authenticator ，微软账号安全状态挂起还有6天，先放着，反正还有45天

还有最蛋疼的问题，这ubuntu拼音输入法老是丢字母啊可恶，没查到合适的解决办法

总之每个周末看一看一周的工作日志看看有什么要做的事情，今天就要提醒一下自己22号之后微软账号ok了记得弄一下github的2FA