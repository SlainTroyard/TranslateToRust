import os
import sys

def remove_trailing_backticks(text):
    """删除文本末尾的三个反引号(```)"""
    if text.endswith('```'):
        return text[:-3]
    return text

def process_files_in_directory(directory_path):
    """处理指定目录中的所有文件"""
    processed_count = 0
    removed_backticks_files = []  # 存储删除了尾部```的文件列表
    
    # 确保目录路径存在
    if not os.path.isdir(directory_path):
        print(f"错误: '{directory_path}' 不是有效的目录路径")
        return
    
    # 遍历目录中的所有文件
    for filename in os.listdir(directory_path):
        file_path = os.path.join(directory_path, filename)
        
        # 跳过子目录，只处理文件
        if os.path.isfile(file_path):
            try:
                # 读取文件内容
                with open(file_path, 'r', encoding='utf-8') as file:
                    content = file.read()
                
                # 处理内容
                new_content = remove_trailing_backticks(content)
                
                # 如果内容有变化，则写回文件
                if new_content != content:
                    with open(file_path, 'w', encoding='utf-8') as file:
                        file.write(new_content)
                    print(f"已处理: {file_path}")
                    processed_count += 1
                    removed_backticks_files.append(file_path)  # 添加到已删除```的文件列表
                    
            except Exception as e:
                print(f"处理文件 '{file_path}' 时出错: {str(e)}")
    
    print(f"处理完成。共修改了 {processed_count} 个文件。")
    
    # 输出删除了尾部```的文件列表和数量
    if removed_backticks_files:
        print("\n以下文件删除了尾部的```:")
        for file in removed_backticks_files:
            print(f"- {file}")
        print(f"\n共有 {len(removed_backticks_files)} 个文件删除了尾部的```。")
    else:
        print("没有文件需要删除尾部的```。")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        directory = sys.argv[1]
    else:
        directory = input("请输入要处理的文件夹路径: ")
    
    process_files_in_directory(directory)