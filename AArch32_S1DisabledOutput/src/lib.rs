#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch32_EL2Enabled::*;
use SetInGuardedPage::*;
use HaveAArch32EL::*;
use u_get_HSCTLR_Type_nTLSMD::*;
use SCTLR_NS_read::*;
use u_get_HCR_EL2_Type_DCT::*;
use HaveMTE2Ext::*;
use HCR_read::*;
use u_get_SCTLR_Type_nTLSMD::*;
use AArch32_S1HasAlignmentFault::*;
use CreateAddressDescriptor::*;
use ELStateUsingAArch32::*;
use SCTLR_read__2::*;
use u_get_HCR_EL2_Type_DC::*;
use u_get_HCR_Type_DC::*;
use HaveTrapLoadStoreMultipleDeviceExt::*;
use AArch32_S1ICacheEnabled::*;
use u__UNKNOWN_AddressDescriptor::*;
use HSCTLR_read::*;
use common::*;
pub fn AArch32_S1DisabledOutput<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    va: u32,
    aligned: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        default_cacheable: bool,
        fault: ProductType1d757adad216cdef,
        return_value: ProductTypedc31059ca7e2391c,
        ntlsmd: bool,
        gs_27679: bool,
        oa: ProductTypeda0231e9dc169f81,
        gs_27693: bool,
        memattrs: ProductTypef170cab34335b70c,
        gs_27694: bool,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        va: u32,
        aligned: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        va,
        aligned,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // S s_0_3: call SetInGuardedPage(s_0_2)
        let s_0_3: () = SetInGuardedPage(state, tracer, s_0_2);
        // D s_0_4: read-var regime:u32
        let s_0_4: u32 = fn_state.regime;
        // C s_0_5: const #4u : u32
        let s_0_5: u32 = 4;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // N s_0_7: branch s_0_6 b44 b1
        if s_0_6 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#27679 <= s_1_0
        fn_state.gs_27679 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var gs#27679:u8
        let s_2_0: bool = fn_state.gs_27679;
        // N s_2_1: branch s_2_0 b41 b3
        if s_2_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var default_cacheable <= s_3_0
        fn_state.default_cacheable = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_4_0: read-var default_cacheable:u8
        let s_4_0: bool = fn_state.default_cacheable;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b31 b5
        if s_4_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_5_0: read-var accdesc.1:struct
        let s_5_0: u32 = fn_state.accdesc._1;
        // C s_5_1: const #0u : u32
        let s_5_1: u32 = 0;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b27 b6
        if s_5_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var memattrs.2 <= s_6_0
        fn_state.memattrs._2 = s_6_0;
        // C s_6_2: const #3u : u32
        let s_6_2: u32 = 3;
        // D s_6_3: write-var memattrs.0 <= s_6_2
        fn_state.memattrs._0 = s_6_2;
        // C s_6_4: const #2u : u32
        let s_6_4: u32 = 2;
        // D s_6_5: write-var memattrs.5 <= s_6_4
        fn_state.memattrs._5 = s_6_4;
        // C s_6_6: const #0u : u32
        let s_6_6: u32 = 0;
        // D s_6_7: write-var memattrs.6 <= s_6_6
        fn_state.memattrs._6 = s_6_6;
        // C s_6_8: const #1u : u8
        let s_6_8: bool = true;
        // D s_6_9: write-var memattrs.7 <= s_6_8
        fn_state.memattrs._7 = s_6_8;
        // N s_6_10: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveTrapLoadStoreMultipleDeviceExt(s_7_0)
        let s_7_1: bool = HaveTrapLoadStoreMultipleDeviceExt(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b16 b8
        if s_7_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var ntlsmd <= s_8_0
        fn_state.ntlsmd = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_9_0: read-var accdesc:struct
        let s_9_0: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_9_1: read-var aligned:u8
        let s_9_1: bool = fn_state.aligned;
        // D s_9_2: read-var ntlsmd:u8
        let s_9_2: bool = fn_state.ntlsmd;
        // D s_9_3: read-var memattrs:struct
        let s_9_3: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_9_4: call AArch32_S1HasAlignmentFault(s_9_0, s_9_1, s_9_2, s_9_3)
        let s_9_4: bool = AArch32_S1HasAlignmentFault(
            state,
            tracer,
            s_9_0,
            s_9_1,
            s_9_2,
            s_9_3,
        );
        // N s_9_5: branch s_9_4 b15 b10
        if s_9_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_10_0: const #56s : i
        let s_10_0: i128 = 56;
        // D s_10_1: read-var va:u32
        let s_10_1: u32 = fn_state.va;
        // D s_10_2: cast zx s_10_1 -> bv
        let s_10_2: Bits = Bits::new(s_10_1 as u128, 32u16);
        // D s_10_3: bits-cast zx s_10_2 -> bv length s_10_0
        let s_10_3: Bits = s_10_2.zero_extend(s_10_0);
        // D s_10_4: cast reint s_10_3 -> u56
        let s_10_4: u64 = (s_10_3.value() as u64);
        // D s_10_5: write-var oa.0 <= s_10_4
        fn_state.oa._0 = s_10_4;
        // D s_10_6: read-var accdesc.25:struct
        let s_10_6: u32 = fn_state.accdesc._25;
        // C s_10_7: const #3u : u32
        let s_10_7: u32 = 3;
        // D s_10_8: cmp-eq s_10_6 s_10_7
        let s_10_8: bool = ((s_10_6) == (s_10_7));
        // N s_10_9: branch s_10_8 b14 b11
        if s_10_8 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_11_0: const #0u : u32
        let s_11_0: u32 = 0;
        // D s_11_1: write-var oa.1 <= s_11_0
        fn_state.oa._1 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_12_0: const #64s : i
        let s_12_0: i128 = 64;
        // D s_12_1: read-var va:u32
        let s_12_1: u32 = fn_state.va;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 32u16);
        // D s_12_3: bits-cast zx s_12_2 -> bv length s_12_0
        let s_12_3: Bits = s_12_2.zero_extend(s_12_0);
        // D s_12_4: cast reint s_12_3 -> u64
        let s_12_4: u64 = (s_12_3.value() as u64);
        // D s_12_5: read-var oa:struct
        let s_12_5: ProductTypeda0231e9dc169f81 = fn_state.oa;
        // D s_12_6: read-var memattrs:struct
        let s_12_6: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_12_7: call CreateAddressDescriptor(s_12_4, s_12_5, s_12_6)
        let s_12_7: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_12_4,
            s_12_5,
            s_12_6,
        );
        // D s_12_8: read-var fault:struct
        let s_12_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_12_9: create-product struct = ["s_12_8", "s_12_7"]
        let s_12_9: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_12_8,
            _1: s_12_7,
        };
        // D s_12_10: write-var return_value <= s_12_9
        fn_state.return_value = s_12_9;
        // N s_12_11: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_13_0: read-var return_value:struct
        let s_13_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: write-var oa.1 <= s_14_0
        fn_state.oa._1 = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_15_0: const #2u : u32
        let s_15_0: u32 = 2;
        // D s_15_1: write-var fault.16 <= s_15_0
        fn_state.fault._16 = s_15_0;
        // C s_15_2: const #() : ()
        let s_15_2: () = ();
        // S s_15_3: call __UNKNOWN_AddressDescriptor(s_15_2)
        let s_15_3: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_15_2,
        );
        // D s_15_4: read-var fault:struct
        let s_15_4: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_5: create-product struct = ["s_15_4", "s_15_3"]
        let s_15_5: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_15_4,
            _1: s_15_3,
        };
        // D s_15_6: write-var return_value <= s_15_5
        fn_state.return_value = s_15_5;
        // N s_15_7: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_16_0: const #1u : u32
        let s_16_0: u32 = 1;
        // D s_16_1: read-var regime:u32
        let s_16_1: u32 = fn_state.regime;
        // D s_16_2: cmp-eq s_16_0 s_16_1
        let s_16_2: bool = ((s_16_0) == (s_16_1));
        // D s_16_3: not s_16_2
        let s_16_3: bool = !s_16_2;
        // N s_16_4: branch s_16_3 b19 b17
        if s_16_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_17_0: const #16456u : u32
        let s_17_0: u32 = 16456;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_SCTLR_Type_nTLSMD(s_17_1)
        let s_17_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_17_1);
        // D s_17_3: write-var ntlsmd <= s_17_2
        fn_state.ntlsmd = s_17_2;
        // N s_17_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_18_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_19_0: const #2u : u32
        let s_19_0: u32 = 2;
        // D s_19_1: read-var regime:u32
        let s_19_1: u32 = fn_state.regime;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call HSCTLR_read(s_20_0)
        let s_20_1: ProductType700c18a878c5601b = HSCTLR_read(state, tracer, s_20_0);
        // S s_20_2: call _get_HSCTLR_Type_nTLSMD(s_20_1)
        let s_20_2: bool = u_get_HSCTLR_Type_nTLSMD(state, tracer, s_20_1);
        // D s_20_3: write-var ntlsmd <= s_20_2
        fn_state.ntlsmd = s_20_2;
        // N s_20_4: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_21_0: const #4u : u32
        let s_21_0: u32 = 4;
        // D s_21_1: read-var regime:u32
        let s_21_1: u32 = fn_state.regime;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b26 b22
        if s_21_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_22_0: const #424u : u32
        let s_22_0: u32 = 424;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call HaveAArch32EL(s_22_1)
        let s_22_2: bool = HaveAArch32EL(state, tracer, s_22_1);
        // N s_22_3: branch s_22_2 b25 b23
        if s_22_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call SCTLR_read__2(s_23_0)
        let s_23_1: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_23_0);
        // S s_23_2: call _get_SCTLR_Type_nTLSMD(s_23_1)
        let s_23_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_23_1);
        // D s_23_3: write-var ntlsmd <= s_23_2
        fn_state.ntlsmd = s_23_2;
        // N s_23_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_24_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call SCTLR_NS_read(s_25_0)
        let s_25_1: ProductType700c18a878c5601b = SCTLR_NS_read(state, tracer, s_25_0);
        // S s_25_2: call _get_SCTLR_Type_nTLSMD(s_25_1)
        let s_25_2: bool = u_get_SCTLR_Type_nTLSMD(state, tracer, s_25_1);
        // D s_25_3: write-var ntlsmd <= s_25_2
        fn_state.ntlsmd = s_25_2;
        // N s_25_4: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_26_0: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_27_0: const #0u : u32
        let s_27_0: u32 = 0;
        // D s_27_1: write-var memattrs.2 <= s_27_0
        fn_state.memattrs._2 = s_27_0;
        // C s_27_2: const #2u : u32
        let s_27_2: u32 = 2;
        // D s_27_3: write-var memattrs.5 <= s_27_2
        fn_state.memattrs._5 = s_27_2;
        // C s_27_4: const #0u : u32
        let s_27_4: u32 = 0;
        // D s_27_5: write-var memattrs.6 <= s_27_4
        fn_state.memattrs._6 = s_27_4;
        // D s_27_6: read-var regime:u32
        let s_27_6: u32 = fn_state.regime;
        // D s_27_7: call AArch32_S1ICacheEnabled(s_27_6)
        let s_27_7: bool = AArch32_S1ICacheEnabled(state, tracer, s_27_6);
        // N s_27_8: branch s_27_7 b30 b28
        if s_27_7 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_28_0: const #464u : u32
        let s_28_0: u32 = 464;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: write-var memattrs.1.0 <= s_28_1
        fn_state.memattrs._1._0 = s_28_1;
        // C s_28_3: const #464u : u32
        let s_28_3: u32 = 464;
        // D s_28_4: read-reg s_28_3:u8
        let s_28_4: u8 = {
            let value = state.read_register::<u8>(s_28_3 as isize);
            tracer.read_register(s_28_3 as isize, value);
            value
        };
        // D s_28_5: write-var memattrs.4.0 <= s_28_4
        fn_state.memattrs._4._0 = s_28_4;
        // N s_28_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var memattrs.7 <= s_29_0
        fn_state.memattrs._7 = s_29_0;
        // N s_29_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_30_0: const #472u : u32
        let s_30_0: u32 = 472;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: write-var memattrs.1.0 <= s_30_1
        fn_state.memattrs._1._0 = s_30_1;
        // C s_30_3: const #504u : u32
        let s_30_3: u32 = 504;
        // D s_30_4: read-reg s_30_3:u8
        let s_30_4: u8 = {
            let value = state.read_register::<u8>(s_30_3 as isize);
            tracer.read_register(s_30_3 as isize, value);
            value
        };
        // D s_30_5: write-var memattrs.1.1 <= s_30_4
        fn_state.memattrs._1._1 = s_30_4;
        // C s_30_6: const #472u : u32
        let s_30_6: u32 = 472;
        // D s_30_7: read-reg s_30_6:u8
        let s_30_7: u8 = {
            let value = state.read_register::<u8>(s_30_6 as isize);
            tracer.read_register(s_30_6 as isize, value);
            value
        };
        // D s_30_8: write-var memattrs.4.0 <= s_30_7
        fn_state.memattrs._4._0 = s_30_7;
        // C s_30_9: const #504u : u32
        let s_30_9: u32 = 504;
        // D s_30_10: read-reg s_30_9:u8
        let s_30_10: u8 = {
            let value = state.read_register::<u8>(s_30_9 as isize);
            tracer.read_register(s_30_9 as isize, value);
            value
        };
        // D s_30_11: write-var memattrs.4.1 <= s_30_10
        fn_state.memattrs._4._1 = s_30_10;
        // N s_30_12: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_31_0: const #0u : u32
        let s_31_0: u32 = 0;
        // D s_31_1: write-var memattrs.2 <= s_31_0
        fn_state.memattrs._2 = s_31_0;
        // C s_31_2: const #480u : u32
        let s_31_2: u32 = 480;
        // D s_31_3: read-reg s_31_2:u8
        let s_31_3: u8 = {
            let value = state.read_register::<u8>(s_31_2 as isize);
            tracer.read_register(s_31_2 as isize, value);
            value
        };
        // D s_31_4: write-var memattrs.1.0 <= s_31_3
        fn_state.memattrs._1._0 = s_31_3;
        // C s_31_5: const #512u : u32
        let s_31_5: u32 = 512;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: write-var memattrs.1.1 <= s_31_6
        fn_state.memattrs._1._1 = s_31_6;
        // C s_31_8: const #480u : u32
        let s_31_8: u32 = 480;
        // D s_31_9: read-reg s_31_8:u8
        let s_31_9: u8 = {
            let value = state.read_register::<u8>(s_31_8 as isize);
            tracer.read_register(s_31_8 as isize, value);
            value
        };
        // D s_31_10: write-var memattrs.4.0 <= s_31_9
        fn_state.memattrs._4._0 = s_31_9;
        // C s_31_11: const #512u : u32
        let s_31_11: u32 = 512;
        // D s_31_12: read-reg s_31_11:u8
        let s_31_12: u8 = {
            let value = state.read_register::<u8>(s_31_11 as isize);
            tracer.read_register(s_31_11 as isize, value);
            value
        };
        // D s_31_13: write-var memattrs.4.1 <= s_31_12
        fn_state.memattrs._4._1 = s_31_12;
        // C s_31_14: const #0u : u32
        let s_31_14: u32 = 0;
        // D s_31_15: write-var memattrs.5 <= s_31_14
        fn_state.memattrs._5 = s_31_14;
        // D s_31_16: read-var accdesc.25:struct
        let s_31_16: u32 = fn_state.accdesc._25;
        // C s_31_17: const #3u : u32
        let s_31_17: u32 = 3;
        // D s_31_18: cmp-eq s_31_16 s_31_17
        let s_31_18: bool = ((s_31_16) == (s_31_17));
        // C s_31_19: const #432u : u32
        let s_31_19: u32 = 432;
        // D s_31_20: read-reg s_31_19:u8
        let s_31_20: u8 = {
            let value = state.read_register::<u8>(s_31_19 as isize);
            tracer.read_register(s_31_19 as isize, value);
            value
        };
        // D s_31_21: call ELStateUsingAArch32(s_31_20, s_31_18)
        let s_31_21: bool = ELStateUsingAArch32(state, tracer, s_31_20, s_31_18);
        // D s_31_22: not s_31_21
        let s_31_22: bool = !s_31_21;
        // N s_31_23: branch s_31_22 b40 b32
        if s_31_22 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#27693 <= s_32_0
        fn_state.gs_27693 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_33_0: read-var gs#27693:u8
        let s_33_0: bool = fn_state.gs_27693;
        // N s_33_1: branch s_33_0 b39 b34
        if s_33_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#27694 <= s_34_0
        fn_state.gs_27694 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_35_0: read-var gs#27694:u8
        let s_35_0: bool = fn_state.gs_27694;
        // N s_35_1: branch s_35_0 b38 b36
        if s_35_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_36_0: const #0u : u32
        let s_36_0: u32 = 0;
        // D s_36_1: write-var memattrs.6 <= s_36_0
        fn_state.memattrs._6 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var memattrs.7 <= s_37_0
        fn_state.memattrs._7 = s_37_0;
        // N s_37_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_38_0: const #1u : u32
        let s_38_0: u32 = 1;
        // D s_38_1: write-var memattrs.6 <= s_38_0
        fn_state.memattrs._6 = s_38_0;
        // N s_38_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_39_0: const #102552u : u32
        let s_39_0: u32 = 102552;
        // D s_39_1: read-reg s_39_0:struct
        let s_39_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call _get_HCR_EL2_Type_DCT(s_39_1)
        let s_39_2: bool = u_get_HCR_EL2_Type_DCT(state, tracer, s_39_1);
        // D s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // C s_39_4: const #1u : u8
        let s_39_4: bool = true;
        // C s_39_5: cast zx s_39_4 -> bv
        let s_39_5: Bits = Bits::new(s_39_4 as u128, 1u16);
        // D s_39_6: cmp-eq s_39_3 s_39_5
        let s_39_6: bool = ((s_39_3) == (s_39_5));
        // D s_39_7: write-var gs#27694 <= s_39_6
        fn_state.gs_27694 = s_39_6;
        // N s_39_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call HaveMTE2Ext(s_40_0)
        let s_40_1: bool = HaveMTE2Ext(state, tracer, s_40_0);
        // D s_40_2: write-var gs#27693 <= s_40_1
        fn_state.gs_27693 = s_40_1;
        // N s_40_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_41_0: read-var accdesc.25:struct
        let s_41_0: u32 = fn_state.accdesc._25;
        // C s_41_1: const #3u : u32
        let s_41_1: u32 = 3;
        // D s_41_2: cmp-eq s_41_0 s_41_1
        let s_41_2: bool = ((s_41_0) == (s_41_1));
        // C s_41_3: const #432u : u32
        let s_41_3: u32 = 432;
        // D s_41_4: read-reg s_41_3:u8
        let s_41_4: u8 = {
            let value = state.read_register::<u8>(s_41_3 as isize);
            tracer.read_register(s_41_3 as isize, value);
            value
        };
        // D s_41_5: call ELStateUsingAArch32(s_41_4, s_41_2)
        let s_41_5: bool = ELStateUsingAArch32(state, tracer, s_41_4, s_41_2);
        // N s_41_6: branch s_41_5 b43 b42
        if s_41_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_42_0: const #102552u : u32
        let s_42_0: u32 = 102552;
        // D s_42_1: read-reg s_42_0:struct
        let s_42_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call _get_HCR_EL2_Type_DC(s_42_1)
        let s_42_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_42_1);
        // D s_42_3: write-var default_cacheable <= s_42_2
        fn_state.default_cacheable = s_42_2;
        // N s_42_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call HCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_HCR_Type_DC(s_43_1)
        let s_43_2: bool = u_get_HCR_Type_DC(state, tracer, s_43_1);
        // D s_43_3: write-var default_cacheable <= s_43_2
        fn_state.default_cacheable = s_43_2;
        // N s_43_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_44_0: read-var accdesc.25:struct
        let s_44_0: u32 = fn_state.accdesc._25;
        // D s_44_1: call AArch32_EL2Enabled(s_44_0)
        let s_44_1: bool = AArch32_EL2Enabled(state, tracer, s_44_0);
        // D s_44_2: write-var gs#27679 <= s_44_1
        fn_state.gs_27679 = s_44_1;
        // N s_44_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
