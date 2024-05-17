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
use AArch64_PAMax::*;
use AArch64_GetS1TTWParams::*;
use AArch64_S1HasAlignmentFault::*;
use SetInGuardedPage::*;
use AArch64_S1DisabledOutputMECID::*;
use u__UNKNOWN_AddressDescriptor::*;
use HasUnprivileged::*;
use replicate_bits_borealis_internal::*;
use is_zero_subrange::*;
use AArch64_AddrTop::*;
use CreateAddressDescriptor::*;
use EL2Enabled::*;
use AArch64_S1ICacheEnabled::*;
use common::*;
pub fn AArch64_S1DisabledOutput<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    regime: u32,
    va_in: u64,
    accdesc: ProductType9878976b5bcce9c9,
    aligned: bool,
) -> ProductTypedc31059ca7e2391c {
    #[derive(Default)]
    struct FunctionState {
        fault: ProductType1d757adad216cdef,
        gs_17437: bool,
        i_cache_attr: ProductType183e6678e5239c85,
        ga_13016: ProductTypeda0231e9dc169f81,
        default_cacheability: ProductType183e6678e5239c85,
        ga_12976: u32,
        gs_17436: bool,
        va: u64,
        oa: ProductTypeda0231e9dc169f81,
        memattrs: ProductTypef170cab34335b70c,
        ipa: ProductTypece7c66ccb2cab13e,
        return_value: ProductTypedc31059ca7e2391c,
        walkparams: ProductTypeef284266e139aee2,
        fault_in: ProductType1d757adad216cdef,
        regime: u32,
        va_in: u64,
        accdesc: ProductType9878976b5bcce9c9,
        aligned: bool,
    }
    let fn_state = FunctionState {
        fault_in,
        regime,
        va_in,
        accdesc,
        aligned,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_0_0: read-var va_in:u64
        let s_0_0: u64 = fn_state.va_in;
        // D s_0_1: write-var va <= s_0_0
        fn_state.va = s_0_0;
        // D s_0_2: read-var accdesc.25:struct
        let s_0_2: u32 = fn_state.accdesc._25;
        // D s_0_3: read-var regime:u32
        let s_0_3: u32 = fn_state.regime;
        // D s_0_4: read-var va:u64
        let s_0_4: u64 = fn_state.va;
        // D s_0_5: call AArch64_GetS1TTWParams(s_0_3, s_0_2, s_0_4)
        let s_0_5: ProductTypeef284266e139aee2 = AArch64_GetS1TTWParams(
            state,
            tracer,
            s_0_3,
            s_0_2,
            s_0_4,
        );
        // D s_0_6: write-var walkparams <= s_0_5
        fn_state.walkparams = s_0_5;
        // D s_0_7: read-var fault_in:struct
        let s_0_7: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_8: write-var fault <= s_0_7
        fn_state.fault = s_0_7;
        // C s_0_9: const #0u : u8
        let s_0_9: bool = false;
        // S s_0_10: call SetInGuardedPage(s_0_9)
        let s_0_10: () = SetInGuardedPage(state, tracer, s_0_9);
        // C s_0_11: const #0s : i
        let s_0_11: i128 = 0;
        // D s_0_12: read-var va:u64
        let s_0_12: u64 = fn_state.va;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 64u16);
        // C s_0_14: const #1s : i64
        let s_0_14: i64 = 1;
        // C s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // C s_0_16: const #55s : i
        let s_0_16: i128 = 55;
        // C s_0_17: add s_0_16 s_0_15
        let s_0_17: i128 = (s_0_16 + s_0_15);
        // D s_0_18: bit-extract s_0_13 s_0_11 s_0_17
        let s_0_18: Bits = (Bits::new(
            ((s_0_13) >> (s_0_11)).value(),
            u16::try_from(s_0_17).unwrap(),
        ));
        // D s_0_19: cast reint s_0_18 -> u56
        let s_0_19: u64 = (s_0_18.value() as u64);
        // D s_0_20: write-var oa.0 <= s_0_19
        fn_state.oa._0 = s_0_19;
        // D s_0_21: read-var accdesc.25:struct
        let s_0_21: u32 = fn_state.accdesc._25;
        // D s_0_22: write-var ga#12976 <= s_0_21
        fn_state.ga_12976 = s_0_21;
        // C s_0_23: const #3u : u32
        let s_0_23: u32 = 3;
        // D s_0_24: read-var ga#12976:u32
        let s_0_24: u32 = fn_state.ga_12976;
        // D s_0_25: cmp-eq s_0_23 s_0_24
        let s_0_25: bool = ((s_0_23) == (s_0_24));
        // D s_0_26: not s_0_25
        let s_0_26: bool = !s_0_25;
        // N s_0_27: branch s_0_26 b44 b1
        if s_0_26 {
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
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var oa.1 <= s_1_0
        fn_state.oa._1 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // C s_2_1: const #4u : u32
        let s_2_1: u32 = 4;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b43 b3
        if s_2_2 {
            return block_43(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#17436 <= s_3_0
        fn_state.gs_17436 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_4_0: read-var gs#17436:u8
        let s_4_0: bool = fn_state.gs_17436;
        // N s_4_1: branch s_4_0 b42 b5
        if s_4_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#17437 <= s_5_0
        fn_state.gs_17437 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_6_0: read-var gs#17437:u8
        let s_6_0: bool = fn_state.gs_17437;
        // N s_6_1: branch s_6_0 b33 b7
        if s_6_0 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_7_0: read-var accdesc.1:struct
        let s_7_0: u32 = fn_state.accdesc._1;
        // C s_7_1: const #0u : u32
        let s_7_1: u32 = 0;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b26 b8
        if s_7_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_8_0: const #1u : u32
        let s_8_0: u32 = 1;
        // D s_8_1: write-var memattrs.2 <= s_8_0
        fn_state.memattrs._2 = s_8_0;
        // C s_8_2: const #3u : u32
        let s_8_2: u32 = 3;
        // D s_8_3: write-var memattrs.0 <= s_8_2
        fn_state.memattrs._0 = s_8_2;
        // C s_8_4: const #2u : u32
        let s_8_4: u32 = 2;
        // D s_8_5: write-var memattrs.5 <= s_8_4
        fn_state.memattrs._5 = s_8_4;
        // D s_8_6: read-var walkparams.19:struct
        let s_8_6: bool = fn_state.walkparams._19;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // C s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // D s_8_10: cmp-eq s_8_7 s_8_9
        let s_8_10: bool = ((s_8_7) == (s_8_9));
        // N s_8_11: branch s_8_10 b20 b9
        if s_8_10 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_9_0: const #0u : u32
        let s_9_0: u32 = 0;
        // D s_9_1: write-var memattrs.6 <= s_9_0
        fn_state.memattrs._6 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_10_0: const #1u : u8
        let s_10_0: bool = true;
        // D s_10_1: write-var memattrs.7 <= s_10_0
        fn_state.memattrs._7 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var memattrs.3 <= s_11_0
        fn_state.memattrs._3 = s_11_0;
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // D s_11_3: write-var fault.9 <= s_11_2
        fn_state.fault._9 = s_11_2;
        // D s_11_4: read-var walkparams.35:struct
        let s_11_4: bool = fn_state.walkparams._35;
        // D s_11_5: read-var accdesc.1:struct
        let s_11_5: u32 = fn_state.accdesc._1;
        // D s_11_6: read-var walkparams.34:struct
        let s_11_6: bool = fn_state.walkparams._34;
        // D s_11_7: call AArch64_AddrTop(s_11_4, s_11_5, s_11_6)
        let s_11_7: i128 = AArch64_AddrTop(state, tracer, s_11_4, s_11_5, s_11_6);
        // C s_11_8: const #() : ()
        let s_11_8: () = ();
        // S s_11_9: call AArch64_PAMax(s_11_8)
        let s_11_9: i64 = AArch64_PAMax(state, tracer, s_11_8);
        // D s_11_10: read-var va:u64
        let s_11_10: u64 = fn_state.va;
        // D s_11_11: cast zx s_11_10 -> bv
        let s_11_11: Bits = Bits::new(s_11_10 as u128, 64u16);
        // S s_11_12: cast zx s_11_9 -> i
        let s_11_12: i128 = (i128::try_from(s_11_9).unwrap());
        // D s_11_13: call is_zero_subrange(s_11_11, s_11_7, s_11_12)
        let s_11_13: bool = is_zero_subrange(state, tracer, s_11_11, s_11_7, s_11_12);
        // D s_11_14: not s_11_13
        let s_11_14: bool = !s_11_13;
        // N s_11_15: branch s_11_14 b19 b12
        if s_11_14 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_12_0: read-var walkparams.21:struct
        let s_12_0: bool = fn_state.walkparams._21;
        // D s_12_1: read-var accdesc:struct
        let s_12_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_12_2: read-var aligned:u8
        let s_12_2: bool = fn_state.aligned;
        // D s_12_3: read-var memattrs:struct
        let s_12_3: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_12_4: call AArch64_S1HasAlignmentFault(s_12_1, s_12_2, s_12_0, s_12_3)
        let s_12_4: bool = AArch64_S1HasAlignmentFault(
            state,
            tracer,
            s_12_1,
            s_12_2,
            s_12_0,
            s_12_3,
        );
        // N s_12_5: branch s_12_4 b18 b13
        if s_12_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_14_0: read-var fault.16:struct
        let s_14_0: u32 = fn_state.fault._16;
        // C s_14_1: const #0u : u32
        let s_14_1: u32 = 0;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b17 b15
        if s_14_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_15_0: read-var va_in:u64
        let s_15_0: u64 = fn_state.va_in;
        // D s_15_1: read-var oa:struct
        let s_15_1: ProductTypeda0231e9dc169f81 = fn_state.oa;
        // D s_15_2: read-var memattrs:struct
        let s_15_2: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_15_3: call CreateAddressDescriptor(s_15_0, s_15_1, s_15_2)
        let s_15_3: ProductTypece7c66ccb2cab13e = CreateAddressDescriptor(
            state,
            tracer,
            s_15_0,
            s_15_1,
            s_15_2,
        );
        // D s_15_4: write-var ipa <= s_15_3
        fn_state.ipa = s_15_3;
        // D s_15_5: read-var ipa.3:struct
        let s_15_5: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_15_6: write-var ga#13016 <= s_15_5
        fn_state.ga_13016 = s_15_5;
        // D s_15_7: read-var ga#13016.1:struct
        let s_15_7: u32 = fn_state.ga_13016._1;
        // D s_15_8: read-var walkparams:struct
        let s_15_8: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_15_9: read-var regime:u32
        let s_15_9: u32 = fn_state.regime;
        // D s_15_10: call AArch64_S1DisabledOutputMECID(s_15_8, s_15_9, s_15_7)
        let s_15_10: u16 = AArch64_S1DisabledOutputMECID(
            state,
            tracer,
            s_15_8,
            s_15_9,
            s_15_7,
        );
        // D s_15_11: write-var ipa.1 <= s_15_10
        fn_state.ipa._1 = s_15_10;
        // D s_15_12: read-var fault:struct
        let s_15_12: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_13: create-product struct = ["s_15_12", "s_15_3"]
        let s_15_13: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_15_12,
            _1: s_15_3,
        };
        // D s_15_14: write-var return_value <= s_15_13
        fn_state.return_value = s_15_13;
        // N s_15_15: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_16_0: read-var return_value:struct
        let s_16_0: ProductTypedc31059ca7e2391c = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call __UNKNOWN_AddressDescriptor(s_17_0)
        let s_17_1: ProductTypece7c66ccb2cab13e = u__UNKNOWN_AddressDescriptor(
            state,
            tracer,
            s_17_0,
        );
        // D s_17_2: read-var fault:struct
        let s_17_2: ProductType1d757adad216cdef = fn_state.fault;
        // D s_17_3: create-product struct = ["s_17_2", "s_17_1"]
        let s_17_3: ProductTypedc31059ca7e2391c = ProductTypedc31059ca7e2391c {
            _0: s_17_2,
            _1: s_17_1,
        };
        // D s_17_4: write-var return_value <= s_17_3
        fn_state.return_value = s_17_3;
        // N s_17_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: write-var fault.16 <= s_18_0
        fn_state.fault._16 = s_18_0;
        // N s_18_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_19_0: const #7u : u32
        let s_19_0: u32 = 7;
        // D s_19_1: write-var fault.16 <= s_19_0
        fn_state.fault._16 = s_19_0;
        // N s_19_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_20_0: const #2u : u32
        let s_20_0: u32 = 2;
        // D s_20_1: write-var memattrs.6 <= s_20_0
        fn_state.memattrs._6 = s_20_0;
        // D s_20_2: read-var walkparams.34:struct
        let s_20_2: bool = fn_state.walkparams._34;
        // D s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // C s_20_4: const #0u : u8
        let s_20_4: bool = false;
        // C s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 1u16);
        // D s_20_6: cmp-eq s_20_3 s_20_5
        let s_20_6: bool = ((s_20_3) == (s_20_5));
        // N s_20_7: branch s_20_6 b23 b21
        if s_20_6 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_21_0: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_22_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_23_0: read-var regime:u32
        let s_23_0: u32 = fn_state.regime;
        // D s_23_1: call HasUnprivileged(s_23_0)
        let s_23_1: bool = HasUnprivileged(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b25 b24
        if s_23_1 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_24_0: const #56s : i
        let s_24_0: i128 = 56;
        // D s_24_1: read-var va:u64
        let s_24_1: u64 = fn_state.va;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 64u16);
        // C s_24_3: const #0u : u8
        let s_24_3: u8 = 0;
        // C s_24_4: cast zx s_24_3 -> bv
        let s_24_4: Bits = Bits::new(s_24_3 as u128, 4u16);
        // C s_24_5: const #3s : i
        let s_24_5: i128 = 3;
        // C s_24_6: const #1u : u64
        let s_24_6: u64 = 1;
        // C s_24_7: cast zx s_24_6 -> bv
        let s_24_7: Bits = Bits::new(s_24_6 as u128, 64u16);
        // C s_24_8: lsl s_24_7 s_24_5
        let s_24_8: Bits = s_24_7 << s_24_5;
        // C s_24_9: sub s_24_8 s_24_7
        let s_24_9: Bits = ((s_24_8) - (s_24_7));
        // C s_24_10: and s_24_4 s_24_9
        let s_24_10: Bits = ((s_24_4) & (s_24_9));
        // C s_24_11: lsl s_24_10 s_24_0
        let s_24_11: Bits = s_24_10 << s_24_0;
        // C s_24_12: lsl s_24_9 s_24_0
        let s_24_12: Bits = s_24_9 << s_24_0;
        // C s_24_13: cmpl s_24_12
        let s_24_13: Bits = !s_24_12;
        // D s_24_14: and s_24_2 s_24_13
        let s_24_14: Bits = ((s_24_2) & (s_24_13));
        // D s_24_15: or s_24_14 s_24_11
        let s_24_15: Bits = ((s_24_14) | (s_24_11));
        // D s_24_16: cast reint s_24_15 -> u64
        let s_24_16: u64 = (s_24_15.value() as u64);
        // D s_24_17: write-var va <= s_24_16
        fn_state.va = s_24_16;
        // N s_24_18: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_25_0: const #55s : i
        let s_25_0: i128 = 55;
        // D s_25_1: read-var va:u64
        let s_25_1: u64 = fn_state.va;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 64u16);
        // C s_25_3: const #1u : u64
        let s_25_3: u64 = 1;
        // D s_25_4: bit-extract s_25_2 s_25_0 s_25_3
        let s_25_4: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_3).unwrap(),
        ));
        // D s_25_5: cast reint s_25_4 -> u8
        let s_25_5: bool = ((s_25_4.value()) != 0);
        // C s_25_6: const #0s : i
        let s_25_6: i128 = 0;
        // C s_25_7: const #0u : u64
        let s_25_7: u64 = 0;
        // D s_25_8: cast zx s_25_5 -> u64
        let s_25_8: u64 = (s_25_5 as u64);
        // C s_25_9: const #1u : u64
        let s_25_9: u64 = 1;
        // D s_25_10: and s_25_8 s_25_9
        let s_25_10: u64 = ((s_25_8) & (s_25_9));
        // D s_25_11: cmp-eq s_25_10 s_25_9
        let s_25_11: bool = ((s_25_10) == (s_25_9));
        // D s_25_12: lsl s_25_8 s_25_6
        let s_25_12: u64 = s_25_8 << s_25_6;
        // D s_25_13: or s_25_7 s_25_12
        let s_25_13: u64 = ((s_25_7) | (s_25_12));
        // D s_25_14: cmpl s_25_12
        let s_25_14: u64 = !s_25_12;
        // D s_25_15: and s_25_7 s_25_14
        let s_25_15: u64 = ((s_25_7) & (s_25_14));
        // D s_25_16: select s_25_11 s_25_13 s_25_15
        let s_25_16: u64 = if s_25_11 { s_25_13 } else { s_25_15 };
        // D s_25_17: cast trunc s_25_16 -> u8
        let s_25_17: bool = ((s_25_16) != 0);
        // D s_25_18: cast zx s_25_17 -> bv
        let s_25_18: Bits = Bits::new(s_25_17 as u128, 1u16);
        // C s_25_19: const #4u : u64
        let s_25_19: u64 = 4;
        // D s_25_20: call replicate_bits_borealis_internal(s_25_18, s_25_19)
        let s_25_20: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_25_18,
            s_25_19,
        );
        // D s_25_21: cast reint s_25_20 -> u8
        let s_25_21: u8 = (s_25_20.value() as u8);
        // C s_25_22: const #56s : i
        let s_25_22: i128 = 56;
        // D s_25_23: read-var va:u64
        let s_25_23: u64 = fn_state.va;
        // D s_25_24: cast zx s_25_23 -> bv
        let s_25_24: Bits = Bits::new(s_25_23 as u128, 64u16);
        // D s_25_25: cast zx s_25_21 -> bv
        let s_25_25: Bits = Bits::new(s_25_21 as u128, 4u16);
        // C s_25_26: const #3s : i
        let s_25_26: i128 = 3;
        // C s_25_27: const #1u : u64
        let s_25_27: u64 = 1;
        // C s_25_28: cast zx s_25_27 -> bv
        let s_25_28: Bits = Bits::new(s_25_27 as u128, 64u16);
        // C s_25_29: lsl s_25_28 s_25_26
        let s_25_29: Bits = s_25_28 << s_25_26;
        // C s_25_30: sub s_25_29 s_25_28
        let s_25_30: Bits = ((s_25_29) - (s_25_28));
        // D s_25_31: and s_25_25 s_25_30
        let s_25_31: Bits = ((s_25_25) & (s_25_30));
        // D s_25_32: lsl s_25_31 s_25_22
        let s_25_32: Bits = s_25_31 << s_25_22;
        // C s_25_33: lsl s_25_30 s_25_22
        let s_25_33: Bits = s_25_30 << s_25_22;
        // C s_25_34: cmpl s_25_33
        let s_25_34: Bits = !s_25_33;
        // D s_25_35: and s_25_24 s_25_34
        let s_25_35: Bits = ((s_25_24) & (s_25_34));
        // D s_25_36: or s_25_35 s_25_32
        let s_25_36: Bits = ((s_25_35) | (s_25_32));
        // D s_25_37: cast reint s_25_36 -> u64
        let s_25_37: u64 = (s_25_36.value() as u64);
        // D s_25_38: write-var va <= s_25_37
        fn_state.va = s_25_37;
        // N s_25_39: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_26_0: read-var regime:u32
        let s_26_0: u32 = fn_state.regime;
        // D s_26_1: call AArch64_S1ICacheEnabled(s_26_0)
        let s_26_1: bool = AArch64_S1ICacheEnabled(state, tracer, s_26_0);
        // N s_26_2: branch s_26_1 b32 b27
        if s_26_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_27_0: const #464u : u32
        let s_27_0: u32 = 464;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: write-var i_cache_attr.0 <= s_27_1
        fn_state.i_cache_attr._0 = s_27_1;
        // N s_27_3: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: write-var memattrs.2 <= s_28_0
        fn_state.memattrs._2 = s_28_0;
        // D s_28_2: read-var i_cache_attr:struct
        let s_28_2: ProductType183e6678e5239c85 = fn_state.i_cache_attr;
        // D s_28_3: write-var memattrs.4 <= s_28_2
        fn_state.memattrs._4 = s_28_2;
        // D s_28_4: read-var i_cache_attr:struct
        let s_28_4: ProductType183e6678e5239c85 = fn_state.i_cache_attr;
        // D s_28_5: write-var memattrs.1 <= s_28_4
        fn_state.memattrs._1 = s_28_4;
        // C s_28_6: const #2u : u32
        let s_28_6: u32 = 2;
        // D s_28_7: write-var memattrs.5 <= s_28_6
        fn_state.memattrs._5 = s_28_6;
        // D s_28_8: read-var walkparams.19:struct
        let s_28_8: bool = fn_state.walkparams._19;
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 1u16);
        // C s_28_10: const #1u : u8
        let s_28_10: bool = true;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 1u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // N s_28_13: branch s_28_12 b31 b29
        if s_28_12 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_29_0: const #0u : u32
        let s_29_0: u32 = 0;
        // D s_29_1: write-var memattrs.6 <= s_29_0
        fn_state.memattrs._6 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var memattrs.7 <= s_30_0
        fn_state.memattrs._7 = s_30_0;
        // N s_30_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_31_0: const #2u : u32
        let s_31_0: u32 = 2;
        // D s_31_1: write-var memattrs.6 <= s_31_0
        fn_state.memattrs._6 = s_31_0;
        // N s_31_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_32_0: const #472u : u32
        let s_32_0: u32 = 472;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: write-var i_cache_attr.0 <= s_32_1
        fn_state.i_cache_attr._0 = s_32_1;
        // C s_32_3: const #504u : u32
        let s_32_3: u32 = 504;
        // D s_32_4: read-reg s_32_3:u8
        let s_32_4: u8 = {
            let value = state.read_register::<u8>(s_32_3 as isize);
            tracer.read_register(s_32_3 as isize, value);
            value
        };
        // D s_32_5: write-var i_cache_attr.1 <= s_32_4
        fn_state.i_cache_attr._1 = s_32_4;
        // C s_32_6: const #0u : u8
        let s_32_6: bool = false;
        // D s_32_7: write-var i_cache_attr.2 <= s_32_6
        fn_state.i_cache_attr._2 = s_32_6;
        // N s_32_8: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_33_0: const #480u : u32
        let s_33_0: u32 = 480;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: write-var default_cacheability.0 <= s_33_1
        fn_state.default_cacheability._0 = s_33_1;
        // C s_33_3: const #512u : u32
        let s_33_3: u32 = 512;
        // D s_33_4: read-reg s_33_3:u8
        let s_33_4: u8 = {
            let value = state.read_register::<u8>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: write-var default_cacheability.1 <= s_33_4
        fn_state.default_cacheability._1 = s_33_4;
        // C s_33_6: const #0u : u8
        let s_33_6: bool = false;
        // D s_33_7: write-var default_cacheability.2 <= s_33_6
        fn_state.default_cacheability._2 = s_33_6;
        // C s_33_8: const #0u : u32
        let s_33_8: u32 = 0;
        // D s_33_9: write-var memattrs.2 <= s_33_8
        fn_state.memattrs._2 = s_33_8;
        // D s_33_10: read-var default_cacheability:struct
        let s_33_10: ProductType183e6678e5239c85 = fn_state.default_cacheability;
        // D s_33_11: write-var memattrs.4 <= s_33_10
        fn_state.memattrs._4 = s_33_10;
        // D s_33_12: read-var default_cacheability:struct
        let s_33_12: ProductType183e6678e5239c85 = fn_state.default_cacheability;
        // D s_33_13: write-var memattrs.1 <= s_33_12
        fn_state.memattrs._1 = s_33_12;
        // C s_33_14: const #0u : u32
        let s_33_14: u32 = 0;
        // D s_33_15: write-var memattrs.5 <= s_33_14
        fn_state.memattrs._5 = s_33_14;
        // D s_33_16: read-var walkparams.5:struct
        let s_33_16: bool = fn_state.walkparams._5;
        // D s_33_17: cast zx s_33_16 -> bv
        let s_33_17: Bits = Bits::new(s_33_16 as u128, 1u16);
        // C s_33_18: const #1u : u8
        let s_33_18: bool = true;
        // C s_33_19: cast zx s_33_18 -> bv
        let s_33_19: Bits = Bits::new(s_33_18 as u128, 1u16);
        // D s_33_20: cmp-eq s_33_17 s_33_19
        let s_33_20: bool = ((s_33_17) == (s_33_19));
        // N s_33_21: branch s_33_20 b41 b34
        if s_33_20 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_34_0: read-var walkparams.19:struct
        let s_34_0: bool = fn_state.walkparams._19;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // N s_34_5: branch s_34_4 b37 b35
        if s_34_4 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_35_0: const #0u : u32
        let s_35_0: u32 = 0;
        // D s_35_1: write-var memattrs.6 <= s_35_0
        fn_state.memattrs._6 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var memattrs.7 <= s_36_0
        fn_state.memattrs._7 = s_36_0;
        // N s_36_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_37_0: const #2u : u32
        let s_37_0: u32 = 2;
        // D s_37_1: write-var memattrs.6 <= s_37_0
        fn_state.memattrs._6 = s_37_0;
        // D s_37_2: read-var walkparams.34:struct
        let s_37_2: bool = fn_state.walkparams._34;
        // D s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // C s_37_4: const #0u : u8
        let s_37_4: bool = false;
        // C s_37_5: cast zx s_37_4 -> bv
        let s_37_5: Bits = Bits::new(s_37_4 as u128, 1u16);
        // D s_37_6: cmp-eq s_37_3 s_37_5
        let s_37_6: bool = ((s_37_3) == (s_37_5));
        // N s_37_7: branch s_37_6 b40 b38
        if s_37_6 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_39_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_40_0: const #55s : i
        let s_40_0: i128 = 55;
        // D s_40_1: read-var va:u64
        let s_40_1: u64 = fn_state.va;
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 64u16);
        // C s_40_3: const #1u : u64
        let s_40_3: u64 = 1;
        // D s_40_4: bit-extract s_40_2 s_40_0 s_40_3
        let s_40_4: Bits = (Bits::new(
            ((s_40_2) >> (s_40_0)).value(),
            u16::try_from(s_40_3).unwrap(),
        ));
        // D s_40_5: cast reint s_40_4 -> u8
        let s_40_5: bool = ((s_40_4.value()) != 0);
        // C s_40_6: const #0s : i
        let s_40_6: i128 = 0;
        // C s_40_7: const #0u : u64
        let s_40_7: u64 = 0;
        // D s_40_8: cast zx s_40_5 -> u64
        let s_40_8: u64 = (s_40_5 as u64);
        // C s_40_9: const #1u : u64
        let s_40_9: u64 = 1;
        // D s_40_10: and s_40_8 s_40_9
        let s_40_10: u64 = ((s_40_8) & (s_40_9));
        // D s_40_11: cmp-eq s_40_10 s_40_9
        let s_40_11: bool = ((s_40_10) == (s_40_9));
        // D s_40_12: lsl s_40_8 s_40_6
        let s_40_12: u64 = s_40_8 << s_40_6;
        // D s_40_13: or s_40_7 s_40_12
        let s_40_13: u64 = ((s_40_7) | (s_40_12));
        // D s_40_14: cmpl s_40_12
        let s_40_14: u64 = !s_40_12;
        // D s_40_15: and s_40_7 s_40_14
        let s_40_15: u64 = ((s_40_7) & (s_40_14));
        // D s_40_16: select s_40_11 s_40_13 s_40_15
        let s_40_16: u64 = if s_40_11 { s_40_13 } else { s_40_15 };
        // D s_40_17: cast trunc s_40_16 -> u8
        let s_40_17: bool = ((s_40_16) != 0);
        // D s_40_18: cast zx s_40_17 -> bv
        let s_40_18: Bits = Bits::new(s_40_17 as u128, 1u16);
        // C s_40_19: const #4u : u64
        let s_40_19: u64 = 4;
        // D s_40_20: call replicate_bits_borealis_internal(s_40_18, s_40_19)
        let s_40_20: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_40_18,
            s_40_19,
        );
        // D s_40_21: cast reint s_40_20 -> u8
        let s_40_21: u8 = (s_40_20.value() as u8);
        // C s_40_22: const #56s : i
        let s_40_22: i128 = 56;
        // D s_40_23: read-var va:u64
        let s_40_23: u64 = fn_state.va;
        // D s_40_24: cast zx s_40_23 -> bv
        let s_40_24: Bits = Bits::new(s_40_23 as u128, 64u16);
        // D s_40_25: cast zx s_40_21 -> bv
        let s_40_25: Bits = Bits::new(s_40_21 as u128, 4u16);
        // C s_40_26: const #3s : i
        let s_40_26: i128 = 3;
        // C s_40_27: const #1u : u64
        let s_40_27: u64 = 1;
        // C s_40_28: cast zx s_40_27 -> bv
        let s_40_28: Bits = Bits::new(s_40_27 as u128, 64u16);
        // C s_40_29: lsl s_40_28 s_40_26
        let s_40_29: Bits = s_40_28 << s_40_26;
        // C s_40_30: sub s_40_29 s_40_28
        let s_40_30: Bits = ((s_40_29) - (s_40_28));
        // D s_40_31: and s_40_25 s_40_30
        let s_40_31: Bits = ((s_40_25) & (s_40_30));
        // D s_40_32: lsl s_40_31 s_40_22
        let s_40_32: Bits = s_40_31 << s_40_22;
        // C s_40_33: lsl s_40_30 s_40_22
        let s_40_33: Bits = s_40_30 << s_40_22;
        // C s_40_34: cmpl s_40_33
        let s_40_34: Bits = !s_40_33;
        // D s_40_35: and s_40_24 s_40_34
        let s_40_35: Bits = ((s_40_24) & (s_40_34));
        // D s_40_36: or s_40_35 s_40_32
        let s_40_36: Bits = ((s_40_35) | (s_40_32));
        // D s_40_37: cast reint s_40_36 -> u64
        let s_40_37: u64 = (s_40_36.value() as u64);
        // D s_40_38: write-var va <= s_40_37
        fn_state.va = s_40_37;
        // N s_40_39: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_41_0: const #1u : u32
        let s_41_0: u32 = 1;
        // D s_41_1: write-var memattrs.6 <= s_41_0
        fn_state.memattrs._6 = s_41_0;
        // N s_41_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // D s_42_0: read-var walkparams.4:struct
        let s_42_0: bool = fn_state.walkparams._4;
        // D s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 1u16);
        // C s_42_2: const #1u : u8
        let s_42_2: bool = true;
        // C s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 1u16);
        // D s_42_4: cmp-eq s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) == (s_42_3));
        // D s_42_5: write-var gs#17437 <= s_42_4
        fn_state.gs_17437 = s_42_4;
        // N s_42_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EL2Enabled(s_43_0)
        let s_43_1: bool = EL2Enabled(state, tracer, s_43_0);
        // D s_43_2: write-var gs#17436 <= s_43_1
        fn_state.gs_17436 = s_43_1;
        // N s_43_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_44_0: const #0u : u32
        let s_44_0: u32 = 0;
        // D s_44_1: read-var ga#12976:u32
        let s_44_1: u32 = fn_state.ga_12976;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b46 b45
        if s_44_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_45_0: const #0u : u32
        let s_45_0: u32 = 0;
        // D s_45_1: write-var oa.1 <= s_45_0
        fn_state.oa._1 = s_45_0;
        // N s_45_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_46_0: const #1u : u32
        let s_46_0: u32 = 1;
        // D s_46_1: read-var ga#12976:u32
        let s_46_1: u32 = fn_state.ga_12976;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // N s_46_4: branch s_46_3 b48 b47
        if s_46_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_47_0: const #2u : u32
        let s_47_0: u32 = 2;
        // D s_47_1: write-var oa.1 <= s_47_0
        fn_state.oa._1 = s_47_0;
        // N s_47_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_48_0: const #2u : u32
        let s_48_0: u32 = 2;
        // D s_48_1: read-var ga#12976:u32
        let s_48_1: u32 = fn_state.ga_12976;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b50 b49
        if s_48_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // C s_49_0: const #3u : u32
        let s_49_0: u32 = 3;
        // D s_49_1: write-var oa.1 <= s_49_0
        fn_state.oa._1 = s_49_0;
        // N s_49_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypedc31059ca7e2391c {
        // N s_50_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
