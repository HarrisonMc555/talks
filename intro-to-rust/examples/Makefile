rust_src_all = $(wildcard *.rs)
cpp_src_all = $(wildcard *.cpp)

rust_src_ignore = use-after-free.rs
cpp_src_ignore = 

rust_src = $(filter-out $(rust_src_ignore),$(rust_src_all))
cpp_src = $(filter-out $(cpp_src_ignore),$(cpp_src_all))

rust_bin = $(rust_src:.rs=)
cpp_bin = $(cpp_src:.cpp=)
all_bins = $(rust_bin) $(cpp_bin)

main: $(all_bins)
# main: debug

debug:
	@echo rust_src_all = $(rust_src_all)
	@echo cpp_src_all = $(cpp_src_all)
	@echo
	@echo rust_src_ignore = $(rust_src_ignore)
	@echo cpp_src_ignore = $(cpp_src_ignore)
	@echo
	@echo rust_src = $(rust_src)
	@echo cpp_src = $(cpp_src)
	@echo
	@echo rust_bin = $(rust_bin)
	@echo cpp_bin = $(cpp_bin)
	@echo all_bins = $(all_bins)

$(rust_bin) : % : %.rs
	rustc $<

$(cpp_bin) : % : %.cpp
	g++ $< -o $@

run: $(all_bins)
	@for bin in $^; do echo === $$bin ===; ./$$bin; echo; done

clean:
	rm -f $(all_bins) *~
