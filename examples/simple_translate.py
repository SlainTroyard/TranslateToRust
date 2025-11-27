"""
Example: Simple C to Rust translation.
"""

import asyncio
from pathlib import Path

from rustify import Rustify


async def main():
    # Create translator instance
    rustify = Rustify()
    
    # Example C code
    c_code = """
    #include <stdio.h>
    
    typedef struct {
        int x;
        int y;
    } Point;
    
    Point create_point(int x, int y) {
        Point p;
        p.x = x;
        p.y = y;
        return p;
    }
    
    void print_point(Point* p) {
        printf("Point(%d, %d)\\n", p->x, p->y);
    }
    
    int main() {
        Point p = create_point(10, 20);
        print_point(&p);
        return 0;
    }
    """
    
    print("=" * 60)
    print("C Code:")
    print("=" * 60)
    print(c_code)
    
    print("\n" + "=" * 60)
    print("Translating to Rust...")
    print("=" * 60)
    
    try:
        rust_code = await rustify.translate_code(c_code)
        print("\nRust Code:")
        print("-" * 60)
        print(rust_code)
    except Exception as e:
        print(f"Translation failed: {e}")


if __name__ == "__main__":
    asyncio.run(main())

