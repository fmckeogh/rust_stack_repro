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
use ConstrainUnpredictableInteger::*;
use IsContextMatchingBreakpoint::*;
use HaveRME::*;
use AArch64_BreakpointValueMatch::*;
use ContextMatchingBreakpointRange::*;
use common::*;
pub fn AArch64_StateMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ssc_in: u8,
    ssce_in: bool,
    hmc_in: bool,
    pxc_in: u8,
    linked_in: bool,
    linked_n_in: i128,
    isbreakpnt: bool,
    vaddress: u64,
    accdesc: ProductType9878976b5bcce9c9,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        is_linked_mismatch: bool,
        hmc: bool,
        EL2_match: bool,
        ssc: u8,
        ga_12266: ProductType396b95aa74979079,
        ga_12250: u8,
        gs_16483: bool,
        gs_16445: bool,
        EL1_match: bool,
        gs_16486: bool,
        gs_16449: bool,
        ga_12265: ProductTypec716851b6df2cc69,
        linked_match: bool,
        c: u32,
        gs_16452: bool,
        gs_16426: bool,
        linked_nshadow_275: i128,
        ga_12270: ProductType8b847afc727d5818,
        gs_16488: bool,
        gs_16485: bool,
        gs_16448: bool,
        gs_16484: bool,
        linked_n: i128,
        EL3_match: bool,
        gs_16451: bool,
        return_value: bool,
        ga_12247: ProductTypeba129578e5d1bd1b,
        gs_16468: bool,
        gs_16469: bool,
        ss_match: bool,
        EL0_match: bool,
        gs_16487: bool,
        gs_16430: bool,
        gs_16429: bool,
        pxc: u8,
        ga_12249: u8,
        ssce: bool,
        gs_16432: bool,
        linked: bool,
        priv_match: bool,
        gs_16431: bool,
        ssc_in: u8,
        ssce_in: bool,
        hmc_in: bool,
        pxc_in: u8,
        linked_in: bool,
        linked_n_in: i128,
        isbreakpnt: bool,
        vaddress: u64,
        accdesc: ProductType9878976b5bcce9c9,
    }
    let fn_state = FunctionState {
        ssc_in,
        ssce_in,
        hmc_in,
        pxc_in,
        linked_in,
        linked_n_in,
        isbreakpnt,
        vaddress,
        accdesc,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveRME(s_0_0)
        let s_0_1: bool = HaveRME(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b94 b1
        if s_0_2 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var ssc_in:u8
        let s_2_0: u8 = fn_state.ssc_in;
        // D s_2_1: write-var ssc <= s_2_0
        fn_state.ssc = s_2_0;
        // D s_2_2: read-var ssce_in:u8
        let s_2_2: bool = fn_state.ssce_in;
        // D s_2_3: write-var ssce <= s_2_2
        fn_state.ssce = s_2_2;
        // D s_2_4: read-var hmc_in:u8
        let s_2_4: bool = fn_state.hmc_in;
        // D s_2_5: write-var hmc <= s_2_4
        fn_state.hmc = s_2_4;
        // D s_2_6: read-var pxc_in:u8
        let s_2_6: u8 = fn_state.pxc_in;
        // D s_2_7: write-var pxc <= s_2_6
        fn_state.pxc = s_2_6;
        // D s_2_8: read-var linked_in:u8
        let s_2_8: bool = fn_state.linked_in;
        // D s_2_9: write-var linked <= s_2_8
        fn_state.linked = s_2_8;
        // D s_2_10: read-var linked_n_in:i
        let s_2_10: i128 = fn_state.linked_n_in;
        // D s_2_11: write-var linked_n <= s_2_10
        fn_state.linked_n = s_2_10;
        // D s_2_12: read-var ssc:u8
        let s_2_12: u8 = fn_state.ssc;
        // D s_2_13: read-var ssce:u8
        let s_2_13: bool = fn_state.ssce;
        // D s_2_14: read-var hmc:u8
        let s_2_14: bool = fn_state.hmc;
        // D s_2_15: read-var pxc:u8
        let s_2_15: u8 = fn_state.pxc;
        // D s_2_16: read-var isbreakpnt:u8
        let s_2_16: bool = fn_state.isbreakpnt;
        // D s_2_17: call CheckValidStateMatch(s_2_12, s_2_13, s_2_14, s_2_15, s_2_16)
        let s_2_17: ProductTypeba129578e5d1bd1b = CheckValidStateMatch(
            state,
            tracer,
            s_2_12,
            s_2_13,
            s_2_14,
            s_2_15,
            s_2_16,
        );
        // D s_2_18: write-var ga#12247 <= s_2_17
        fn_state.ga_12247 = s_2_17;
        // D s_2_19: read-var ga#12247.0:struct
        let s_2_19: u32 = fn_state.ga_12247._0;
        // D s_2_20: read-var ga#12247.1:struct
        let s_2_20: u8 = fn_state.ga_12247._1;
        // D s_2_21: read-var ga#12247.2:struct
        let s_2_21: bool = fn_state.ga_12247._2;
        // D s_2_22: read-var ga#12247.3:struct
        let s_2_22: bool = fn_state.ga_12247._3;
        // D s_2_23: read-var ga#12247.4:struct
        let s_2_23: u8 = fn_state.ga_12247._4;
        // D s_2_24: write-var c <= s_2_19
        fn_state.c = s_2_19;
        // D s_2_25: write-var ssc <= s_2_20
        fn_state.ssc = s_2_20;
        // D s_2_26: write-var ssce <= s_2_21
        fn_state.ssce = s_2_21;
        // D s_2_27: write-var hmc <= s_2_22
        fn_state.hmc = s_2_22;
        // D s_2_28: write-var pxc <= s_2_23
        fn_state.pxc = s_2_23;
        // D s_2_29: read-var c:u32
        let s_2_29: u32 = fn_state.c;
        // C s_2_30: const #7u : u32
        let s_2_30: u32 = 7;
        // D s_2_31: cmp-eq s_2_29 s_2_30
        let s_2_31: bool = ((s_2_29) == (s_2_30));
        // N s_2_32: branch s_2_31 b93 b3
        if s_2_31 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #424u : u32
        let s_3_0: u32 = 424;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: u8 = {
            let value = state.read_register::<u8>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #2u : u8
        let s_3_2: u8 = 2;
        // D s_3_3: cmp-lt s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) < (s_3_2));
        // N s_3_4: branch s_3_3 b92 b4
        if s_3_3 {
            return block_92(state, tracer, fn_state);
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
        // D s_4_1: write-var gs#16426 <= s_4_0
        fn_state.gs_16426 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#16426:u8
        let s_5_0: bool = fn_state.gs_16426;
        // N s_5_1: branch s_5_0 b91 b6
        if s_5_0 {
            return block_91(state, tracer, fn_state);
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
        // D s_6_1: write-var gs#16429 <= s_6_0
        fn_state.gs_16429 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_7_0: read-var gs#16429:u8
        let s_7_0: bool = fn_state.gs_16429;
        // D s_7_1: write-var EL3_match <= s_7_0
        fn_state.EL3_match = s_7_0;
        // C s_7_2: const #432u : u32
        let s_7_2: u32 = 432;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // C s_7_4: const #2u : u8
        let s_7_4: u8 = 2;
        // D s_7_5: cmp-lt s_7_3 s_7_4
        let s_7_5: bool = ((s_7_3) < (s_7_4));
        // N s_7_6: branch s_7_5 b84 b8
        if s_7_5 {
            return block_84(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#16432 <= s_8_0
        fn_state.gs_16432 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_9_0: read-var gs#16432:u8
        let s_9_0: bool = fn_state.gs_16432;
        // D s_9_1: write-var EL2_match <= s_9_0
        fn_state.EL2_match = s_9_0;
        // C s_9_2: const #0s : i
        let s_9_2: i128 = 0;
        // D s_9_3: read-var pxc:u8
        let s_9_3: u8 = fn_state.pxc;
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // C s_9_5: const #1u : u64
        let s_9_5: u64 = 1;
        // D s_9_6: bit-extract s_9_4 s_9_2 s_9_5
        let s_9_6: Bits = (Bits::new(
            ((s_9_4) >> (s_9_2)).value(),
            u16::try_from(s_9_5).unwrap(),
        ));
        // D s_9_7: cast reint s_9_6 -> u8
        let s_9_7: bool = ((s_9_6.value()) != 0);
        // C s_9_8: const #0s : i
        let s_9_8: i128 = 0;
        // C s_9_9: const #0u : u64
        let s_9_9: u64 = 0;
        // D s_9_10: cast zx s_9_7 -> u64
        let s_9_10: u64 = (s_9_7 as u64);
        // C s_9_11: const #1u : u64
        let s_9_11: u64 = 1;
        // D s_9_12: and s_9_10 s_9_11
        let s_9_12: u64 = ((s_9_10) & (s_9_11));
        // D s_9_13: cmp-eq s_9_12 s_9_11
        let s_9_13: bool = ((s_9_12) == (s_9_11));
        // D s_9_14: lsl s_9_10 s_9_8
        let s_9_14: u64 = s_9_10 << s_9_8;
        // D s_9_15: or s_9_9 s_9_14
        let s_9_15: u64 = ((s_9_9) | (s_9_14));
        // D s_9_16: cmpl s_9_14
        let s_9_16: u64 = !s_9_14;
        // D s_9_17: and s_9_9 s_9_16
        let s_9_17: u64 = ((s_9_9) & (s_9_16));
        // D s_9_18: select s_9_13 s_9_15 s_9_17
        let s_9_18: u64 = if s_9_13 { s_9_15 } else { s_9_17 };
        // D s_9_19: cast trunc s_9_18 -> u8
        let s_9_19: bool = ((s_9_18) != 0);
        // D s_9_20: cast zx s_9_19 -> bv
        let s_9_20: Bits = Bits::new(s_9_19 as u128, 1u16);
        // C s_9_21: const #1u : u8
        let s_9_21: bool = true;
        // C s_9_22: cast zx s_9_21 -> bv
        let s_9_22: Bits = Bits::new(s_9_21 as u128, 1u16);
        // D s_9_23: cmp-eq s_9_20 s_9_22
        let s_9_23: bool = ((s_9_20) == (s_9_22));
        // D s_9_24: write-var EL1_match <= s_9_23
        fn_state.EL1_match = s_9_23;
        // C s_9_25: const #1s : i
        let s_9_25: i128 = 1;
        // D s_9_26: read-var pxc:u8
        let s_9_26: u8 = fn_state.pxc;
        // D s_9_27: cast zx s_9_26 -> bv
        let s_9_27: Bits = Bits::new(s_9_26 as u128, 2u16);
        // C s_9_28: const #1u : u64
        let s_9_28: u64 = 1;
        // D s_9_29: bit-extract s_9_27 s_9_25 s_9_28
        let s_9_29: Bits = (Bits::new(
            ((s_9_27) >> (s_9_25)).value(),
            u16::try_from(s_9_28).unwrap(),
        ));
        // D s_9_30: cast reint s_9_29 -> u8
        let s_9_30: bool = ((s_9_29.value()) != 0);
        // C s_9_31: const #0s : i
        let s_9_31: i128 = 0;
        // C s_9_32: const #0u : u64
        let s_9_32: u64 = 0;
        // D s_9_33: cast zx s_9_30 -> u64
        let s_9_33: u64 = (s_9_30 as u64);
        // C s_9_34: const #1u : u64
        let s_9_34: u64 = 1;
        // D s_9_35: and s_9_33 s_9_34
        let s_9_35: u64 = ((s_9_33) & (s_9_34));
        // D s_9_36: cmp-eq s_9_35 s_9_34
        let s_9_36: bool = ((s_9_35) == (s_9_34));
        // D s_9_37: lsl s_9_33 s_9_31
        let s_9_37: u64 = s_9_33 << s_9_31;
        // D s_9_38: or s_9_32 s_9_37
        let s_9_38: u64 = ((s_9_32) | (s_9_37));
        // D s_9_39: cmpl s_9_37
        let s_9_39: u64 = !s_9_37;
        // D s_9_40: and s_9_32 s_9_39
        let s_9_40: u64 = ((s_9_32) & (s_9_39));
        // D s_9_41: select s_9_36 s_9_38 s_9_40
        let s_9_41: u64 = if s_9_36 { s_9_38 } else { s_9_40 };
        // D s_9_42: cast trunc s_9_41 -> u8
        let s_9_42: bool = ((s_9_41) != 0);
        // D s_9_43: cast zx s_9_42 -> bv
        let s_9_43: Bits = Bits::new(s_9_42 as u128, 1u16);
        // C s_9_44: const #1u : u8
        let s_9_44: bool = true;
        // C s_9_45: cast zx s_9_44 -> bv
        let s_9_45: Bits = Bits::new(s_9_44 as u128, 1u16);
        // D s_9_46: cmp-eq s_9_43 s_9_45
        let s_9_46: bool = ((s_9_43) == (s_9_45));
        // D s_9_47: write-var EL0_match <= s_9_46
        fn_state.EL0_match = s_9_46;
        // D s_9_48: read-var accdesc.8:struct
        let s_9_48: u8 = fn_state.accdesc._8;
        // D s_9_49: write-var ga#12249 <= s_9_48
        fn_state.ga_12249 = s_9_48;
        // D s_9_50: read-var ga#12249:u8
        let s_9_50: u8 = fn_state.ga_12249;
        // D s_9_51: cast zx s_9_50 -> bv
        let s_9_51: Bits = Bits::new(s_9_50 as u128, 2u16);
        // C s_9_52: const #424u : u32
        let s_9_52: u32 = 424;
        // D s_9_53: read-reg s_9_52:u8
        let s_9_53: u8 = {
            let value = state.read_register::<u8>(s_9_52 as isize);
            tracer.read_register(s_9_52 as isize, value);
            value
        };
        // D s_9_54: cast zx s_9_53 -> bv
        let s_9_54: Bits = Bits::new(s_9_53 as u128, 2u16);
        // D s_9_55: cmp-eq s_9_51 s_9_54
        let s_9_55: bool = ((s_9_51) == (s_9_54));
        // D s_9_56: not s_9_55
        let s_9_56: bool = !s_9_55;
        // N s_9_57: branch s_9_56 b77 b10
        if s_9_56 {
            return block_77(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var EL3_match:u8
        let s_10_0: bool = fn_state.EL3_match;
        // D s_10_1: write-var priv_match <= s_10_0
        fn_state.priv_match = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var ssce:u8
        let s_11_0: bool = fn_state.ssce;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // D s_11_2: read-var ssc:u8
        let s_11_2: u8 = fn_state.ssc;
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 2u16);
        // D s_11_4: cast reint s_11_1 -> u128
        let s_11_4: u128 = (s_11_1.value() as u128);
        // D s_11_5: size-of s_11_1
        let s_11_5: u16 = s_11_1.length();
        // D s_11_6: cast reint s_11_3 -> u128
        let s_11_6: u128 = (s_11_3.value() as u128);
        // D s_11_7: size-of s_11_3
        let s_11_7: u16 = s_11_3.length();
        // D s_11_8: lsl s_11_4 s_11_7
        let s_11_8: u128 = s_11_4 << s_11_7;
        // D s_11_9: or s_11_8 s_11_6
        let s_11_9: u128 = ((s_11_8) | (s_11_6));
        // D s_11_10: add s_11_5 s_11_7
        let s_11_10: u16 = (s_11_5 + s_11_7);
        // D s_11_11: create-bits s_11_9 s_11_10
        let s_11_11: Bits = Bits::new(s_11_9, s_11_10);
        // D s_11_12: cast reint s_11_11 -> u8
        let s_11_12: u8 = (s_11_11.value() as u8);
        // D s_11_13: write-var ga#12250 <= s_11_12
        fn_state.ga_12250 = s_11_12;
        // D s_11_14: read-var ga#12250:u8
        let s_11_14: u8 = fn_state.ga_12250;
        // D s_11_15: cast zx s_11_14 -> bv
        let s_11_15: Bits = Bits::new(s_11_14 as u128, 3u16);
        // C s_11_16: const #0u : u8
        let s_11_16: u8 = 0;
        // C s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 3u16);
        // D s_11_18: cmp-eq s_11_15 s_11_17
        let s_11_18: bool = ((s_11_15) == (s_11_17));
        // D s_11_19: not s_11_18
        let s_11_19: bool = !s_11_18;
        // N s_11_20: branch s_11_19 b56 b12
        if s_11_19 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var hmc:u8
        let s_12_0: bool = fn_state.hmc;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #1u : u8
        let s_12_2: bool = true;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // N s_12_5: branch s_12_4 b55 b13
        if s_12_4 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var accdesc.25:struct
        let s_13_0: u32 = fn_state.accdesc._25;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#16445 <= s_13_2
        fn_state.gs_16445 = s_13_2;
        // N s_13_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#16445:u8
        let s_14_0: bool = fn_state.gs_16445;
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
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // D s_15_3: write-var is_linked_mismatch <= s_15_2
        fn_state.is_linked_mismatch = s_15_2;
        // D s_15_4: read-var linked:u8
        let s_15_4: bool = fn_state.linked;
        // N s_15_5: branch s_15_4 b40 b16
        if s_15_4 {
            return block_40(state, tracer, fn_state);
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
        // D s_17_0: read-var linked_n:i
        let s_17_0: i128 = fn_state.linked_n;
        // D s_17_1: write-var linked_nshadow#275 <= s_17_0
        fn_state.linked_nshadow_275 = s_17_0;
        // D s_17_2: read-var linked:u8
        let s_17_2: bool = fn_state.linked;
        // N s_17_3: branch s_17_2 b39 b18
        if s_17_2 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var priv_match:u8
        let s_19_0: bool = fn_state.priv_match;
        // N s_19_1: branch s_19_0 b38 b20
        if s_19_0 {
            return block_38(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#16483 <= s_20_0
        fn_state.gs_16483 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var gs#16483:u8
        let s_21_0: bool = fn_state.gs_16483;
        // N s_21_1: branch s_21_0 b25 b22
        if s_21_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#16488 <= s_22_0
        fn_state.gs_16488 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#16488:u8
        let s_23_0: bool = fn_state.gs_16488;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var return_value:u8
        let s_24_0: bool = fn_state.return_value;
        // N s_24_1: return s_24_0
        return s_24_0;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var linked:u8
        let s_25_0: bool = fn_state.linked;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // N s_25_2: branch s_25_1 b37 b26
        if s_25_1 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var is_linked_mismatch:u8
        let s_26_0: bool = fn_state.is_linked_mismatch;
        // D s_26_1: not s_26_0
        let s_26_1: bool = !s_26_0;
        // N s_26_2: branch s_26_1 b36 b27
        if s_26_1 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#16484 <= s_27_0
        fn_state.gs_16484 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var gs#16484:u8
        let s_28_0: bool = fn_state.gs_16484;
        // D s_28_1: write-var gs#16485 <= s_28_0
        fn_state.gs_16485 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var gs#16485:u8
        let s_29_0: bool = fn_state.gs_16485;
        // N s_29_1: branch s_29_0 b35 b30
        if s_29_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var is_linked_mismatch:u8
        let s_30_0: bool = fn_state.is_linked_mismatch;
        // N s_30_1: branch s_30_0 b34 b31
        if s_30_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#16486 <= s_31_0
        fn_state.gs_16486 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var gs#16486:u8
        let s_32_0: bool = fn_state.gs_16486;
        // D s_32_1: write-var gs#16487 <= s_32_0
        fn_state.gs_16487 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var gs#16487:u8
        let s_33_0: bool = fn_state.gs_16487;
        // D s_33_1: write-var gs#16488 <= s_33_0
        fn_state.gs_16488 = s_33_0;
        // N s_33_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var linked_match:u8
        let s_34_0: bool = fn_state.linked_match;
        // D s_34_1: not s_34_0
        let s_34_1: bool = !s_34_0;
        // D s_34_2: write-var gs#16486 <= s_34_1
        fn_state.gs_16486 = s_34_1;
        // N s_34_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#16487 <= s_35_0
        fn_state.gs_16487 = s_35_0;
        // N s_35_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var linked_match:u8
        let s_36_0: bool = fn_state.linked_match;
        // D s_36_1: write-var gs#16484 <= s_36_0
        fn_state.gs_16484 = s_36_0;
        // N s_36_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#16485 <= s_37_0
        fn_state.gs_16485 = s_37_0;
        // N s_37_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_38_0: read-var ss_match:u8
        let s_38_0: bool = fn_state.ss_match;
        // D s_38_1: write-var gs#16483 <= s_38_0
        fn_state.gs_16483 = s_38_0;
        // N s_38_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: read-var linked_nshadow#275:i
        let s_39_1: i128 = fn_state.linked_nshadow_275;
        // D s_39_2: read-var vaddress:u64
        let s_39_2: u64 = fn_state.vaddress;
        // D s_39_3: read-var isbreakpnt:u8
        let s_39_3: bool = fn_state.isbreakpnt;
        // D s_39_4: call AArch64_BreakpointValueMatch(s_39_1, s_39_2, s_39_0, s_39_3)
        let s_39_4: ProductType8b847afc727d5818 = AArch64_BreakpointValueMatch(
            state,
            tracer,
            s_39_1,
            s_39_2,
            s_39_0,
            s_39_3,
        );
        // D s_39_5: write-var ga#12270 <= s_39_4
        fn_state.ga_12270 = s_39_4;
        // D s_39_6: read-var ga#12270.0:struct
        let s_39_6: bool = fn_state.ga_12270._0;
        // D s_39_7: read-var ga#12270.1:struct
        let s_39_7: bool = fn_state.ga_12270._1;
        // D s_39_8: write-var linked_match <= s_39_6
        fn_state.linked_match = s_39_6;
        // D s_39_9: write-var is_linked_mismatch <= s_39_7
        fn_state.is_linked_mismatch = s_39_7;
        // N s_39_10: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_40_0: read-var linked_n:i
        let s_40_0: i128 = fn_state.linked_n;
        // D s_40_1: call IsContextMatchingBreakpoint(s_40_0)
        let s_40_1: bool = IsContextMatchingBreakpoint(state, tracer, s_40_0);
        // D s_40_2: not s_40_1
        let s_40_2: bool = !s_40_1;
        // N s_40_3: branch s_40_2 b43 b41
        if s_40_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_41_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_42_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call ContextMatchingBreakpointRange(s_43_0)
        let s_43_1: ProductTypec716851b6df2cc69 = ContextMatchingBreakpointRange(
            state,
            tracer,
            s_43_0,
        );
        // D s_43_2: write-var ga#12265 <= s_43_1
        fn_state.ga_12265 = s_43_1;
        // D s_43_3: read-var ga#12265.0:struct
        let s_43_3: i128 = fn_state.ga_12265._0;
        // D s_43_4: read-var ga#12265.1:struct
        let s_43_4: i128 = fn_state.ga_12265._1;
        // C s_43_5: const #35u : u32
        let s_43_5: u32 = 35;
        // D s_43_6: call ConstrainUnpredictableInteger(s_43_3, s_43_4, s_43_5)
        let s_43_6: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_43_3,
            s_43_4,
            s_43_5,
        );
        // D s_43_7: write-var ga#12266 <= s_43_6
        fn_state.ga_12266 = s_43_6;
        // D s_43_8: read-var ga#12266.0:struct
        let s_43_8: u32 = fn_state.ga_12266._0;
        // D s_43_9: read-var ga#12266.1:struct
        let s_43_9: i128 = fn_state.ga_12266._1;
        // D s_43_10: write-var c <= s_43_8
        fn_state.c = s_43_8;
        // D s_43_11: write-var linked_n <= s_43_9
        fn_state.linked_n = s_43_9;
        // D s_43_12: read-var c:u32
        let s_43_12: u32 = fn_state.c;
        // C s_43_13: const #7u : u32
        let s_43_13: u32 = 7;
        // D s_43_14: cmp-eq s_43_12 s_43_13
        let s_43_14: bool = ((s_43_12) == (s_43_13));
        // N s_43_15: branch s_43_14 b54 b44
        if s_43_14 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var c:u32
        let s_44_0: u32 = fn_state.c;
        // C s_44_1: const #0u : u32
        let s_44_1: u32 = 0;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // N s_44_3: branch s_44_2 b53 b45
        if s_44_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var c:u32
        let s_45_0: u32 = fn_state.c;
        // C s_45_1: const #1u : u32
        let s_45_1: u32 = 1;
        // D s_45_2: cmp-eq s_45_0 s_45_1
        let s_45_2: bool = ((s_45_0) == (s_45_1));
        // D s_45_3: write-var gs#16468 <= s_45_2
        fn_state.gs_16468 = s_45_2;
        // N s_45_4: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_46_0: read-var gs#16468:u8
        let s_46_0: bool = fn_state.gs_16468;
        // D s_46_1: write-var gs#16469 <= s_46_0
        fn_state.gs_16469 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var gs#16469:u8
        let s_47_0: bool = fn_state.gs_16469;
        // N s_47_1: assert s_47_0
        let s_47_1: () = assert!(s_47_0);
        // C s_47_2: const #7u : u32
        let s_47_2: u32 = 7;
        // D s_47_3: read-var c:u32
        let s_47_3: u32 = fn_state.c;
        // D s_47_4: cmp-eq s_47_2 s_47_3
        let s_47_4: bool = ((s_47_2) == (s_47_3));
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // N s_47_6: branch s_47_5 b49 b48
        if s_47_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var return_value <= s_48_0
        fn_state.return_value = s_48_0;
        // N s_48_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #0u : u32
        let s_49_0: u32 = 0;
        // D s_49_1: read-var c:u32
        let s_49_1: u32 = fn_state.c;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // N s_49_4: branch s_49_3 b52 b50
        if s_49_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var linked <= s_50_0
        fn_state.linked = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_51_0: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_52_0: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#16468 <= s_53_0
        fn_state.gs_16468 = s_53_0;
        // N s_53_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_54_0: const #1u : u8
        let s_54_0: bool = true;
        // D s_54_1: write-var gs#16469 <= s_54_0
        fn_state.gs_16469 = s_54_0;
        // N s_54_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_55_0: const #1u : u8
        let s_55_0: bool = true;
        // D s_55_1: write-var gs#16445 <= s_55_0
        fn_state.gs_16445 = s_55_0;
        // N s_55_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_56_0: read-var ga#12250:u8
        let s_56_0: u8 = fn_state.ga_12250;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 3u16);
        // C s_56_2: const #1u : u8
        let s_56_2: u8 = 1;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 3u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: not s_56_4
        let s_56_5: bool = !s_56_4;
        // N s_56_6: branch s_56_5 b58 b57
        if s_56_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_57_0: read-var accdesc.25:struct
        let s_57_0: u32 = fn_state.accdesc._25;
        // C s_57_1: const #0u : u32
        let s_57_1: u32 = 0;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: write-var ss_match <= s_57_2
        fn_state.ss_match = s_57_2;
        // N s_57_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_58_0: read-var ga#12250:u8
        let s_58_0: u8 = fn_state.ga_12250;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 3u16);
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 3u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: not s_58_4
        let s_58_5: bool = !s_58_4;
        // N s_58_6: branch s_58_5 b66 b59
        if s_58_5 {
            return block_66(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_59_0: read-var hmc:u8
        let s_59_0: bool = fn_state.hmc;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // N s_59_5: branch s_59_4 b65 b60
        if s_59_4 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#16448 <= s_60_0
        fn_state.gs_16448 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_61_0: read-var gs#16448:u8
        let s_61_0: bool = fn_state.gs_16448;
        // N s_61_1: branch s_61_0 b64 b62
        if s_61_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_62_0: read-var accdesc.25:struct
        let s_62_0: u32 = fn_state.accdesc._25;
        // C s_62_1: const #3u : u32
        let s_62_1: u32 = 3;
        // D s_62_2: cmp-eq s_62_0 s_62_1
        let s_62_2: bool = ((s_62_0) == (s_62_1));
        // D s_62_3: write-var gs#16449 <= s_62_2
        fn_state.gs_16449 = s_62_2;
        // N s_62_4: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_63_0: read-var gs#16449:u8
        let s_63_0: bool = fn_state.gs_16449;
        // D s_63_1: write-var ss_match <= s_63_0
        fn_state.ss_match = s_63_0;
        // N s_63_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_64_0: const #1u : u8
        let s_64_0: bool = true;
        // D s_64_1: write-var gs#16449 <= s_64_0
        fn_state.gs_16449 = s_64_0;
        // N s_64_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_65_0: read-var accdesc.25:struct
        let s_65_0: u32 = fn_state.accdesc._25;
        // C s_65_1: const #1u : u32
        let s_65_1: u32 = 1;
        // D s_65_2: cmp-eq s_65_0 s_65_1
        let s_65_2: bool = ((s_65_0) == (s_65_1));
        // D s_65_3: write-var gs#16448 <= s_65_2
        fn_state.gs_16448 = s_65_2;
        // N s_65_4: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_66_0: read-var ga#12250:u8
        let s_66_0: u8 = fn_state.ga_12250;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 3u16);
        // C s_66_2: const #3u : u8
        let s_66_2: u8 = 3;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 3u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: not s_66_4
        let s_66_5: bool = !s_66_4;
        // N s_66_6: branch s_66_5 b74 b67
        if s_66_5 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_67_0: read-var hmc:u8
        let s_67_0: bool = fn_state.hmc;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // N s_67_5: branch s_67_4 b73 b68
        if s_67_4 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#16451 <= s_68_0
        fn_state.gs_16451 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_69_0: read-var gs#16451:u8
        let s_69_0: bool = fn_state.gs_16451;
        // N s_69_1: branch s_69_0 b72 b70
        if s_69_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_70_0: read-var accdesc.25:struct
        let s_70_0: u32 = fn_state.accdesc._25;
        // C s_70_1: const #3u : u32
        let s_70_1: u32 = 3;
        // D s_70_2: cmp-eq s_70_0 s_70_1
        let s_70_2: bool = ((s_70_0) == (s_70_1));
        // D s_70_3: write-var gs#16452 <= s_70_2
        fn_state.gs_16452 = s_70_2;
        // N s_70_4: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_71_0: read-var gs#16452:u8
        let s_71_0: bool = fn_state.gs_16452;
        // D s_71_1: write-var ss_match <= s_71_0
        fn_state.ss_match = s_71_0;
        // N s_71_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_72_0: const #1u : u8
        let s_72_0: bool = true;
        // D s_72_1: write-var gs#16452 <= s_72_0
        fn_state.gs_16452 = s_72_0;
        // N s_72_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_73_0: read-var accdesc.25:struct
        let s_73_0: u32 = fn_state.accdesc._25;
        // C s_73_1: const #1u : u32
        let s_73_1: u32 = 1;
        // D s_73_2: cmp-eq s_73_0 s_73_1
        let s_73_2: bool = ((s_73_0) == (s_73_1));
        // D s_73_3: write-var gs#16451 <= s_73_2
        fn_state.gs_16451 = s_73_2;
        // N s_73_4: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_74_0: read-var ga#12250:u8
        let s_74_0: u8 = fn_state.ga_12250;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 3u16);
        // C s_74_2: const #5u : u8
        let s_74_2: u8 = 5;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 3u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: not s_74_4
        let s_74_5: bool = !s_74_4;
        // N s_74_6: branch s_74_5 b76 b75
        if s_74_5 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_75_0: read-var accdesc.25:struct
        let s_75_0: u32 = fn_state.accdesc._25;
        // C s_75_1: const #2u : u32
        let s_75_1: u32 = 2;
        // D s_75_2: cmp-eq s_75_0 s_75_1
        let s_75_2: bool = ((s_75_0) == (s_75_1));
        // D s_75_3: write-var ss_match <= s_75_2
        fn_state.ss_match = s_75_2;
        // N s_75_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_76_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_77_0: read-var ga#12249:u8
        let s_77_0: u8 = fn_state.ga_12249;
        // D s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 2u16);
        // C s_77_2: const #432u : u32
        let s_77_2: u32 = 432;
        // D s_77_3: read-reg s_77_2:u8
        let s_77_3: u8 = {
            let value = state.read_register::<u8>(s_77_2 as isize);
            tracer.read_register(s_77_2 as isize, value);
            value
        };
        // D s_77_4: cast zx s_77_3 -> bv
        let s_77_4: Bits = Bits::new(s_77_3 as u128, 2u16);
        // D s_77_5: cmp-eq s_77_1 s_77_4
        let s_77_5: bool = ((s_77_1) == (s_77_4));
        // D s_77_6: not s_77_5
        let s_77_6: bool = !s_77_5;
        // N s_77_7: branch s_77_6 b79 b78
        if s_77_6 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_78_0: read-var EL2_match:u8
        let s_78_0: bool = fn_state.EL2_match;
        // D s_78_1: write-var priv_match <= s_78_0
        fn_state.priv_match = s_78_0;
        // N s_78_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_79_0: read-var ga#12249:u8
        let s_79_0: u8 = fn_state.ga_12249;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 2u16);
        // C s_79_2: const #440u : u32
        let s_79_2: u32 = 440;
        // D s_79_3: read-reg s_79_2:u8
        let s_79_3: u8 = {
            let value = state.read_register::<u8>(s_79_2 as isize);
            tracer.read_register(s_79_2 as isize, value);
            value
        };
        // D s_79_4: cast zx s_79_3 -> bv
        let s_79_4: Bits = Bits::new(s_79_3 as u128, 2u16);
        // D s_79_5: cmp-eq s_79_1 s_79_4
        let s_79_5: bool = ((s_79_1) == (s_79_4));
        // D s_79_6: not s_79_5
        let s_79_6: bool = !s_79_5;
        // N s_79_7: branch s_79_6 b81 b80
        if s_79_6 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_80_0: read-var EL1_match:u8
        let s_80_0: bool = fn_state.EL1_match;
        // D s_80_1: write-var priv_match <= s_80_0
        fn_state.priv_match = s_80_0;
        // N s_80_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_81_0: read-var ga#12249:u8
        let s_81_0: u8 = fn_state.ga_12249;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 2u16);
        // C s_81_2: const #448u : u32
        let s_81_2: u32 = 448;
        // D s_81_3: read-reg s_81_2:u8
        let s_81_3: u8 = {
            let value = state.read_register::<u8>(s_81_2 as isize);
            tracer.read_register(s_81_2 as isize, value);
            value
        };
        // D s_81_4: cast zx s_81_3 -> bv
        let s_81_4: Bits = Bits::new(s_81_3 as u128, 2u16);
        // D s_81_5: cmp-eq s_81_1 s_81_4
        let s_81_5: bool = ((s_81_1) == (s_81_4));
        // D s_81_6: not s_81_5
        let s_81_6: bool = !s_81_5;
        // N s_81_7: branch s_81_6 b83 b82
        if s_81_6 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_82_0: read-var EL0_match:u8
        let s_82_0: bool = fn_state.EL0_match;
        // D s_82_1: write-var priv_match <= s_82_0
        fn_state.priv_match = s_82_0;
        // N s_82_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_83_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_84_0: read-var hmc:u8
        let s_84_0: bool = fn_state.hmc;
        // D s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 1u16);
        // C s_84_2: const #1u : u8
        let s_84_2: bool = true;
        // C s_84_3: cast zx s_84_2 -> bv
        let s_84_3: Bits = Bits::new(s_84_2 as u128, 1u16);
        // D s_84_4: cmp-eq s_84_1 s_84_3
        let s_84_4: bool = ((s_84_1) == (s_84_3));
        // N s_84_5: branch s_84_4 b90 b85
        if s_84_4 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#16430 <= s_85_0
        fn_state.gs_16430 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_86_0: read-var gs#16430:u8
        let s_86_0: bool = fn_state.gs_16430;
        // N s_86_1: branch s_86_0 b89 b87
        if s_86_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_87_0: read-var ssc:u8
        let s_87_0: u8 = fn_state.ssc;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 2u16);
        // C s_87_2: const #3u : u8
        let s_87_2: u8 = 3;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 2u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#16431 <= s_87_4
        fn_state.gs_16431 = s_87_4;
        // N s_87_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_88_0: read-var gs#16431:u8
        let s_88_0: bool = fn_state.gs_16431;
        // D s_88_1: write-var gs#16432 <= s_88_0
        fn_state.gs_16432 = s_88_0;
        // N s_88_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_89_0: const #1u : u8
        let s_89_0: bool = true;
        // D s_89_1: write-var gs#16431 <= s_89_0
        fn_state.gs_16431 = s_89_0;
        // N s_89_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_90_0: read-var ssc:u8
        let s_90_0: u8 = fn_state.ssc;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 2u16);
        // D s_90_2: read-var pxc:u8
        let s_90_2: u8 = fn_state.pxc;
        // D s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 2u16);
        // D s_90_4: cast reint s_90_1 -> u128
        let s_90_4: u128 = (s_90_1.value() as u128);
        // D s_90_5: size-of s_90_1
        let s_90_5: u16 = s_90_1.length();
        // D s_90_6: cast reint s_90_3 -> u128
        let s_90_6: u128 = (s_90_3.value() as u128);
        // D s_90_7: size-of s_90_3
        let s_90_7: u16 = s_90_3.length();
        // D s_90_8: lsl s_90_4 s_90_7
        let s_90_8: u128 = s_90_4 << s_90_7;
        // D s_90_9: or s_90_8 s_90_6
        let s_90_9: u128 = ((s_90_8) | (s_90_6));
        // D s_90_10: add s_90_5 s_90_7
        let s_90_10: u16 = (s_90_5 + s_90_7);
        // D s_90_11: create-bits s_90_9 s_90_10
        let s_90_11: Bits = Bits::new(s_90_9, s_90_10);
        // D s_90_12: cast reint s_90_11 -> u8
        let s_90_12: u8 = (s_90_11.value() as u8);
        // D s_90_13: cast zx s_90_12 -> bv
        let s_90_13: Bits = Bits::new(s_90_12 as u128, 4u16);
        // C s_90_14: const #8u : u8
        let s_90_14: u8 = 8;
        // C s_90_15: cast zx s_90_14 -> bv
        let s_90_15: Bits = Bits::new(s_90_14 as u128, 4u16);
        // D s_90_16: cmp-ne s_90_13 s_90_15
        let s_90_16: bool = ((s_90_13) != (s_90_15));
        // D s_90_17: write-var gs#16430 <= s_90_16
        fn_state.gs_16430 = s_90_16;
        // N s_90_18: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_91_0: const #0s : i
        let s_91_0: i128 = 0;
        // D s_91_1: read-var ssc:u8
        let s_91_1: u8 = fn_state.ssc;
        // D s_91_2: cast zx s_91_1 -> bv
        let s_91_2: Bits = Bits::new(s_91_1 as u128, 2u16);
        // C s_91_3: const #1u : u64
        let s_91_3: u64 = 1;
        // D s_91_4: bit-extract s_91_2 s_91_0 s_91_3
        let s_91_4: Bits = (Bits::new(
            ((s_91_2) >> (s_91_0)).value(),
            u16::try_from(s_91_3).unwrap(),
        ));
        // D s_91_5: cast reint s_91_4 -> u8
        let s_91_5: bool = ((s_91_4.value()) != 0);
        // C s_91_6: const #0s : i
        let s_91_6: i128 = 0;
        // C s_91_7: const #0u : u64
        let s_91_7: u64 = 0;
        // D s_91_8: cast zx s_91_5 -> u64
        let s_91_8: u64 = (s_91_5 as u64);
        // C s_91_9: const #1u : u64
        let s_91_9: u64 = 1;
        // D s_91_10: and s_91_8 s_91_9
        let s_91_10: u64 = ((s_91_8) & (s_91_9));
        // D s_91_11: cmp-eq s_91_10 s_91_9
        let s_91_11: bool = ((s_91_10) == (s_91_9));
        // D s_91_12: lsl s_91_8 s_91_6
        let s_91_12: u64 = s_91_8 << s_91_6;
        // D s_91_13: or s_91_7 s_91_12
        let s_91_13: u64 = ((s_91_7) | (s_91_12));
        // D s_91_14: cmpl s_91_12
        let s_91_14: u64 = !s_91_12;
        // D s_91_15: and s_91_7 s_91_14
        let s_91_15: u64 = ((s_91_7) & (s_91_14));
        // D s_91_16: select s_91_11 s_91_13 s_91_15
        let s_91_16: u64 = if s_91_11 { s_91_13 } else { s_91_15 };
        // D s_91_17: cast trunc s_91_16 -> u8
        let s_91_17: bool = ((s_91_16) != 0);
        // D s_91_18: cast zx s_91_17 -> bv
        let s_91_18: Bits = Bits::new(s_91_17 as u128, 1u16);
        // C s_91_19: const #0u : u8
        let s_91_19: bool = false;
        // C s_91_20: cast zx s_91_19 -> bv
        let s_91_20: Bits = Bits::new(s_91_19 as u128, 1u16);
        // D s_91_21: cmp-eq s_91_18 s_91_20
        let s_91_21: bool = ((s_91_18) == (s_91_20));
        // D s_91_22: write-var gs#16429 <= s_91_21
        fn_state.gs_16429 = s_91_21;
        // N s_91_23: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_92_0: read-var hmc:u8
        let s_92_0: bool = fn_state.hmc;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#16426 <= s_92_4
        fn_state.gs_16426 = s_92_4;
        // N s_92_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var return_value <= s_93_0
        fn_state.return_value = s_93_0;
        // N s_93_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_94_0: read-var ssce_in:u8
        let s_94_0: bool = fn_state.ssce_in;
        // D s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 1u16);
        // C s_94_2: const #0u : u8
        let s_94_2: bool = false;
        // C s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 1u16);
        // D s_94_4: cmp-eq s_94_1 s_94_3
        let s_94_4: bool = ((s_94_1) == (s_94_3));
        // N s_94_5: assert s_94_4
        let s_94_5: () = assert!(s_94_4);
        // N s_94_6: jump b2
        return block_2(state, tracer, fn_state);
    }
}
