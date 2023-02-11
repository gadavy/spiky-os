use raw_cpuid::CpuId;

pub fn has_x2apic() -> bool {
    let cpuid = CpuId::new();

    match cpuid.get_feature_info() {
        Some(finfo) => finfo.has_x2apic(),
        None => false,
    }
}
