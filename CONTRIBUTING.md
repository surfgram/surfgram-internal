# **Contributing to Surfgram**  

🚀 **Want to help improve Surfgram? Great!** Here’s how:  

---

## **1. How to Contribute**  
- **🐛 Report bugs** – Open an [Issue](https://github.com/surfgram/surfgram/issues).  
- **💡 Suggest features** – Start a [Discussion](https://github.com/surfgram/surfgram/discussions).  
- **📝 Improve docs** – Fix typos, clarify guides, or add examples.  
- **🔧 Code contributions** – Fix bugs or add new features (check the [Roadmap](#development-roadmap)).  

---

## **2. Setup for Development**  
### **Requirements**  
- Python 3.8+  
- Git  
- Poetry  
- Rust

### **Install & Run**  
```bash
git clone https://github.com/surfgram/surfgram-internal
cd surfgram_internal
poetry install  # Install deps
poetry shell    # Activate virtualenv
pytest          # Run tests
```

---

## **3. Contribution Rules**  
✔ **Branch naming**: `feature/name` or `fix/issue`  
✔ **Commits**: Follow [Conventional Commits](https://www.conventionalcommits.org/)  

---

## **4. Making a Pull Request**  
1. Fork → Create a new branch.  
2. Make changes.  
3. Update docs if needed.  
4. Run `pytest` & `poetry run lint`.  
5. Open a **clear, descriptive PR**.  

---

🎉 **Thanks for contributing!**