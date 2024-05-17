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
use HaveECVExt::*;
use ConstrainUnpredictableBits::*;
use u_get_BRBCR_EL2_Type_TS::*;
use u_get_BRBCR_EL1_Type_TS::*;
use Unreachable::*;
use common::*;
pub fn BRBETimeStamp<T: Tracer>(state: &mut State, tracer: &T, gs_2817: ()) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_2819: bool,
        gs_432045: ProductType690b94b58c91cec7,
        ga_1666: u32,
        gs_2832: bool,
        return_value: u32,
        gs_2831: bool,
        TS_el2: u8,
        gs_432034: ProductType690b94b58c91cec7,
        TS_el1: u8,
        gs_2817: (),
    }
    let fn_state = FunctionState {
        gs_2817,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #432u : u32
        let s_0_0: u32 = 432;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u8
        let s_0_2: u8 = 2;
        // D s_0_3: cmp-lt s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) < (s_0_2));
        // N s_0_4: branch s_0_3 b19 b1
        if s_0_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_2_0: const #90640u : u32
        let s_2_0: u32 = 90640;
        // D s_2_1: read-reg s_2_0:struct
        let s_2_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call _get_BRBCR_EL1_Type_TS(s_2_1)
        let s_2_2: u8 = u_get_BRBCR_EL1_Type_TS(state, tracer, s_2_1);
        // D s_2_3: write-var TS_el1 <= s_2_2
        fn_state.TS_el1 = s_2_2;
        // D s_2_4: read-var TS_el1:u8
        let s_2_4: u8 = fn_state.TS_el1;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 2u16);
        // C s_2_6: const #0u : u8
        let s_2_6: u8 = 0;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 2u16);
        // D s_2_8: cmp-eq s_2_5 s_2_7
        let s_2_8: bool = ((s_2_5) == (s_2_7));
        // N s_2_9: branch s_2_8 b18 b3
        if s_2_8 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call HaveECVExt(s_3_0)
        let s_3_1: bool = HaveECVExt(state, tracer, s_3_0);
        // S s_3_2: not s_3_1
        let s_3_2: bool = !s_3_1;
        // N s_3_3: branch s_3_2 b17 b4
        if s_3_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#2831 <= s_4_0
        fn_state.gs_2831 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_5_0: read-var gs#2831:u8
        let s_5_0: bool = fn_state.gs_2831;
        // D s_5_1: write-var gs#2832 <= s_5_0
        fn_state.gs_2832 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_6_0: read-var gs#2832:u8
        let s_6_0: bool = fn_state.gs_2832;
        // N s_6_1: branch s_6_0 b16 b7
        if s_6_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_8_0: read-var TS_el1:u8
        let s_8_0: u8 = fn_state.TS_el1;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #1u : u8
        let s_8_2: u8 = 1;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 2u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b11 b9
        if s_8_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: write-var return_value <= s_9_0
        fn_state.return_value = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_10_0: read-var return_value:u32
        let s_10_0: u32 = fn_state.return_value;
        // N s_10_1: return s_10_0
        return s_10_0;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_11_0: read-var TS_el1:u8
        let s_11_0: u8 = fn_state.TS_el1;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 2u16);
        // C s_11_2: const #2u : u8
        let s_11_2: u8 = 2;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // D s_11_5: not s_11_4
        let s_11_5: bool = !s_11_4;
        // N s_11_6: branch s_11_5 b13 b12
        if s_11_5 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_12_0: const #3u : u32
        let s_12_0: u32 = 3;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_13_0: read-var TS_el1:u8
        let s_13_0: u8 = fn_state.TS_el1;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #3u : u8
        let s_13_2: u8 = 3;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b15 b14
        if s_13_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_14_0: const #2u : u32
        let s_14_0: u32 = 2;
        // D s_14_1: write-var return_value <= s_14_0
        fn_state.return_value = s_14_0;
        // N s_14_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Unreachable(s_15_0)
        let s_15_1: () = Unreachable(state, tracer, s_15_0);
        // D s_15_2: read-var ga#1666:u32
        let s_15_2: u32 = fn_state.ga_1666;
        // D s_15_3: write-var return_value <= s_15_2
        fn_state.return_value = s_15_2;
        // N s_15_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_16_0: const #2s : i
        let s_16_0: i128 = 2;
        // C s_16_1: const #63u : u32
        let s_16_1: u32 = 63;
        // S s_16_2: call ConstrainUnpredictableBits(s_16_1, s_16_0)
        let s_16_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_16_1,
            s_16_0,
        );
        // D s_16_3: write-var gs#432045 <= s_16_2
        fn_state.gs_432045 = s_16_2;
        // D s_16_4: read-var gs#432045.1:struct
        let s_16_4: Bits = fn_state.gs_432045._1;
        // D s_16_5: cast reint s_16_4 -> u8
        let s_16_5: u8 = (s_16_4.value() as u8);
        // D s_16_6: write-var TS_el1 <= s_16_5
        fn_state.TS_el1 = s_16_5;
        // N s_16_7: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_17_0: read-var TS_el1:u8
        let s_17_0: u8 = fn_state.TS_el1;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: write-var gs#2831 <= s_17_4
        fn_state.gs_2831 = s_17_4;
        // N s_17_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#2832 <= s_18_0
        fn_state.gs_2832 = s_18_0;
        // N s_18_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_19_0: const #18272u : u32
        let s_19_0: u32 = 18272;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_BRBCR_EL2_Type_TS(s_19_1)
        let s_19_2: u8 = u_get_BRBCR_EL2_Type_TS(state, tracer, s_19_1);
        // D s_19_3: write-var TS_el2 <= s_19_2
        fn_state.TS_el2 = s_19_2;
        // C s_19_4: const #() : ()
        let s_19_4: () = ();
        // S s_19_5: call HaveECVExt(s_19_4)
        let s_19_5: bool = HaveECVExt(state, tracer, s_19_4);
        // S s_19_6: not s_19_5
        let s_19_6: bool = !s_19_5;
        // N s_19_7: branch s_19_6 b31 b20
        if s_19_6 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#2819 <= s_20_0
        fn_state.gs_2819 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_21_0: read-var gs#2819:u8
        let s_21_0: bool = fn_state.gs_2819;
        // N s_21_1: branch s_21_0 b30 b22
        if s_21_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_22_0: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_23_0: read-var TS_el2:u8
        let s_23_0: u8 = fn_state.TS_el2;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 2u16);
        // C s_23_2: const #0u : u8
        let s_23_2: u8 = 0;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 2u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_24_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_25_0: read-var TS_el2:u8
        let s_25_0: u8 = fn_state.TS_el2;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 2u16);
        // C s_25_2: const #1u : u8
        let s_25_2: u8 = 1;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: not s_25_4
        let s_25_5: bool = !s_25_4;
        // N s_25_6: branch s_25_5 b27 b26
        if s_25_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_26_0: const #4u : u32
        let s_26_0: u32 = 4;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_27_0: read-var TS_el2:u8
        let s_27_0: u8 = fn_state.TS_el2;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 2u16);
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: not s_27_4
        let s_27_5: bool = !s_27_4;
        // N s_27_6: branch s_27_5 b29 b28
        if s_27_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call HaveECVExt(s_28_0)
        let s_28_1: bool = HaveECVExt(state, tracer, s_28_0);
        // N s_28_2: assert s_28_1
        let s_28_2: () = assert!(s_28_1);
        // C s_28_3: const #3u : u32
        let s_28_3: u32 = 3;
        // D s_28_4: write-var return_value <= s_28_3
        fn_state.return_value = s_28_3;
        // N s_28_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_29_0: const #2u : u32
        let s_29_0: u32 = 2;
        // D s_29_1: write-var return_value <= s_29_0
        fn_state.return_value = s_29_0;
        // N s_29_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_30_0: const #2s : i
        let s_30_0: i128 = 2;
        // C s_30_1: const #62u : u32
        let s_30_1: u32 = 62;
        // S s_30_2: call ConstrainUnpredictableBits(s_30_1, s_30_0)
        let s_30_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_30_1,
            s_30_0,
        );
        // D s_30_3: write-var gs#432034 <= s_30_2
        fn_state.gs_432034 = s_30_2;
        // D s_30_4: read-var gs#432034.1:struct
        let s_30_4: Bits = fn_state.gs_432034._1;
        // D s_30_5: cast reint s_30_4 -> u8
        let s_30_5: u8 = (s_30_4.value() as u8);
        // D s_30_6: write-var TS_el2 <= s_30_5
        fn_state.TS_el2 = s_30_5;
        // N s_30_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_31_0: read-var TS_el2:u8
        let s_31_0: u8 = fn_state.TS_el2;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 2u16);
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 2u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#2819 <= s_31_4
        fn_state.gs_2819 = s_31_4;
        // N s_31_6: jump b21
        return block_21(state, tracer, fn_state);
    }
}
