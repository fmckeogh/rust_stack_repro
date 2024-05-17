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
use CheckValidStateMatch::*;
use u__UNKNOWN_bits::*;
use AArch32_BreakpointValueMatch::*;
use IsContextMatchingBreakpoint::*;
use ContextMatchingBreakpointRange::*;
use ConstrainUnpredictableInteger::*;
use common::*;
pub fn AArch32_StateMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ssc_in: u8,
    hmc_in: bool,
    pxc_in: u8,
    linked_in: bool,
    linked_n_in: i128,
    isbreakpnt: bool,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        pl2_match: bool,
        gs_29958: bool,
        hmc: bool,
        ssc: u8,
        gs_29920: bool,
        gs_29975: bool,
        gs_29943: bool,
        gs_29959: bool,
        gs_29937: bool,
        ga_23145: ProductTypeba129578e5d1bd1b,
        gs_29974: bool,
        ga_23160: ProductType396b95aa74979079,
        linked_match: bool,
        c: u32,
        gs_29929: bool,
        gs_29922: bool,
        linked_n: i128,
        return_value: bool,
        ga_23159: ProductTypec716851b6df2cc69,
        pl0_match: bool,
        ss_match: bool,
        pl1_match: bool,
        ga_23152: u8,
        gs_29927: bool,
        pxc: u8,
        gs_29938: bool,
        gs_29928: bool,
        ga_23164: ProductType8b847afc727d5818,
        gs_29973: bool,
        linked: bool,
        gs_29921: bool,
        priv_match: bool,
        ssc_in: u8,
        hmc_in: bool,
        pxc_in: u8,
        linked_in: bool,
        linked_n_in: i128,
        isbreakpnt: bool,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        ssc_in,
        hmc_in,
        pxc_in,
        linked_in,
        linked_n_in,
        isbreakpnt,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var hmc_in:u8
        let s_0_0: bool = fn_state.hmc_in;
        // D s_0_1: write-var hmc <= s_0_0
        fn_state.hmc = s_0_0;
        // D s_0_2: read-var ssc_in:u8
        let s_0_2: u8 = fn_state.ssc_in;
        // D s_0_3: write-var ssc <= s_0_2
        fn_state.ssc = s_0_2;
        // D s_0_4: read-var pxc_in:u8
        let s_0_4: u8 = fn_state.pxc_in;
        // D s_0_5: write-var pxc <= s_0_4
        fn_state.pxc = s_0_4;
        // D s_0_6: read-var linked_in:u8
        let s_0_6: bool = fn_state.linked_in;
        // D s_0_7: write-var linked <= s_0_6
        fn_state.linked = s_0_6;
        // D s_0_8: read-var linked_n_in:i
        let s_0_8: i128 = fn_state.linked_n_in;
        // D s_0_9: write-var linked_n <= s_0_8
        fn_state.linked_n = s_0_8;
        // D s_0_10: read-var ssc:u8
        let s_0_10: u8 = fn_state.ssc;
        // C s_0_11: const #0u : u8
        let s_0_11: bool = false;
        // D s_0_12: read-var hmc:u8
        let s_0_12: bool = fn_state.hmc;
        // D s_0_13: read-var pxc:u8
        let s_0_13: u8 = fn_state.pxc;
        // D s_0_14: read-var isbreakpnt:u8
        let s_0_14: bool = fn_state.isbreakpnt;
        // D s_0_15: call CheckValidStateMatch(s_0_10, s_0_11, s_0_12, s_0_13, s_0_14)
        let s_0_15: ProductTypeba129578e5d1bd1b = CheckValidStateMatch(
            state,
            tracer,
            s_0_10,
            s_0_11,
            s_0_12,
            s_0_13,
            s_0_14,
        );
        // D s_0_16: write-var ga#23145 <= s_0_15
        fn_state.ga_23145 = s_0_15;
        // D s_0_17: read-var ga#23145.0:struct
        let s_0_17: u32 = fn_state.ga_23145._0;
        // D s_0_18: read-var ga#23145.1:struct
        let s_0_18: u8 = fn_state.ga_23145._1;
        // D s_0_19: read-var ga#23145.3:struct
        let s_0_19: bool = fn_state.ga_23145._3;
        // D s_0_20: read-var ga#23145.4:struct
        let s_0_20: u8 = fn_state.ga_23145._4;
        // D s_0_21: write-var c <= s_0_17
        fn_state.c = s_0_17;
        // D s_0_22: write-var ssc <= s_0_18
        fn_state.ssc = s_0_18;
        // D s_0_23: write-var hmc <= s_0_19
        fn_state.hmc = s_0_19;
        // D s_0_24: write-var pxc <= s_0_20
        fn_state.pxc = s_0_20;
        // D s_0_25: read-var c:u32
        let s_0_25: u32 = fn_state.c;
        // C s_0_26: const #7u : u32
        let s_0_26: u32 = 7;
        // D s_0_27: cmp-eq s_0_25 s_0_26
        let s_0_27: bool = ((s_0_25) == (s_0_26));
        // N s_0_28: branch s_0_27 b77 b1
        if s_0_27 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #432u : u32
        let s_1_0: u32 = 432;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: u8 = {
            let value = state.read_register::<u8>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #2u : u8
        let s_1_2: u8 = 2;
        // D s_1_3: cmp-lt s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) < (s_1_2));
        // N s_1_4: branch s_1_3 b70 b2
        if s_1_3 {
            return block_70(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#29922 <= s_2_0
        fn_state.gs_29922 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#29922:u8
        let s_3_0: bool = fn_state.gs_29922;
        // D s_3_1: write-var pl2_match <= s_3_0
        fn_state.pl2_match = s_3_0;
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: read-var pxc:u8
        let s_3_3: u8 = fn_state.pxc;
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // C s_3_5: const #1u : u64
        let s_3_5: u64 = 1;
        // D s_3_6: bit-extract s_3_4 s_3_2 s_3_5
        let s_3_6: Bits = (Bits::new(
            ((s_3_4) >> (s_3_2)).value(),
            u16::try_from(s_3_5).unwrap(),
        ));
        // D s_3_7: cast reint s_3_6 -> u8
        let s_3_7: bool = ((s_3_6.value()) != 0);
        // C s_3_8: const #0s : i
        let s_3_8: i128 = 0;
        // C s_3_9: const #0u : u64
        let s_3_9: u64 = 0;
        // D s_3_10: cast zx s_3_7 -> u64
        let s_3_10: u64 = (s_3_7 as u64);
        // C s_3_11: const #1u : u64
        let s_3_11: u64 = 1;
        // D s_3_12: and s_3_10 s_3_11
        let s_3_12: u64 = ((s_3_10) & (s_3_11));
        // D s_3_13: cmp-eq s_3_12 s_3_11
        let s_3_13: bool = ((s_3_12) == (s_3_11));
        // D s_3_14: lsl s_3_10 s_3_8
        let s_3_14: u64 = s_3_10 << s_3_8;
        // D s_3_15: or s_3_9 s_3_14
        let s_3_15: u64 = ((s_3_9) | (s_3_14));
        // D s_3_16: cmpl s_3_14
        let s_3_16: u64 = !s_3_14;
        // D s_3_17: and s_3_9 s_3_16
        let s_3_17: u64 = ((s_3_9) & (s_3_16));
        // D s_3_18: select s_3_13 s_3_15 s_3_17
        let s_3_18: u64 = if s_3_13 { s_3_15 } else { s_3_17 };
        // D s_3_19: cast trunc s_3_18 -> u8
        let s_3_19: bool = ((s_3_18) != 0);
        // D s_3_20: cast zx s_3_19 -> bv
        let s_3_20: Bits = Bits::new(s_3_19 as u128, 1u16);
        // C s_3_21: const #1u : u8
        let s_3_21: bool = true;
        // C s_3_22: cast zx s_3_21 -> bv
        let s_3_22: Bits = Bits::new(s_3_21 as u128, 1u16);
        // D s_3_23: cmp-eq s_3_20 s_3_22
        let s_3_23: bool = ((s_3_20) == (s_3_22));
        // D s_3_24: write-var pl1_match <= s_3_23
        fn_state.pl1_match = s_3_23;
        // C s_3_25: const #1s : i
        let s_3_25: i128 = 1;
        // D s_3_26: read-var pxc:u8
        let s_3_26: u8 = fn_state.pxc;
        // D s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 2u16);
        // C s_3_28: const #1u : u64
        let s_3_28: u64 = 1;
        // D s_3_29: bit-extract s_3_27 s_3_25 s_3_28
        let s_3_29: Bits = (Bits::new(
            ((s_3_27) >> (s_3_25)).value(),
            u16::try_from(s_3_28).unwrap(),
        ));
        // D s_3_30: cast reint s_3_29 -> u8
        let s_3_30: bool = ((s_3_29.value()) != 0);
        // C s_3_31: const #0s : i
        let s_3_31: i128 = 0;
        // C s_3_32: const #0u : u64
        let s_3_32: u64 = 0;
        // D s_3_33: cast zx s_3_30 -> u64
        let s_3_33: u64 = (s_3_30 as u64);
        // C s_3_34: const #1u : u64
        let s_3_34: u64 = 1;
        // D s_3_35: and s_3_33 s_3_34
        let s_3_35: u64 = ((s_3_33) & (s_3_34));
        // D s_3_36: cmp-eq s_3_35 s_3_34
        let s_3_36: bool = ((s_3_35) == (s_3_34));
        // D s_3_37: lsl s_3_33 s_3_31
        let s_3_37: u64 = s_3_33 << s_3_31;
        // D s_3_38: or s_3_32 s_3_37
        let s_3_38: u64 = ((s_3_32) | (s_3_37));
        // D s_3_39: cmpl s_3_37
        let s_3_39: u64 = !s_3_37;
        // D s_3_40: and s_3_32 s_3_39
        let s_3_40: u64 = ((s_3_32) & (s_3_39));
        // D s_3_41: select s_3_36 s_3_38 s_3_40
        let s_3_41: u64 = if s_3_36 { s_3_38 } else { s_3_40 };
        // D s_3_42: cast trunc s_3_41 -> u8
        let s_3_42: bool = ((s_3_41) != 0);
        // D s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 1u16);
        // C s_3_44: const #1u : u8
        let s_3_44: bool = true;
        // C s_3_45: cast zx s_3_44 -> bv
        let s_3_45: Bits = Bits::new(s_3_44 as u128, 1u16);
        // D s_3_46: cmp-eq s_3_43 s_3_45
        let s_3_46: bool = ((s_3_43) == (s_3_45));
        // D s_3_47: write-var pl0_match <= s_3_46
        fn_state.pl0_match = s_3_46;
        // D s_3_48: read-var isbreakpnt:u8
        let s_3_48: bool = fn_state.isbreakpnt;
        // N s_3_49: branch s_3_48 b69 b4
        if s_3_48 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#29927 <= s_4_0
        fn_state.gs_29927 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#29927:u8
        let s_5_0: bool = fn_state.gs_29927;
        // N s_5_1: branch s_5_0 b68 b6
        if s_5_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#29928 <= s_6_0
        fn_state.gs_29928 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#29928:u8
        let s_7_0: bool = fn_state.gs_29928;
        // N s_7_1: branch s_7_0 b67 b8
        if s_7_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#29929 <= s_8_0
        fn_state.gs_29929 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#29929:u8
        let s_9_0: bool = fn_state.gs_29929;
        // N s_9_1: branch s_9_0 b60 b10
        if s_9_0 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var accdesc.8:struct
        let s_10_0: u8 = fn_state.accdesc._8;
        // D s_10_1: write-var ga#23152 <= s_10_0
        fn_state.ga_23152 = s_10_0;
        // D s_10_2: read-var ga#23152:u8
        let s_10_2: u8 = fn_state.ga_23152;
        // D s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 2u16);
        // C s_10_4: const #424u : u32
        let s_10_4: u32 = 424;
        // D s_10_5: read-reg s_10_4:u8
        let s_10_5: u8 = {
            let value = state.read_register::<u8>(s_10_4 as isize);
            tracer.read_register(s_10_4 as isize, value);
            value
        };
        // D s_10_6: cast zx s_10_5 -> bv
        let s_10_6: Bits = Bits::new(s_10_5 as u128, 2u16);
        // D s_10_7: cmp-eq s_10_3 s_10_6
        let s_10_7: bool = ((s_10_3) == (s_10_6));
        // D s_10_8: not s_10_7
        let s_10_8: bool = !s_10_7;
        // N s_10_9: branch s_10_8 b53 b11
        if s_10_8 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var pl1_match:u8
        let s_11_0: bool = fn_state.pl1_match;
        // D s_11_1: write-var priv_match <= s_11_0
        fn_state.priv_match = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var ssc:u8
        let s_13_0: u8 = fn_state.ssc;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 2u16);
        // C s_13_2: const #0u : u8
        let s_13_2: u8 = 0;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 2u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: not s_13_4
        let s_13_5: bool = !s_13_4;
        // N s_13_6: branch s_13_5 b43 b14
        if s_13_5 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var ss_match <= s_14_0
        fn_state.ss_match = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var linked_match <= s_15_0
        fn_state.linked_match = s_15_0;
        // D s_15_2: read-var linked:u8
        let s_15_2: bool = fn_state.linked;
        // N s_15_3: branch s_15_2 b28 b16
        if s_15_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var priv_match:u8
        let s_17_0: bool = fn_state.priv_match;
        // N s_17_1: branch s_17_0 b27 b18
        if s_17_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#29973 <= s_18_0
        fn_state.gs_29973 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var gs#29973:u8
        let s_19_0: bool = fn_state.gs_29973;
        // N s_19_1: branch s_19_0 b23 b20
        if s_19_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#29975 <= s_20_0
        fn_state.gs_29975 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var gs#29975:u8
        let s_21_0: bool = fn_state.gs_29975;
        // D s_21_1: write-var return_value <= s_21_0
        fn_state.return_value = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_22_0: read-var return_value:u8
        let s_22_0: bool = fn_state.return_value;
        // N s_22_1: return s_22_0
        return s_22_0;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var linked:u8
        let s_23_0: bool = fn_state.linked;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // N s_23_2: branch s_23_1 b26 b24
        if s_23_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var linked_match:u8
        let s_24_0: bool = fn_state.linked_match;
        // D s_24_1: write-var gs#29974 <= s_24_0
        fn_state.gs_29974 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var gs#29974:u8
        let s_25_0: bool = fn_state.gs_29974;
        // D s_25_1: write-var gs#29975 <= s_25_0
        fn_state.gs_29975 = s_25_0;
        // N s_25_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#29974 <= s_26_0
        fn_state.gs_29974 = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var ss_match:u8
        let s_27_0: bool = fn_state.ss_match;
        // D s_27_1: write-var gs#29973 <= s_27_0
        fn_state.gs_29973 = s_27_0;
        // N s_27_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var linked_n:i
        let s_28_0: i128 = fn_state.linked_n;
        // D s_28_1: call IsContextMatchingBreakpoint(s_28_0)
        let s_28_1: bool = IsContextMatchingBreakpoint(state, tracer, s_28_0);
        // D s_28_2: not s_28_1
        let s_28_2: bool = !s_28_1;
        // N s_28_3: branch s_28_2 b31 b29
        if s_28_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_29_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var linked_n:i
        let s_30_0: i128 = fn_state.linked_n;
        // C s_30_1: const #32s : i64
        let s_30_1: i64 = 32;
        // C s_30_2: cast zx s_30_1 -> i
        let s_30_2: i128 = (i128::try_from(s_30_1).unwrap());
        // S s_30_3: call __UNKNOWN_bits(s_30_2)
        let s_30_3: Bits = u__UNKNOWN_bits(state, tracer, s_30_2);
        // S s_30_4: cast reint s_30_3 -> u32
        let s_30_4: u32 = (s_30_3.value() as u32);
        // C s_30_5: const #1u : u8
        let s_30_5: bool = true;
        // D s_30_6: call AArch32_BreakpointValueMatch(s_30_0, s_30_4, s_30_5)
        let s_30_6: ProductType8b847afc727d5818 = AArch32_BreakpointValueMatch(
            state,
            tracer,
            s_30_0,
            s_30_4,
            s_30_5,
        );
        // D s_30_7: write-var ga#23164 <= s_30_6
        fn_state.ga_23164 = s_30_6;
        // D s_30_8: read-var ga#23164.0:struct
        let s_30_8: bool = fn_state.ga_23164._0;
        // D s_30_9: write-var linked_match <= s_30_8
        fn_state.linked_match = s_30_8;
        // N s_30_10: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call ContextMatchingBreakpointRange(s_31_0)
        let s_31_1: ProductTypec716851b6df2cc69 = ContextMatchingBreakpointRange(
            state,
            tracer,
            s_31_0,
        );
        // D s_31_2: write-var ga#23159 <= s_31_1
        fn_state.ga_23159 = s_31_1;
        // D s_31_3: read-var ga#23159.0:struct
        let s_31_3: i128 = fn_state.ga_23159._0;
        // D s_31_4: read-var ga#23159.1:struct
        let s_31_4: i128 = fn_state.ga_23159._1;
        // C s_31_5: const #35u : u32
        let s_31_5: u32 = 35;
        // D s_31_6: call ConstrainUnpredictableInteger(s_31_3, s_31_4, s_31_5)
        let s_31_6: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_31_3,
            s_31_4,
            s_31_5,
        );
        // D s_31_7: write-var ga#23160 <= s_31_6
        fn_state.ga_23160 = s_31_6;
        // D s_31_8: read-var ga#23160.0:struct
        let s_31_8: u32 = fn_state.ga_23160._0;
        // D s_31_9: read-var ga#23160.1:struct
        let s_31_9: i128 = fn_state.ga_23160._1;
        // D s_31_10: write-var c <= s_31_8
        fn_state.c = s_31_8;
        // D s_31_11: write-var linked_n <= s_31_9
        fn_state.linked_n = s_31_9;
        // D s_31_12: read-var c:u32
        let s_31_12: u32 = fn_state.c;
        // C s_31_13: const #7u : u32
        let s_31_13: u32 = 7;
        // D s_31_14: cmp-eq s_31_12 s_31_13
        let s_31_14: bool = ((s_31_12) == (s_31_13));
        // N s_31_15: branch s_31_14 b42 b32
        if s_31_14 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var c:u32
        let s_32_0: u32 = fn_state.c;
        // C s_32_1: const #0u : u32
        let s_32_1: u32 = 0;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // N s_32_3: branch s_32_2 b41 b33
        if s_32_2 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var c:u32
        let s_33_0: u32 = fn_state.c;
        // C s_33_1: const #1u : u32
        let s_33_1: u32 = 1;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // D s_33_3: write-var gs#29958 <= s_33_2
        fn_state.gs_29958 = s_33_2;
        // N s_33_4: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var gs#29958:u8
        let s_34_0: bool = fn_state.gs_29958;
        // D s_34_1: write-var gs#29959 <= s_34_0
        fn_state.gs_29959 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var gs#29959:u8
        let s_35_0: bool = fn_state.gs_29959;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // C s_35_2: const #7u : u32
        let s_35_2: u32 = 7;
        // D s_35_3: read-var c:u32
        let s_35_3: u32 = fn_state.c;
        // D s_35_4: cmp-eq s_35_2 s_35_3
        let s_35_4: bool = ((s_35_2) == (s_35_3));
        // D s_35_5: not s_35_4
        let s_35_5: bool = !s_35_4;
        // N s_35_6: branch s_35_5 b37 b36
        if s_35_5 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var return_value <= s_36_0
        fn_state.return_value = s_36_0;
        // N s_36_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #0u : u32
        let s_37_0: u32 = 0;
        // D s_37_1: read-var c:u32
        let s_37_1: u32 = fn_state.c;
        // D s_37_2: cmp-eq s_37_0 s_37_1
        let s_37_2: bool = ((s_37_0) == (s_37_1));
        // D s_37_3: not s_37_2
        let s_37_3: bool = !s_37_2;
        // N s_37_4: branch s_37_3 b40 b38
        if s_37_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var linked <= s_38_0
        fn_state.linked = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_39_0: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_40_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#29958 <= s_41_0
        fn_state.gs_29958 = s_41_0;
        // N s_41_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#29959 <= s_42_0
        fn_state.gs_29959 = s_42_0;
        // N s_42_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_43_0: read-var ssc:u8
        let s_43_0: u8 = fn_state.ssc;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 2u16);
        // C s_43_2: const #1u : u8
        let s_43_2: u8 = 1;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 2u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: not s_43_4
        let s_43_5: bool = !s_43_4;
        // N s_43_6: branch s_43_5 b45 b44
        if s_43_5 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var accdesc.25:struct
        let s_44_0: u32 = fn_state.accdesc._25;
        // C s_44_1: const #0u : u32
        let s_44_1: u32 = 0;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: write-var ss_match <= s_44_2
        fn_state.ss_match = s_44_2;
        // N s_44_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var ssc:u8
        let s_45_0: u8 = fn_state.ssc;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 2u16);
        // C s_45_2: const #2u : u8
        let s_45_2: u8 = 2;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 2u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: not s_45_4
        let s_45_5: bool = !s_45_4;
        // N s_45_6: branch s_45_5 b47 b46
        if s_45_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var accdesc.25:struct
        let s_46_0: u32 = fn_state.accdesc._25;
        // C s_46_1: const #3u : u32
        let s_46_1: u32 = 3;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: write-var ss_match <= s_46_2
        fn_state.ss_match = s_46_2;
        // N s_46_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var ssc:u8
        let s_47_0: u8 = fn_state.ssc;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 2u16);
        // C s_47_2: const #3u : u8
        let s_47_2: u8 = 3;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 2u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // N s_47_6: branch s_47_5 b52 b48
        if s_47_5 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_48_0: read-var hmc:u8
        let s_48_0: bool = fn_state.hmc;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // N s_48_5: branch s_48_4 b51 b49
        if s_48_4 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_49_0: read-var accdesc.25:struct
        let s_49_0: u32 = fn_state.accdesc._25;
        // C s_49_1: const #3u : u32
        let s_49_1: u32 = 3;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: write-var gs#29943 <= s_49_2
        fn_state.gs_29943 = s_49_2;
        // N s_49_4: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_50_0: read-var gs#29943:u8
        let s_50_0: bool = fn_state.gs_29943;
        // D s_50_1: write-var ss_match <= s_50_0
        fn_state.ss_match = s_50_0;
        // N s_50_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_51_0: const #1u : u8
        let s_51_0: bool = true;
        // D s_51_1: write-var gs#29943 <= s_51_0
        fn_state.gs_29943 = s_51_0;
        // N s_51_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_52_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_53_0: read-var ga#23152:u8
        let s_53_0: u8 = fn_state.ga_23152;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 2u16);
        // C s_53_2: const #432u : u32
        let s_53_2: u32 = 432;
        // D s_53_3: read-reg s_53_2:u8
        let s_53_3: u8 = {
            let value = state.read_register::<u8>(s_53_2 as isize);
            tracer.read_register(s_53_2 as isize, value);
            value
        };
        // D s_53_4: cast zx s_53_3 -> bv
        let s_53_4: Bits = Bits::new(s_53_3 as u128, 2u16);
        // D s_53_5: cmp-eq s_53_1 s_53_4
        let s_53_5: bool = ((s_53_1) == (s_53_4));
        // D s_53_6: not s_53_5
        let s_53_6: bool = !s_53_5;
        // N s_53_7: branch s_53_6 b55 b54
        if s_53_6 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var pl2_match:u8
        let s_54_0: bool = fn_state.pl2_match;
        // D s_54_1: write-var priv_match <= s_54_0
        fn_state.priv_match = s_54_0;
        // N s_54_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_55_0: read-var ga#23152:u8
        let s_55_0: u8 = fn_state.ga_23152;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 2u16);
        // C s_55_2: const #440u : u32
        let s_55_2: u32 = 440;
        // D s_55_3: read-reg s_55_2:u8
        let s_55_3: u8 = {
            let value = state.read_register::<u8>(s_55_2 as isize);
            tracer.read_register(s_55_2 as isize, value);
            value
        };
        // D s_55_4: cast zx s_55_3 -> bv
        let s_55_4: Bits = Bits::new(s_55_3 as u128, 2u16);
        // D s_55_5: cmp-eq s_55_1 s_55_4
        let s_55_5: bool = ((s_55_1) == (s_55_4));
        // D s_55_6: not s_55_5
        let s_55_6: bool = !s_55_5;
        // N s_55_7: branch s_55_6 b57 b56
        if s_55_6 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var pl1_match:u8
        let s_56_0: bool = fn_state.pl1_match;
        // D s_56_1: write-var priv_match <= s_56_0
        fn_state.priv_match = s_56_0;
        // N s_56_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_57_0: read-var ga#23152:u8
        let s_57_0: u8 = fn_state.ga_23152;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 2u16);
        // C s_57_2: const #448u : u32
        let s_57_2: u32 = 448;
        // D s_57_3: read-reg s_57_2:u8
        let s_57_3: u8 = {
            let value = state.read_register::<u8>(s_57_2 as isize);
            tracer.read_register(s_57_2 as isize, value);
            value
        };
        // D s_57_4: cast zx s_57_3 -> bv
        let s_57_4: Bits = Bits::new(s_57_3 as u128, 2u16);
        // D s_57_5: cmp-eq s_57_1 s_57_4
        let s_57_5: bool = ((s_57_1) == (s_57_4));
        // D s_57_6: not s_57_5
        let s_57_6: bool = !s_57_5;
        // N s_57_7: branch s_57_6 b59 b58
        if s_57_6 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var pl0_match:u8
        let s_58_0: bool = fn_state.pl0_match;
        // D s_58_1: write-var priv_match <= s_58_0
        fn_state.priv_match = s_58_0;
        // N s_58_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_59_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_60_0: const #16983u : u32
        let s_60_0: u32 = 16983;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: cast zx s_60_1 -> bv
        let s_60_2: Bits = Bits::new(s_60_1 as u128, 5u16);
        // C s_60_3: const #352u : u32
        let s_60_3: u32 = 352;
        // D s_60_4: read-reg s_60_3:u8
        let s_60_4: u8 = {
            let value = state.read_register::<u8>(s_60_3 as isize);
            tracer.read_register(s_60_3 as isize, value);
            value
        };
        // D s_60_5: cast zx s_60_4 -> bv
        let s_60_5: Bits = Bits::new(s_60_4 as u128, 5u16);
        // D s_60_6: cmp-eq s_60_2 s_60_5
        let s_60_6: bool = ((s_60_2) == (s_60_5));
        // N s_60_7: branch s_60_6 b66 b61
        if s_60_6 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_61_0: const #16983u : u32
        let s_61_0: u32 = 16983;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: cast zx s_61_1 -> bv
        let s_61_2: Bits = Bits::new(s_61_1 as u128, 5u16);
        // C s_61_3: const #376u : u32
        let s_61_3: u32 = 376;
        // D s_61_4: read-reg s_61_3:u8
        let s_61_4: u8 = {
            let value = state.read_register::<u8>(s_61_3 as isize);
            tracer.read_register(s_61_3 as isize, value);
            value
        };
        // D s_61_5: cast zx s_61_4 -> bv
        let s_61_5: Bits = Bits::new(s_61_4 as u128, 5u16);
        // D s_61_6: cmp-eq s_61_2 s_61_5
        let s_61_6: bool = ((s_61_2) == (s_61_5));
        // N s_61_7: branch s_61_6 b65 b62
        if s_61_6 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_62_0: const #16983u : u32
        let s_62_0: u32 = 16983;
        // D s_62_1: read-reg s_62_0:u8
        let s_62_1: u8 = {
            let value = state.read_register::<u8>(s_62_0 as isize);
            tracer.read_register(s_62_0 as isize, value);
            value
        };
        // D s_62_2: cast zx s_62_1 -> bv
        let s_62_2: Bits = Bits::new(s_62_1 as u128, 5u16);
        // C s_62_3: const #416u : u32
        let s_62_3: u32 = 416;
        // D s_62_4: read-reg s_62_3:u8
        let s_62_4: u8 = {
            let value = state.read_register::<u8>(s_62_3 as isize);
            tracer.read_register(s_62_3 as isize, value);
            value
        };
        // D s_62_5: cast zx s_62_4 -> bv
        let s_62_5: Bits = Bits::new(s_62_4 as u128, 5u16);
        // D s_62_6: cmp-eq s_62_2 s_62_5
        let s_62_6: bool = ((s_62_2) == (s_62_5));
        // D s_62_7: write-var gs#29937 <= s_62_6
        fn_state.gs_29937 = s_62_6;
        // N s_62_8: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var gs#29937:u8
        let s_63_0: bool = fn_state.gs_29937;
        // D s_63_1: write-var gs#29938 <= s_63_0
        fn_state.gs_29938 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_64_0: read-var gs#29938:u8
        let s_64_0: bool = fn_state.gs_29938;
        // D s_64_1: write-var priv_match <= s_64_0
        fn_state.priv_match = s_64_0;
        // N s_64_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#29937 <= s_65_0
        fn_state.gs_29937 = s_65_0;
        // N s_65_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_66_0: const #1u : u8
        let s_66_0: bool = true;
        // D s_66_1: write-var gs#29938 <= s_66_0
        fn_state.gs_29938 = s_66_0;
        // N s_66_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_67_0: read-var ssc:u8
        let s_67_0: u8 = fn_state.ssc;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 2u16);
        // C s_67_2: const #3u : u8
        let s_67_2: u8 = 3;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 2u16);
        // D s_67_4: cmp-ne s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) != (s_67_3));
        // D s_67_5: write-var gs#29929 <= s_67_4
        fn_state.gs_29929 = s_67_4;
        // N s_67_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_68_0: read-var pxc:u8
        let s_68_0: u8 = fn_state.pxc;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 2u16);
        // C s_68_2: const #0u : u8
        let s_68_2: u8 = 0;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 2u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#29928 <= s_68_4
        fn_state.gs_29928 = s_68_4;
        // N s_68_6: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_69_0: read-var hmc:u8
        let s_69_0: bool = fn_state.hmc;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #0u : u8
        let s_69_2: bool = false;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#29927 <= s_69_4
        fn_state.gs_29927 = s_69_4;
        // N s_69_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_70_0: read-var hmc:u8
        let s_70_0: bool = fn_state.hmc;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // N s_70_5: branch s_70_4 b76 b71
        if s_70_4 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#29920 <= s_71_0
        fn_state.gs_29920 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_72_0: read-var gs#29920:u8
        let s_72_0: bool = fn_state.gs_29920;
        // N s_72_1: branch s_72_0 b75 b73
        if s_72_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_73_0: read-var ssc:u8
        let s_73_0: u8 = fn_state.ssc;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 2u16);
        // C s_73_2: const #3u : u8
        let s_73_2: u8 = 3;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 2u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#29921 <= s_73_4
        fn_state.gs_29921 = s_73_4;
        // N s_73_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_74_0: read-var gs#29921:u8
        let s_74_0: bool = fn_state.gs_29921;
        // D s_74_1: write-var gs#29922 <= s_74_0
        fn_state.gs_29922 = s_74_0;
        // N s_74_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_75_0: const #1u : u8
        let s_75_0: bool = true;
        // D s_75_1: write-var gs#29921 <= s_75_0
        fn_state.gs_29921 = s_75_0;
        // N s_75_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_76_0: read-var ssc:u8
        let s_76_0: u8 = fn_state.ssc;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 2u16);
        // D s_76_2: read-var pxc:u8
        let s_76_2: u8 = fn_state.pxc;
        // D s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 2u16);
        // D s_76_4: cast reint s_76_1 -> u128
        let s_76_4: u128 = (s_76_1.value() as u128);
        // D s_76_5: size-of s_76_1
        let s_76_5: u16 = s_76_1.length();
        // D s_76_6: cast reint s_76_3 -> u128
        let s_76_6: u128 = (s_76_3.value() as u128);
        // D s_76_7: size-of s_76_3
        let s_76_7: u16 = s_76_3.length();
        // D s_76_8: lsl s_76_4 s_76_7
        let s_76_8: u128 = s_76_4 << s_76_7;
        // D s_76_9: or s_76_8 s_76_6
        let s_76_9: u128 = ((s_76_8) | (s_76_6));
        // D s_76_10: add s_76_5 s_76_7
        let s_76_10: u16 = (s_76_5 + s_76_7);
        // D s_76_11: create-bits s_76_9 s_76_10
        let s_76_11: Bits = Bits::new(s_76_9, s_76_10);
        // D s_76_12: cast reint s_76_11 -> u8
        let s_76_12: u8 = (s_76_11.value() as u8);
        // D s_76_13: cast zx s_76_12 -> bv
        let s_76_13: Bits = Bits::new(s_76_12 as u128, 4u16);
        // C s_76_14: const #8u : u8
        let s_76_14: u8 = 8;
        // C s_76_15: cast zx s_76_14 -> bv
        let s_76_15: Bits = Bits::new(s_76_14 as u128, 4u16);
        // D s_76_16: cmp-ne s_76_13 s_76_15
        let s_76_16: bool = ((s_76_13) != (s_76_15));
        // D s_76_17: write-var gs#29920 <= s_76_16
        fn_state.gs_29920 = s_76_16;
        // N s_76_18: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var return_value <= s_77_0
        fn_state.return_value = s_77_0;
        // N s_77_2: jump b22
        return block_22(state, tracer, fn_state);
    }
}
