# mine-rs


> minepy 的 rust wrap 实现. mic(最大信息系数) 计算, 基于 [minepy](https://github.com/minepy/minepy).

## 安装
```bash
# 需要先安装 rust 开发环境
# 开发态安装,测试
maturin develop -r -F py
python test.py
# 构建 whl 安装包
maturin build --release -F py
```
