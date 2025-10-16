# ZKVM Demos Docker Configuration

这个目录包含了为不同ZKVMs创建独立Docker环境的配置文件，解决了不同ZKVMs工具链之间的冲突问题。

## 文件结构

```
docker/
├── Dockerfile.nexus      # Nexus ZKVM Docker配置
├── Dockerfile.risc0      # Risc0 ZKVM Docker配置  
├── Dockerfile.sp1        # SP1 ZKVM Docker配置
├── Dockerfile.zkm        # ZKM ZKVM Docker配置
├── docker-compose.yml    # Docker Compose编排文件
├── docker-manager.sh     # 便捷管理脚本
└── README.md            # 本说明文档
```

## 支持的ZKVMs

- **Nexus ZKVM**: 基于RISC-V的零知识虚拟机
- **Risc0 ZKVM**: Risc0项目的零知识虚拟机实现
- **SP1 ZKVM**: Succinct Labs的SP1零知识虚拟机
- **ZKM ZKVM**: Project ZKM的零知识虚拟机

## 快速开始

### 1. 使用便捷脚本（推荐）

```bash
# 构建所有ZKVMs镜像
./docker/docker-manager.sh build all

# 运行特定ZKVM演示
./docker/docker-manager.sh run nexus
./docker/docker-manager.sh run risc0
./docker/docker-manager.sh run sp1
./docker/docker-manager.sh run zkm

# 启动开发环境
./docker/docker-manager.sh dev nexus
./docker/docker-manager.sh dev risc0

# 查看日志
./docker/docker-manager.sh logs nexus

# 停止容器
./docker/docker-manager.sh stop all

# 清理Docker资源
./docker/docker-manager.sh clean
```

### 2. 使用Docker Compose

```bash
# 构建所有镜像
docker-compose -f docker/docker-compose.yml build

# 运行特定ZKVM（使用profiles）
docker-compose -f docker/docker-compose.yml --profile nexus up nexus-zkvm
docker-compose -f docker/docker-compose.yml --profile risc0 up risc0-zkvm
docker-compose -f docker/docker-compose.yml --profile sp1 up sp1-zkvm
docker-compose -f docker/docker-compose.yml --profile zkm up zkm-zkvm

# 运行所有ZKVMs
docker-compose -f docker/docker-compose.yml --profile all up

# 启动开发环境
docker-compose -f docker/docker-compose.yml --profile dev up -d nexus-dev
docker exec -it nexus-dev bash
```

### 3. 直接使用Docker

```bash
# 构建特定ZKVM镜像
docker build -f docker/Dockerfile.nexus -t nexus-zkvm .
docker build -f docker/Dockerfile.risc0 -t risc0-zkvm .
docker build -f docker/Dockerfile.sp1 -t sp1-zkvm .
docker build -f docker/Dockerfile.zkm -t zkm-zkvm .

# 运行容器
docker run -it --rm -v $(pwd):/workspace nexus-zkvm
docker run -it --rm -v $(pwd):/workspace risc0-zkvm
docker run -it --rm -v $(pwd):/workspace sp1-zkvm
docker run -it --rm -v $(pwd):/workspace zkm-zkvm
```

## 详细说明

### Docker Profiles

Docker Compose使用profiles来管理不同的服务组合：

- `nexus`: 仅运行Nexus ZKVM
- `risc0`: 仅运行Risc0 ZKVM  
- `sp1`: 仅运行SP1 ZKVM
- `zkm`: 仅运行ZKM ZKVM
- `all`: 运行所有ZKVMs
- `dev`: 启动开发环境（交互式shell）

### 环境变量

每个ZKVM都有特定的环境变量配置：

#### Nexus ZKVM
- `NEXUS_TOOLCHAIN_VERSION`: Nexus工具链版本
- `NEXUS_CLI_VERSION_TAG`: Nexus CLI版本标签

#### Risc0 ZKVM
- `RISC0_VERSION`: Risc0版本
- `RISC0_CPP_VERSION`: Risc0 C++版本
- `RISC0_RUST_VERSION`: Risc0 Rust版本
- `RISC0_DEV_MODE`: 开发模式标志

#### SP1 ZKVM
- `SP1_DIR`: SP1安装目录
- `SP1_VERSION`: SP1版本

#### ZKM ZKVM
- 使用默认的ZKM工具链配置

### 卷挂载

所有容器都将项目根目录挂载到`/workspace`，这样可以在容器内直接编辑代码并看到更改。

### 网络隔离

每个ZKVM运行在独立的容器中，避免了工具链冲突：

- 不同的Rust工具链版本
- 不同的目标架构
- 不同的依赖库版本
- 不同的环境变量

## 故障排除

### 常见问题

1. **构建失败**: 检查网络连接，某些工具链需要从GitHub下载
2. **权限问题**: 确保Docker有足够权限访问项目目录
3. **端口冲突**: 如果同时运行多个ZKVMs，确保没有端口冲突

### 调试

```bash
# 查看容器日志
docker-compose -f docker/docker-compose.yml logs nexus-zkvm

# 进入容器调试
docker exec -it nexus-dev bash

# 检查镜像大小
docker images | grep zkvm
```

### 清理

```bash
# 清理所有Docker资源
./docker/docker-manager.sh clean

# 或者手动清理
docker-compose -f docker/docker-compose.yml down --remove-orphans
docker system prune -f
```

## 开发建议

1. **使用开发环境**: 对于日常开发，建议使用`dev` profile启动交互式shell
2. **增量构建**: Docker会缓存构建层，只有代码更改时才重新构建
3. **资源监控**: 运行多个ZKVMs时注意系统资源使用情况
4. **版本管理**: 定期更新ZKVM工具链版本以获得最新功能

## 参考资源

- [Nexus ZKVM文档](https://docs.nexus.xyz/zkvm/nexus-zkvm)
- [Risc0文档](https://dev.risczero.com/api/zkvm/quickstart)
- [SP1文档](https://docs.succinct.xyz/docs/sp1/getting-started/quickstart)
- [ZKM文档](https://docs.zkm.io/introduction/quickstart.html)
- [Docker Compose文档](https://docs.docker.com/compose/)
