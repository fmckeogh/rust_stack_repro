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
use IsFault__2::*;
use HaveMTE2Ext::*;
use AArch64_AllocationTagAccessIsEnabled::*;
use AArch64_Abort::*;
use HaveMTECanonicalTagCheckingExt::*;
use PhysMemTagRead::*;
use IsFault::*;
use HandleExternalReadAbort::*;
use AArch64_TranslateAddress::*;
use common::*;
pub fn AArch64_MemTag_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    accdesc_in: ProductType9878976b5bcce9c9,
) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        tag: u8,
        ga_16214: ProductTypef170cab34335b70c,
        ga_16218: ProductType2743ddd4af418639,
        ga_16221: ProductTypef170cab34335b70c,
        ga_16228: u8,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        return_value: u8,
        memstatus: ProductTypef8c3639b88223255,
        gs_20805: bool,
        gs_20807: bool,
        gs_20806: bool,
        gs_20801: bool,
        address: u64,
        accdesc_in: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        address,
        accdesc_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_0_0: read-var accdesc_in.27:struct
        let s_0_0: bool = fn_state.accdesc_in._27;
        // N s_0_1: branch s_0_0 b30 b1
        if s_0_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#20801 <= s_1_0
        fn_state.gs_20801 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var gs#20801:u8
        let s_2_0: bool = fn_state.gs_20801;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var accdesc_in:struct
        let s_2_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_2_3: write-var accdesc <= s_2_2
        fn_state.accdesc = s_2_2;
        // C s_2_4: const #() : ()
        let s_2_4: () = ();
        // S s_2_5: call HaveMTE2Ext(s_2_4)
        let s_2_5: bool = HaveMTE2Ext(state, tracer, s_2_4);
        // N s_2_6: branch s_2_5 b29 b3
        if s_2_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #1344u : u32
        let s_4_0: u32 = 1344;
        // D s_4_1: read-reg s_4_0:i64
        let s_4_1: i64 = {
            let value = state.read_register::<i64>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: read-var address:u64
        let s_4_3: u64 = fn_state.address;
        // D s_4_4: read-var accdesc:struct
        let s_4_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // C s_4_5: const #1u : u8
        let s_4_5: bool = true;
        // D s_4_6: call AArch64_TranslateAddress(s_4_3, s_4_4, s_4_5, s_4_2)
        let s_4_6: ProductTypece7c66ccb2cab13e = AArch64_TranslateAddress(
            state,
            tracer,
            s_4_3,
            s_4_4,
            s_4_5,
            s_4_2,
        );
        // D s_4_7: write-var memaddrdesc <= s_4_6
        fn_state.memaddrdesc = s_4_6;
        // N s_4_8: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_5_0: read-var memaddrdesc:struct
        let s_5_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_5_1: call IsFault(s_5_0)
        let s_5_1: bool = IsFault(state, tracer, s_5_0);
        // N s_5_2: branch s_5_1 b28 b6
        if s_5_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var accdesc.27:struct
        let s_7_0: bool = fn_state.accdesc._27;
        // N s_7_1: branch s_7_0 b27 b8
        if s_7_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#20805 <= s_8_0
        fn_state.gs_20805 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_9_0: read-var gs#20805:u8
        let s_9_0: bool = fn_state.gs_20805;
        // N s_9_1: branch s_9_0 b23 b10
        if s_9_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call HaveMTECanonicalTagCheckingExt(s_10_0)
        let s_10_1: bool = HaveMTECanonicalTagCheckingExt(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b22 b11
        if s_10_1 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#20806 <= s_11_0
        fn_state.gs_20806 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_12_0: read-var gs#20806:u8
        let s_12_0: bool = fn_state.gs_20806;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#20807 <= s_13_0
        fn_state.gs_20807 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_14_0: read-var gs#20807:u8
        let s_14_0: bool = fn_state.gs_20807;
        // N s_14_1: branch s_14_0 b17 b15
        if s_14_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_15_0: const #0u : u8
        let s_15_0: u8 = 0;
        // D s_15_1: write-var return_value <= s_15_0
        fn_state.return_value = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_16_0: read-var return_value:u8
        let s_16_0: u8 = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_17_0: const #55s : i
        let s_17_0: i128 = 55;
        // D s_17_1: read-var address:u64
        let s_17_1: u64 = fn_state.address;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 64u16);
        // C s_17_3: const #1u : u64
        let s_17_3: u64 = 1;
        // D s_17_4: bit-extract s_17_2 s_17_0 s_17_3
        let s_17_4: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_3).unwrap(),
        ));
        // D s_17_5: cast reint s_17_4 -> u8
        let s_17_5: bool = ((s_17_4.value()) != 0);
        // C s_17_6: const #0s : i
        let s_17_6: i128 = 0;
        // C s_17_7: const #0u : u64
        let s_17_7: u64 = 0;
        // D s_17_8: cast zx s_17_5 -> u64
        let s_17_8: u64 = (s_17_5 as u64);
        // C s_17_9: const #1u : u64
        let s_17_9: u64 = 1;
        // D s_17_10: and s_17_8 s_17_9
        let s_17_10: u64 = ((s_17_8) & (s_17_9));
        // D s_17_11: cmp-eq s_17_10 s_17_9
        let s_17_11: bool = ((s_17_10) == (s_17_9));
        // D s_17_12: lsl s_17_8 s_17_6
        let s_17_12: u64 = s_17_8 << s_17_6;
        // D s_17_13: or s_17_7 s_17_12
        let s_17_13: u64 = ((s_17_7) | (s_17_12));
        // D s_17_14: cmpl s_17_12
        let s_17_14: u64 = !s_17_12;
        // D s_17_15: and s_17_7 s_17_14
        let s_17_15: u64 = ((s_17_7) & (s_17_14));
        // D s_17_16: select s_17_11 s_17_13 s_17_15
        let s_17_16: u64 = if s_17_11 { s_17_13 } else { s_17_15 };
        // D s_17_17: cast trunc s_17_16 -> u8
        let s_17_17: bool = ((s_17_16) != 0);
        // D s_17_18: cast zx s_17_17 -> bv
        let s_17_18: Bits = Bits::new(s_17_17 as u128, 1u16);
        // C s_17_19: const #0u : u8
        let s_17_19: bool = false;
        // C s_17_20: cast zx s_17_19 -> bv
        let s_17_20: Bits = Bits::new(s_17_19 as u128, 1u16);
        // D s_17_21: cmp-eq s_17_18 s_17_20
        let s_17_21: bool = ((s_17_18) == (s_17_20));
        // N s_17_22: branch s_17_21 b20 b18
        if s_17_21 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_18_0: const #15u : u8
        let s_18_0: u8 = 15;
        // D s_18_1: write-var ga#16228 <= s_18_0
        fn_state.ga_16228 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_19_0: read-var ga#16228:u8
        let s_19_0: u8 = fn_state.ga_16228;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_20_0: const #0u : u8
        let s_20_0: u8 = 0;
        // D s_20_1: write-var ga#16228 <= s_20_0
        fn_state.ga_16228 = s_20_0;
        // N s_20_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_21_0: read-var memaddrdesc.2:struct
        let s_21_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_21_1: write-var ga#16221 <= s_21_0
        fn_state.ga_16221 = s_21_0;
        // D s_21_2: read-var ga#16221.6:struct
        let s_21_2: u32 = fn_state.ga_16221._6;
        // C s_21_3: const #2u : u32
        let s_21_3: u32 = 2;
        // D s_21_4: cmp-eq s_21_2 s_21_3
        let s_21_4: bool = ((s_21_2) == (s_21_3));
        // D s_21_5: write-var gs#20807 <= s_21_4
        fn_state.gs_20807 = s_21_4;
        // N s_21_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_22_0: read-var accdesc.27:struct
        let s_22_0: bool = fn_state.accdesc._27;
        // D s_22_1: write-var gs#20806 <= s_22_0
        fn_state.gs_20806 = s_22_0;
        // N s_22_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_23_0: read-var memaddrdesc:struct
        let s_23_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_23_1: read-var accdesc:struct
        let s_23_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_23_2: call PhysMemTagRead(s_23_0, s_23_1)
        let s_23_2: ProductType2743ddd4af418639 = PhysMemTagRead(
            state,
            tracer,
            s_23_0,
            s_23_1,
        );
        // D s_23_3: write-var ga#16218 <= s_23_2
        fn_state.ga_16218 = s_23_2;
        // D s_23_4: read-var ga#16218.0:struct
        let s_23_4: ProductTypef8c3639b88223255 = fn_state.ga_16218._0;
        // D s_23_5: read-var ga#16218.1:struct
        let s_23_5: u8 = fn_state.ga_16218._1;
        // D s_23_6: write-var memstatus <= s_23_4
        fn_state.memstatus = s_23_4;
        // D s_23_7: write-var tag <= s_23_5
        fn_state.tag = s_23_5;
        // D s_23_8: read-var memstatus:struct
        let s_23_8: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_23_9: call IsFault__2(s_23_8)
        let s_23_9: bool = IsFault__2(state, tracer, s_23_8);
        // N s_23_10: branch s_23_9 b26 b24
        if s_23_9 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_25_0: read-var tag:u8
        let s_25_0: u8 = fn_state.tag;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_26_0: const #1s : i
        let s_26_0: i128 = 1;
        // D s_26_1: read-var memstatus:struct
        let s_26_1: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_26_2: read-var memaddrdesc:struct
        let s_26_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_26_3: read-var accdesc:struct
        let s_26_3: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_26_4: call HandleExternalReadAbort(s_26_1, s_26_2, s_26_0, s_26_3)
        let s_26_4: () = HandleExternalReadAbort(
            state,
            tracer,
            s_26_1,
            s_26_2,
            s_26_0,
            s_26_3,
        );
        // N s_26_5: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_27_0: read-var memaddrdesc.2:struct
        let s_27_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_27_1: write-var ga#16214 <= s_27_0
        fn_state.ga_16214 = s_27_0;
        // D s_27_2: read-var ga#16214.6:struct
        let s_27_2: u32 = fn_state.ga_16214._6;
        // C s_27_3: const #1u : u32
        let s_27_3: u32 = 1;
        // D s_27_4: cmp-eq s_27_2 s_27_3
        let s_27_4: bool = ((s_27_2) == (s_27_3));
        // D s_27_5: write-var gs#20805 <= s_27_4
        fn_state.gs_20805 = s_27_4;
        // N s_27_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_28_0: read-var memaddrdesc.0:struct
        let s_28_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_28_1: read-var address:u64
        let s_28_1: u64 = fn_state.address;
        // D s_28_2: call AArch64_Abort(s_28_1, s_28_0)
        let s_28_2: () = AArch64_Abort(state, tracer, s_28_1, s_28_0);
        // N s_28_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_29_0: read-var accdesc.8:struct
        let s_29_0: u8 = fn_state.accdesc._8;
        // D s_29_1: call AArch64_AllocationTagAccessIsEnabled(s_29_0)
        let s_29_1: bool = AArch64_AllocationTagAccessIsEnabled(state, tracer, s_29_0);
        // D s_29_2: write-var accdesc.27 <= s_29_1
        fn_state.accdesc._27 = s_29_1;
        // N s_29_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_30_0: read-var accdesc_in.28:struct
        let s_30_0: bool = fn_state.accdesc_in._28;
        // D s_30_1: not s_30_0
        let s_30_1: bool = !s_30_0;
        // D s_30_2: write-var gs#20801 <= s_30_1
        fn_state.gs_20801 = s_30_1;
        // N s_30_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
