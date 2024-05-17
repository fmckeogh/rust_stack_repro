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
use u_get_PMSIDR_EL1_Type_PBT::*;
use Bit::*;
use u__IMPDEF_boolean::*;
use CurrentSecurityState::*;
use IsZero::*;
use StatisticalProfilingEnabled::*;
use Unreachable::*;
use common::*;
pub fn SPEBranch__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    target: Bits,
    branch_type: u32,
    conditional: bool,
    taken_flag: bool,
    is_isb: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u_153: bool,
        gs_3625: bool,
        ga_2229: bool,
        gs_3621: bool,
        ns: bool,
        ga_2209: bool,
        collect_prev_br_isb: bool,
        gs_3628: bool,
        collect_prev_br_exception: bool,
        u_152: bool,
        collect_prev_brshadow_34: bool,
        gs_3624: bool,
        gs_3626: bool,
        ga_2218: u32,
        collect_prev_br: bool,
        nse: bool,
        collect_prev_br_eret: bool,
        ga_2211: bool,
        ga_2203: u32,
        target: Bits,
        branch_type: u32,
        conditional: bool,
        taken_flag: bool,
        is_isb: bool,
    }
    let fn_state = FunctionState {
        target,
        branch_type,
        conditional,
        taken_flag,
        is_isb,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #"SPE prev br on eret" : str
        let s_0_0: &'static str = "SPE prev br on eret";
        // S s_0_1: call __IMPDEF_boolean(s_0_0)
        let s_0_1: bool = u__IMPDEF_boolean(state, tracer, s_0_0);
        // D s_0_2: write-var collect_prev_br_eret <= s_0_1
        fn_state.collect_prev_br_eret = s_0_1;
        // C s_0_3: const #"SPE prev br on exception" : str
        let s_0_3: &'static str = "SPE prev br on exception";
        // S s_0_4: call __IMPDEF_boolean(s_0_3)
        let s_0_4: bool = u__IMPDEF_boolean(state, tracer, s_0_3);
        // D s_0_5: write-var collect_prev_br_exception <= s_0_4
        fn_state.collect_prev_br_exception = s_0_4;
        // C s_0_6: const #"SPE prev br on isb" : str
        let s_0_6: &'static str = "SPE prev br on isb";
        // S s_0_7: call __IMPDEF_boolean(s_0_6)
        let s_0_7: bool = u__IMPDEF_boolean(state, tracer, s_0_6);
        // D s_0_8: write-var collect_prev_br_isb <= s_0_7
        fn_state.collect_prev_br_isb = s_0_7;
        // C s_0_9: const #7u : u32
        let s_0_9: u32 = 7;
        // D s_0_10: read-var branch_type:u32
        let s_0_10: u32 = fn_state.branch_type;
        // D s_0_11: cmp-eq s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) == (s_0_10));
        // D s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // N s_0_13: branch s_0_12 b53 b1
        if s_0_12 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var collect_prev_br_exception:u8
        let s_1_0: bool = fn_state.collect_prev_br_exception;
        // D s_1_1: write-var collect_prev_br <= s_1_0
        fn_state.collect_prev_br = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var collect_prev_br:u8
        let s_2_0: bool = fn_state.collect_prev_br;
        // D s_2_1: write-var collect_prev_brshadow#34 <= s_2_0
        fn_state.collect_prev_brshadow_34 = s_2_0;
        // D s_2_2: read-var taken_flag:u8
        let s_2_2: bool = fn_state.taken_flag;
        // N s_2_3: branch s_2_2 b52 b3
        if s_2_2 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#3624 <= s_3_0
        fn_state.gs_3624 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#3624:u8
        let s_4_0: bool = fn_state.gs_3624;
        // N s_4_1: branch s_4_0 b51 b5
        if s_4_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#3625 <= s_5_0
        fn_state.gs_3625 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#3625:u8
        let s_6_0: bool = fn_state.gs_3625;
        // N s_6_1: branch s_6_0 b50 b7
        if s_6_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#3626 <= s_7_0
        fn_state.gs_3626 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#3626:u8
        let s_8_0: bool = fn_state.gs_3626;
        // N s_8_1: branch s_8_0 b39 b9
        if s_8_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call StatisticalProfilingEnabled(s_10_0)
        let s_10_1: bool = StatisticalProfilingEnabled(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b36 b11
        if s_10_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #22416u : u32
        let s_11_0: u32 = 22416;
        // D s_11_1: read-reg s_11_0:u8
        let s_11_1: bool = {
            let value = state.read_register::<bool>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // N s_11_2: branch s_11_1 b13 b12
        if s_11_1 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var branch_type:u32
        let s_13_0: u32 = fn_state.branch_type;
        // C s_13_1: const #5u : u32
        let s_13_1: u32 = 5;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b35 b14
        if s_13_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var branch_type:u32
        let s_14_0: u32 = fn_state.branch_type;
        // C s_14_1: const #0u : u32
        let s_14_1: u32 = 0;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: write-var gs#3628 <= s_14_2
        fn_state.gs_3628 = s_14_2;
        // N s_14_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#3628:u8
        let s_15_0: bool = fn_state.gs_3628;
        // C s_15_1: const #2u : u8
        let s_15_1: u8 = 2;
        // C s_15_2: const #17136u : u32
        let s_15_2: u32 = 17136;
        // N s_15_3: write-reg s_15_2 <= s_15_1
        let s_15_3: () = {
            state.write_register::<u8>(s_15_2 as isize, s_15_1);
            tracer.write_register(s_15_2 as isize, s_15_1);
        };
        // N s_15_4: branch s_15_0 b34 b16
        if s_15_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var ga#2209 <= s_16_0
        fn_state.ga_2209 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#2209:u8
        let s_17_0: bool = fn_state.ga_2209;
        // D s_17_1: call Bit(s_17_0)
        let s_17_1: bool = Bit(state, tracer, s_17_0);
        // C s_17_2: const #1s : i
        let s_17_2: i128 = 1;
        // C s_17_3: const #13528u : u32
        let s_17_3: u32 = 13528;
        // D s_17_4: read-reg s_17_3:u8
        let s_17_4: u8 = {
            let value = state.read_register::<u8>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: cast zx s_17_4 -> bv
        let s_17_5: Bits = Bits::new(s_17_4 as u128, 8u16);
        // C s_17_6: const #1u : u64
        let s_17_6: u64 = 1;
        // D s_17_7: bit-insert s_17_5 s_17_5 s_17_2 s_17_6
        let s_17_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_5.length(),
            );
            (s_17_5 & mask) | (s_17_5 << s_17_2)
        };
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #13528u : u32
        let s_17_9: u32 = 13528;
        // N s_17_10: write-reg s_17_9 <= s_17_8
        let s_17_10: () = {
            state.write_register::<u8>(s_17_9 as isize, s_17_8);
            tracer.write_register(s_17_9 as isize, s_17_8);
        };
        // D s_17_11: read-var conditional:u8
        let s_17_11: bool = fn_state.conditional;
        // N s_17_12: branch s_17_11 b33 b18
        if s_17_11 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var ga#2211 <= s_18_0
        fn_state.ga_2211 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ga#2211:u8
        let s_19_0: bool = fn_state.ga_2211;
        // D s_19_1: call Bit(s_19_0)
        let s_19_1: bool = Bit(state, tracer, s_19_0);
        // C s_19_2: const #0s : i
        let s_19_2: i128 = 0;
        // C s_19_3: const #13528u : u32
        let s_19_3: u32 = 13528;
        // D s_19_4: read-reg s_19_3:u8
        let s_19_4: u8 = {
            let value = state.read_register::<u8>(s_19_3 as isize);
            tracer.read_register(s_19_3 as isize, value);
            value
        };
        // D s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 8u16);
        // C s_19_6: const #1u : u64
        let s_19_6: u64 = 1;
        // D s_19_7: bit-insert s_19_5 s_19_5 s_19_2 s_19_6
        let s_19_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_19_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_19_5.length(),
            );
            (s_19_5 & mask) | (s_19_5 << s_19_2)
        };
        // D s_19_8: cast reint s_19_7 -> u8
        let s_19_8: u8 = (s_19_7.value() as u8);
        // C s_19_9: const #13528u : u32
        let s_19_9: u32 = 13528;
        // N s_19_10: write-reg s_19_9 <= s_19_8
        let s_19_10: () = {
            state.write_register::<u8>(s_19_9 as isize, s_19_8);
            tracer.write_register(s_19_9 as isize, s_19_8);
        };
        // C s_19_11: const #3u : u32
        let s_19_11: u32 = 3;
        // C s_19_12: const #19040u : u32
        let s_19_12: u32 = 19040;
        // N s_19_13: write-reg s_19_12 <= s_19_11
        let s_19_13: () = {
            state.write_register::<u32>(s_19_12 as isize, s_19_11);
            tracer.write_register(s_19_12 as isize, s_19_11);
        };
        // D s_19_14: read-var taken_flag:u8
        let s_19_14: bool = fn_state.taken_flag;
        // N s_19_15: branch s_19_14 b25 b20
        if s_19_14 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_20_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var taken_flag:u8
        let s_21_0: bool = fn_state.taken_flag;
        // D s_21_1: not s_21_0
        let s_21_1: bool = !s_21_0;
        // N s_21_2: branch s_21_1 b24 b22
        if s_21_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var ga#2229 <= s_22_0
        fn_state.ga_2229 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var ga#2229:u8
        let s_23_0: bool = fn_state.ga_2229;
        // D s_23_1: call Bit(s_23_0)
        let s_23_1: bool = Bit(state, tracer, s_23_0);
        // C s_23_2: const #6s : i
        let s_23_2: i128 = 6;
        // C s_23_3: const #104856u : u32
        let s_23_3: u32 = 104856;
        // D s_23_4: read-reg s_23_3:u64
        let s_23_4: u64 = {
            let value = state.read_register::<u64>(s_23_3 as isize);
            tracer.read_register(s_23_3 as isize, value);
            value
        };
        // D s_23_5: cast zx s_23_4 -> bv
        let s_23_5: Bits = Bits::new(s_23_4 as u128, 64u16);
        // C s_23_6: const #1u : u64
        let s_23_6: u64 = 1;
        // D s_23_7: bit-insert s_23_5 s_23_5 s_23_2 s_23_6
        let s_23_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_23_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_23_5.length(),
            );
            (s_23_5 & mask) | (s_23_5 << s_23_2)
        };
        // D s_23_8: cast reint s_23_7 -> u64
        let s_23_8: u64 = (s_23_7.value() as u64);
        // C s_23_9: const #104856u : u32
        let s_23_9: u32 = 104856;
        // N s_23_10: write-reg s_23_9 <= s_23_8
        let s_23_10: () = {
            state.write_register::<u64>(s_23_9 as isize, s_23_8);
            tracer.write_register(s_23_9 as isize, s_23_8);
        };
        // N s_23_11: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var ga#2229 <= s_24_0
        fn_state.ga_2229 = s_24_0;
        // N s_24_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var target:bv
        let s_25_0: Bits = fn_state.target;
        // D s_25_1: size-of s_25_0
        let s_25_1: u16 = s_25_0.length();
        // D s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // C s_25_3: const #55s : i
        let s_25_3: i128 = 55;
        // D s_25_4: cmp-lt s_25_3 s_25_2
        let s_25_4: bool = ((s_25_3) < (s_25_2));
        // N s_25_5: assert s_25_4
        let s_25_5: () = assert!(s_25_4);
        // C s_25_6: const #13776u : u32
        let s_25_6: u32 = 13776;
        // D s_25_7: read-reg s_25_6:[u64; 32]
        let s_25_7: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_25_6 as isize);
            tracer.read_register(s_25_6 as isize, value);
            value
        };
        // C s_25_8: const #1024u : u32
        let s_25_8: u32 = 1024;
        // D s_25_9: read-reg s_25_8:i64
        let s_25_9: i64 = {
            let value = state.read_register::<i64>(s_25_8 as isize);
            tracer.read_register(s_25_8 as isize, value);
            value
        };
        // D s_25_10: cast zx s_25_9 -> i
        let s_25_10: i128 = (i128::try_from(s_25_9).unwrap());
        // D s_25_11: read-element s_25_7[s_25_10]
        let s_25_11: u64 = s_25_7[(s_25_10) as usize];
        // C s_25_12: const #0s : i
        let s_25_12: i128 = 0;
        // D s_25_13: read-var target:bv
        let s_25_13: Bits = fn_state.target;
        // C s_25_14: const #1s : i64
        let s_25_14: i64 = 1;
        // C s_25_15: cast zx s_25_14 -> i
        let s_25_15: i128 = (i128::try_from(s_25_14).unwrap());
        // C s_25_16: const #55s : i
        let s_25_16: i128 = 55;
        // C s_25_17: add s_25_16 s_25_15
        let s_25_17: i128 = (s_25_16 + s_25_15);
        // D s_25_18: bit-extract s_25_13 s_25_12 s_25_17
        let s_25_18: Bits = (Bits::new(
            ((s_25_13) >> (s_25_12)).value(),
            u16::try_from(s_25_17).unwrap(),
        ));
        // D s_25_19: cast reint s_25_18 -> u56
        let s_25_19: u64 = (s_25_18.value() as u64);
        // C s_25_20: const #0s : i
        let s_25_20: i128 = 0;
        // D s_25_21: cast zx s_25_11 -> bv
        let s_25_21: Bits = Bits::new(s_25_11 as u128, 64u16);
        // D s_25_22: cast zx s_25_19 -> bv
        let s_25_22: Bits = Bits::new(s_25_19 as u128, 56u16);
        // C s_25_23: const #55s : i
        let s_25_23: i128 = 55;
        // C s_25_24: const #1u : u64
        let s_25_24: u64 = 1;
        // C s_25_25: cast zx s_25_24 -> bv
        let s_25_25: Bits = Bits::new(s_25_24 as u128, 64u16);
        // C s_25_26: lsl s_25_25 s_25_23
        let s_25_26: Bits = s_25_25 << s_25_23;
        // C s_25_27: sub s_25_26 s_25_25
        let s_25_27: Bits = ((s_25_26) - (s_25_25));
        // D s_25_28: and s_25_22 s_25_27
        let s_25_28: Bits = ((s_25_22) & (s_25_27));
        // D s_25_29: lsl s_25_28 s_25_20
        let s_25_29: Bits = s_25_28 << s_25_20;
        // C s_25_30: lsl s_25_27 s_25_20
        let s_25_30: Bits = s_25_27 << s_25_20;
        // C s_25_31: cmpl s_25_30
        let s_25_31: Bits = !s_25_30;
        // D s_25_32: and s_25_21 s_25_31
        let s_25_32: Bits = ((s_25_21) & (s_25_31));
        // D s_25_33: or s_25_32 s_25_29
        let s_25_33: Bits = ((s_25_32) | (s_25_29));
        // D s_25_34: cast reint s_25_33 -> u64
        let s_25_34: u64 = (s_25_33.value() as u64);
        // C s_25_35: const #13776u : u32
        let s_25_35: u32 = 13776;
        // D s_25_36: read-reg s_25_35:[u64; 32]
        let s_25_36: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_25_35 as isize);
            tracer.read_register(s_25_35 as isize, value);
            value
        };
        // C s_25_37: const #1024u : u32
        let s_25_37: u32 = 1024;
        // D s_25_38: read-reg s_25_37:i64
        let s_25_38: i64 = {
            let value = state.read_register::<i64>(s_25_37 as isize);
            tracer.read_register(s_25_37 as isize, value);
            value
        };
        // D s_25_39: cast zx s_25_38 -> i
        let s_25_39: i128 = (i128::try_from(s_25_38).unwrap());
        // D s_25_40: mutate-element s_25_36[s_25_39] <= s_25_34
        let s_25_40: [u64; 32usize] = {
            let mut local = s_25_36.clone();
            local[(s_25_39) as usize] = s_25_34;
            local
        };
        // D s_25_41: cast cvt s_25_40 -> [u64; 0]
        let s_25_41: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_25_40);
        // D s_25_42: cast cvt s_25_41 -> [u64; 32]
        let s_25_42: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_25_41);
            buf
        };
        // C s_25_43: const #13776u : u32
        let s_25_43: u32 = 13776;
        // N s_25_44: write-reg s_25_43 <= s_25_42
        let s_25_44: () = {
            state.write_register::<[u64; 32usize]>(s_25_43 as isize, s_25_42);
            tracer.write_register(s_25_43 as isize, s_25_42);
        };
        // C s_25_45: const #() : ()
        let s_25_45: () = ();
        // S s_25_46: call CurrentSecurityState(s_25_45)
        let s_25_46: u32 = CurrentSecurityState(state, tracer, s_25_45);
        // D s_25_47: write-var ga#2218 <= s_25_46
        fn_state.ga_2218 = s_25_46;
        // C s_25_48: const #3u : u32
        let s_25_48: u32 = 3;
        // D s_25_49: read-var ga#2218:u32
        let s_25_49: u32 = fn_state.ga_2218;
        // D s_25_50: cmp-eq s_25_48 s_25_49
        let s_25_50: bool = ((s_25_48) == (s_25_49));
        // D s_25_51: not s_25_50
        let s_25_51: bool = !s_25_50;
        // N s_25_52: branch s_25_51 b28 b26
        if s_25_51 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var u#152 <= s_26_0
        fn_state.u_152 = s_26_0;
        // C s_26_2: const #0u : u8
        let s_26_2: bool = false;
        // D s_26_3: write-var u#153 <= s_26_2
        fn_state.u_153 = s_26_2;
        // N s_26_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #13776u : u32
        let s_27_0: u32 = 13776;
        // D s_27_1: read-reg s_27_0:[u64; 32]
        let s_27_1: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #1024u : u32
        let s_27_2: u32 = 1024;
        // D s_27_3: read-reg s_27_2:i64
        let s_27_3: i64 = {
            let value = state.read_register::<i64>(s_27_2 as isize);
            tracer.read_register(s_27_2 as isize, value);
            value
        };
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: read-element s_27_1[s_27_4]
        let s_27_5: u64 = s_27_1[(s_27_4) as usize];
        // D s_27_6: read-var u#152:u8
        let s_27_6: bool = fn_state.u_152;
        // D s_27_7: call Bit(s_27_6)
        let s_27_7: bool = Bit(state, tracer, s_27_6);
        // C s_27_8: const #63s : i
        let s_27_8: i128 = 63;
        // D s_27_9: cast zx s_27_5 -> bv
        let s_27_9: Bits = Bits::new(s_27_5 as u128, 64u16);
        // C s_27_10: const #1u : u64
        let s_27_10: u64 = 1;
        // D s_27_11: bit-insert s_27_9 s_27_9 s_27_8 s_27_10
        let s_27_11: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_10 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_9.length(),
            );
            (s_27_9 & mask) | (s_27_9 << s_27_8)
        };
        // D s_27_12: cast reint s_27_11 -> u64
        let s_27_12: u64 = (s_27_11.value() as u64);
        // C s_27_13: const #13776u : u32
        let s_27_13: u32 = 13776;
        // D s_27_14: read-reg s_27_13:[u64; 32]
        let s_27_14: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_13 as isize);
            tracer.read_register(s_27_13 as isize, value);
            value
        };
        // C s_27_15: const #1024u : u32
        let s_27_15: u32 = 1024;
        // D s_27_16: read-reg s_27_15:i64
        let s_27_16: i64 = {
            let value = state.read_register::<i64>(s_27_15 as isize);
            tracer.read_register(s_27_15 as isize, value);
            value
        };
        // D s_27_17: cast zx s_27_16 -> i
        let s_27_17: i128 = (i128::try_from(s_27_16).unwrap());
        // D s_27_18: mutate-element s_27_14[s_27_17] <= s_27_12
        let s_27_18: [u64; 32usize] = {
            let mut local = s_27_14.clone();
            local[(s_27_17) as usize] = s_27_12;
            local
        };
        // D s_27_19: cast cvt s_27_18 -> [u64; 0]
        let s_27_19: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_27_18);
        // D s_27_20: cast cvt s_27_19 -> [u64; 32]
        let s_27_20: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_27_19);
            buf
        };
        // C s_27_21: const #13776u : u32
        let s_27_21: u32 = 13776;
        // N s_27_22: write-reg s_27_21 <= s_27_20
        let s_27_22: () = {
            state.write_register::<[u64; 32usize]>(s_27_21 as isize, s_27_20);
            tracer.write_register(s_27_21 as isize, s_27_20);
        };
        // C s_27_23: const #13776u : u32
        let s_27_23: u32 = 13776;
        // D s_27_24: read-reg s_27_23:[u64; 32]
        let s_27_24: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_23 as isize);
            tracer.read_register(s_27_23 as isize, value);
            value
        };
        // C s_27_25: const #1024u : u32
        let s_27_25: u32 = 1024;
        // D s_27_26: read-reg s_27_25:i64
        let s_27_26: i64 = {
            let value = state.read_register::<i64>(s_27_25 as isize);
            tracer.read_register(s_27_25 as isize, value);
            value
        };
        // D s_27_27: cast zx s_27_26 -> i
        let s_27_27: i128 = (i128::try_from(s_27_26).unwrap());
        // D s_27_28: read-element s_27_24[s_27_27]
        let s_27_28: u64 = s_27_24[(s_27_27) as usize];
        // D s_27_29: read-var u#153:u8
        let s_27_29: bool = fn_state.u_153;
        // D s_27_30: call Bit(s_27_29)
        let s_27_30: bool = Bit(state, tracer, s_27_29);
        // C s_27_31: const #60s : i
        let s_27_31: i128 = 60;
        // D s_27_32: cast zx s_27_28 -> bv
        let s_27_32: Bits = Bits::new(s_27_28 as u128, 64u16);
        // C s_27_33: const #1u : u64
        let s_27_33: u64 = 1;
        // D s_27_34: bit-insert s_27_32 s_27_32 s_27_31 s_27_33
        let s_27_34: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_27_33 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_27_32.length(),
            );
            (s_27_32 & mask) | (s_27_32 << s_27_31)
        };
        // D s_27_35: cast reint s_27_34 -> u64
        let s_27_35: u64 = (s_27_34.value() as u64);
        // C s_27_36: const #13776u : u32
        let s_27_36: u32 = 13776;
        // D s_27_37: read-reg s_27_36:[u64; 32]
        let s_27_37: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_36 as isize);
            tracer.read_register(s_27_36 as isize, value);
            value
        };
        // C s_27_38: const #1024u : u32
        let s_27_38: u32 = 1024;
        // D s_27_39: read-reg s_27_38:i64
        let s_27_39: i64 = {
            let value = state.read_register::<i64>(s_27_38 as isize);
            tracer.read_register(s_27_38 as isize, value);
            value
        };
        // D s_27_40: cast zx s_27_39 -> i
        let s_27_40: i128 = (i128::try_from(s_27_39).unwrap());
        // D s_27_41: mutate-element s_27_37[s_27_40] <= s_27_35
        let s_27_41: [u64; 32usize] = {
            let mut local = s_27_37.clone();
            local[(s_27_40) as usize] = s_27_35;
            local
        };
        // D s_27_42: cast cvt s_27_41 -> [u64; 0]
        let s_27_42: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_27_41);
        // D s_27_43: cast cvt s_27_42 -> [u64; 32]
        let s_27_43: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_27_42);
            buf
        };
        // C s_27_44: const #13776u : u32
        let s_27_44: u32 = 13776;
        // N s_27_45: write-reg s_27_44 <= s_27_43
        let s_27_45: () = {
            state.write_register::<[u64; 32usize]>(s_27_44 as isize, s_27_43);
            tracer.write_register(s_27_44 as isize, s_27_43);
        };
        // C s_27_46: const #13776u : u32
        let s_27_46: u32 = 13776;
        // D s_27_47: read-reg s_27_46:[u64; 32]
        let s_27_47: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_46 as isize);
            tracer.read_register(s_27_46 as isize, value);
            value
        };
        // C s_27_48: const #1024u : u32
        let s_27_48: u32 = 1024;
        // D s_27_49: read-reg s_27_48:i64
        let s_27_49: i64 = {
            let value = state.read_register::<i64>(s_27_48 as isize);
            tracer.read_register(s_27_48 as isize, value);
            value
        };
        // D s_27_50: cast zx s_27_49 -> i
        let s_27_50: i128 = (i128::try_from(s_27_49).unwrap());
        // D s_27_51: read-element s_27_47[s_27_50]
        let s_27_51: u64 = s_27_47[(s_27_50) as usize];
        // C s_27_52: const #16975u : u32
        let s_27_52: u32 = 16975;
        // D s_27_53: read-reg s_27_52:u8
        let s_27_53: u8 = {
            let value = state.read_register::<u8>(s_27_52 as isize);
            tracer.read_register(s_27_52 as isize, value);
            value
        };
        // C s_27_54: const #61s : i
        let s_27_54: i128 = 61;
        // D s_27_55: cast zx s_27_51 -> bv
        let s_27_55: Bits = Bits::new(s_27_51 as u128, 64u16);
        // D s_27_56: cast zx s_27_53 -> bv
        let s_27_56: Bits = Bits::new(s_27_53 as u128, 2u16);
        // C s_27_57: const #1s : i
        let s_27_57: i128 = 1;
        // C s_27_58: const #1u : u64
        let s_27_58: u64 = 1;
        // C s_27_59: cast zx s_27_58 -> bv
        let s_27_59: Bits = Bits::new(s_27_58 as u128, 64u16);
        // C s_27_60: lsl s_27_59 s_27_57
        let s_27_60: Bits = s_27_59 << s_27_57;
        // C s_27_61: sub s_27_60 s_27_59
        let s_27_61: Bits = ((s_27_60) - (s_27_59));
        // D s_27_62: and s_27_56 s_27_61
        let s_27_62: Bits = ((s_27_56) & (s_27_61));
        // D s_27_63: lsl s_27_62 s_27_54
        let s_27_63: Bits = s_27_62 << s_27_54;
        // C s_27_64: lsl s_27_61 s_27_54
        let s_27_64: Bits = s_27_61 << s_27_54;
        // C s_27_65: cmpl s_27_64
        let s_27_65: Bits = !s_27_64;
        // D s_27_66: and s_27_55 s_27_65
        let s_27_66: Bits = ((s_27_55) & (s_27_65));
        // D s_27_67: or s_27_66 s_27_63
        let s_27_67: Bits = ((s_27_66) | (s_27_63));
        // D s_27_68: cast reint s_27_67 -> u64
        let s_27_68: u64 = (s_27_67.value() as u64);
        // C s_27_69: const #13776u : u32
        let s_27_69: u32 = 13776;
        // D s_27_70: read-reg s_27_69:[u64; 32]
        let s_27_70: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_27_69 as isize);
            tracer.read_register(s_27_69 as isize, value);
            value
        };
        // C s_27_71: const #1024u : u32
        let s_27_71: u32 = 1024;
        // D s_27_72: read-reg s_27_71:i64
        let s_27_72: i64 = {
            let value = state.read_register::<i64>(s_27_71 as isize);
            tracer.read_register(s_27_71 as isize, value);
            value
        };
        // D s_27_73: cast zx s_27_72 -> i
        let s_27_73: i128 = (i128::try_from(s_27_72).unwrap());
        // D s_27_74: mutate-element s_27_70[s_27_73] <= s_27_68
        let s_27_74: [u64; 32usize] = {
            let mut local = s_27_70.clone();
            local[(s_27_73) as usize] = s_27_68;
            local
        };
        // D s_27_75: cast cvt s_27_74 -> [u64; 0]
        let s_27_75: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_27_74);
        // D s_27_76: cast cvt s_27_75 -> [u64; 32]
        let s_27_76: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_27_75);
            buf
        };
        // C s_27_77: const #13776u : u32
        let s_27_77: u32 = 13776;
        // N s_27_78: write-reg s_27_77 <= s_27_76
        let s_27_78: () = {
            state.write_register::<[u64; 32usize]>(s_27_77 as isize, s_27_76);
            tracer.write_register(s_27_77 as isize, s_27_76);
        };
        // C s_27_79: const #11136u : u32
        let s_27_79: u32 = 11136;
        // D s_27_80: read-reg s_27_79:[u8; 32]
        let s_27_80: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_27_79 as isize);
            tracer.read_register(s_27_79 as isize, value);
            value
        };
        // C s_27_81: const #1024u : u32
        let s_27_81: u32 = 1024;
        // D s_27_82: read-reg s_27_81:i64
        let s_27_82: i64 = {
            let value = state.read_register::<i64>(s_27_81 as isize);
            tracer.read_register(s_27_81 as isize, value);
            value
        };
        // D s_27_83: cast zx s_27_82 -> i
        let s_27_83: i128 = (i128::try_from(s_27_82).unwrap());
        // C s_27_84: const #1u : u8
        let s_27_84: bool = true;
        // D s_27_85: mutate-element s_27_80[s_27_83] <= s_27_84
        let s_27_85: [bool; 32usize] = {
            let mut local = s_27_80.clone();
            local[(s_27_83) as usize] = s_27_84;
            local
        };
        // D s_27_86: cast cvt s_27_85 -> [u8; 0]
        let s_27_86: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_27_85);
        // D s_27_87: cast cvt s_27_86 -> [u8; 32]
        let s_27_87: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_27_86);
            buf
        };
        // C s_27_88: const #11136u : u32
        let s_27_88: u32 = 11136;
        // N s_27_89: write-reg s_27_88 <= s_27_87
        let s_27_89: () = {
            state.write_register::<[bool; 32usize]>(s_27_88 as isize, s_27_87);
            tracer.write_register(s_27_88 as isize, s_27_87);
        };
        // N s_27_90: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u32
        let s_28_0: u32 = 0;
        // D s_28_1: read-var ga#2218:u32
        let s_28_1: u32 = fn_state.ga_2218;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b30 b29
        if s_28_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var u#152 <= s_29_0
        fn_state.u_152 = s_29_0;
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // D s_29_3: write-var u#153 <= s_29_2
        fn_state.u_153 = s_29_2;
        // N s_29_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #2u : u32
        let s_30_0: u32 = 2;
        // D s_30_1: read-var ga#2218:u32
        let s_30_1: u32 = fn_state.ga_2218;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b32 b31
        if s_30_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var u#152 <= s_31_0
        fn_state.u_152 = s_31_0;
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // D s_31_3: write-var u#153 <= s_31_2
        fn_state.u_153 = s_31_2;
        // N s_31_4: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call Unreachable(s_32_0)
        let s_32_1: () = Unreachable(state, tracer, s_32_0);
        // N s_32_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var ga#2211 <= s_33_0
        fn_state.ga_2211 = s_33_0;
        // N s_33_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var ga#2209 <= s_34_0
        fn_state.ga_2209 = s_34_0;
        // N s_34_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#3628 <= s_35_0
        fn_state.gs_3628 = s_35_0;
        // N s_35_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var taken_flag:u8
        let s_36_0: bool = fn_state.taken_flag;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_37_0: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // C s_38_1: const #20728u : u32
        let s_38_1: u32 = 20728;
        // N s_38_2: write-reg s_38_1 <= s_38_0
        let s_38_2: () = {
            state.write_register::<bool>(s_38_1 as isize, s_38_0);
            tracer.write_register(s_38_1 as isize, s_38_0);
        };
        // N s_38_3: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #22416u : u32
        let s_39_0: u32 = 22416;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: bool = {
            let value = state.read_register::<bool>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // N s_39_2: branch s_39_1 b49 b40
        if s_39_1 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_40_0: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var target:bv
        let s_41_0: Bits = fn_state.target;
        // D s_41_1: size-of s_41_0
        let s_41_1: u16 = s_41_0.length();
        // D s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (i128::try_from(s_41_1).unwrap());
        // C s_41_3: const #55s : i
        let s_41_3: i128 = 55;
        // D s_41_4: cmp-lt s_41_3 s_41_2
        let s_41_4: bool = ((s_41_3) < (s_41_2));
        // N s_41_5: assert s_41_4
        let s_41_5: () = assert!(s_41_4);
        // C s_41_6: const #0s : i
        let s_41_6: i128 = 0;
        // D s_41_7: read-var target:bv
        let s_41_7: Bits = fn_state.target;
        // C s_41_8: const #1s : i64
        let s_41_8: i64 = 1;
        // C s_41_9: cast zx s_41_8 -> i
        let s_41_9: i128 = (i128::try_from(s_41_8).unwrap());
        // C s_41_10: const #55s : i
        let s_41_10: i128 = 55;
        // C s_41_11: add s_41_10 s_41_9
        let s_41_11: i128 = (s_41_10 + s_41_9);
        // D s_41_12: bit-extract s_41_7 s_41_6 s_41_11
        let s_41_12: Bits = (Bits::new(
            ((s_41_7) >> (s_41_6)).value(),
            u16::try_from(s_41_11).unwrap(),
        ));
        // D s_41_13: cast reint s_41_12 -> u56
        let s_41_13: u64 = (s_41_12.value() as u64);
        // C s_41_14: const #0s : i
        let s_41_14: i128 = 0;
        // C s_41_15: const #15440u : u32
        let s_41_15: u32 = 15440;
        // D s_41_16: read-reg s_41_15:u64
        let s_41_16: u64 = {
            let value = state.read_register::<u64>(s_41_15 as isize);
            tracer.read_register(s_41_15 as isize, value);
            value
        };
        // D s_41_17: cast zx s_41_16 -> bv
        let s_41_17: Bits = Bits::new(s_41_16 as u128, 64u16);
        // D s_41_18: cast zx s_41_13 -> bv
        let s_41_18: Bits = Bits::new(s_41_13 as u128, 56u16);
        // C s_41_19: const #55s : i
        let s_41_19: i128 = 55;
        // C s_41_20: const #1u : u64
        let s_41_20: u64 = 1;
        // C s_41_21: cast zx s_41_20 -> bv
        let s_41_21: Bits = Bits::new(s_41_20 as u128, 64u16);
        // C s_41_22: lsl s_41_21 s_41_19
        let s_41_22: Bits = s_41_21 << s_41_19;
        // C s_41_23: sub s_41_22 s_41_21
        let s_41_23: Bits = ((s_41_22) - (s_41_21));
        // D s_41_24: and s_41_18 s_41_23
        let s_41_24: Bits = ((s_41_18) & (s_41_23));
        // D s_41_25: lsl s_41_24 s_41_14
        let s_41_25: Bits = s_41_24 << s_41_14;
        // C s_41_26: lsl s_41_23 s_41_14
        let s_41_26: Bits = s_41_23 << s_41_14;
        // C s_41_27: cmpl s_41_26
        let s_41_27: Bits = !s_41_26;
        // D s_41_28: and s_41_17 s_41_27
        let s_41_28: Bits = ((s_41_17) & (s_41_27));
        // D s_41_29: or s_41_28 s_41_25
        let s_41_29: Bits = ((s_41_28) | (s_41_25));
        // D s_41_30: cast reint s_41_29 -> u64
        let s_41_30: u64 = (s_41_29.value() as u64);
        // C s_41_31: const #15440u : u32
        let s_41_31: u32 = 15440;
        // N s_41_32: write-reg s_41_31 <= s_41_30
        let s_41_32: () = {
            state.write_register::<u64>(s_41_31 as isize, s_41_30);
            tracer.write_register(s_41_31 as isize, s_41_30);
        };
        // C s_41_33: const #() : ()
        let s_41_33: () = ();
        // S s_41_34: call CurrentSecurityState(s_41_33)
        let s_41_34: u32 = CurrentSecurityState(state, tracer, s_41_33);
        // D s_41_35: write-var ga#2203 <= s_41_34
        fn_state.ga_2203 = s_41_34;
        // C s_41_36: const #3u : u32
        let s_41_36: u32 = 3;
        // D s_41_37: read-var ga#2203:u32
        let s_41_37: u32 = fn_state.ga_2203;
        // D s_41_38: cmp-eq s_41_36 s_41_37
        let s_41_38: bool = ((s_41_36) == (s_41_37));
        // D s_41_39: not s_41_38
        let s_41_39: bool = !s_41_38;
        // N s_41_40: branch s_41_39 b44 b42
        if s_41_39 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var ns <= s_42_0
        fn_state.ns = s_42_0;
        // C s_42_2: const #0u : u8
        let s_42_2: bool = false;
        // D s_42_3: write-var nse <= s_42_2
        fn_state.nse = s_42_2;
        // N s_42_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var ns:u8
        let s_43_0: bool = fn_state.ns;
        // D s_43_1: call Bit(s_43_0)
        let s_43_1: bool = Bit(state, tracer, s_43_0);
        // C s_43_2: const #63s : i
        let s_43_2: i128 = 63;
        // C s_43_3: const #15440u : u32
        let s_43_3: u32 = 15440;
        // D s_43_4: read-reg s_43_3:u64
        let s_43_4: u64 = {
            let value = state.read_register::<u64>(s_43_3 as isize);
            tracer.read_register(s_43_3 as isize, value);
            value
        };
        // D s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 64u16);
        // C s_43_6: const #1u : u64
        let s_43_6: u64 = 1;
        // D s_43_7: bit-insert s_43_5 s_43_5 s_43_2 s_43_6
        let s_43_7: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_43_6 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_43_5.length(),
            );
            (s_43_5 & mask) | (s_43_5 << s_43_2)
        };
        // D s_43_8: cast reint s_43_7 -> u64
        let s_43_8: u64 = (s_43_7.value() as u64);
        // C s_43_9: const #15440u : u32
        let s_43_9: u32 = 15440;
        // N s_43_10: write-reg s_43_9 <= s_43_8
        let s_43_10: () = {
            state.write_register::<u64>(s_43_9 as isize, s_43_8);
            tracer.write_register(s_43_9 as isize, s_43_8);
        };
        // D s_43_11: read-var nse:u8
        let s_43_11: bool = fn_state.nse;
        // D s_43_12: call Bit(s_43_11)
        let s_43_12: bool = Bit(state, tracer, s_43_11);
        // C s_43_13: const #60s : i
        let s_43_13: i128 = 60;
        // C s_43_14: const #15440u : u32
        let s_43_14: u32 = 15440;
        // D s_43_15: read-reg s_43_14:u64
        let s_43_15: u64 = {
            let value = state.read_register::<u64>(s_43_14 as isize);
            tracer.read_register(s_43_14 as isize, value);
            value
        };
        // D s_43_16: cast zx s_43_15 -> bv
        let s_43_16: Bits = Bits::new(s_43_15 as u128, 64u16);
        // C s_43_17: const #1u : u64
        let s_43_17: u64 = 1;
        // D s_43_18: bit-insert s_43_16 s_43_16 s_43_13 s_43_17
        let s_43_18: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_43_17 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_43_16.length(),
            );
            (s_43_16 & mask) | (s_43_16 << s_43_13)
        };
        // D s_43_19: cast reint s_43_18 -> u64
        let s_43_19: u64 = (s_43_18.value() as u64);
        // C s_43_20: const #15440u : u32
        let s_43_20: u32 = 15440;
        // N s_43_21: write-reg s_43_20 <= s_43_19
        let s_43_21: () = {
            state.write_register::<u64>(s_43_20 as isize, s_43_19);
            tracer.write_register(s_43_20 as isize, s_43_19);
        };
        // C s_43_22: const #16975u : u32
        let s_43_22: u32 = 16975;
        // D s_43_23: read-reg s_43_22:u8
        let s_43_23: u8 = {
            let value = state.read_register::<u8>(s_43_22 as isize);
            tracer.read_register(s_43_22 as isize, value);
            value
        };
        // C s_43_24: const #61s : i
        let s_43_24: i128 = 61;
        // C s_43_25: const #15440u : u32
        let s_43_25: u32 = 15440;
        // D s_43_26: read-reg s_43_25:u64
        let s_43_26: u64 = {
            let value = state.read_register::<u64>(s_43_25 as isize);
            tracer.read_register(s_43_25 as isize, value);
            value
        };
        // D s_43_27: cast zx s_43_26 -> bv
        let s_43_27: Bits = Bits::new(s_43_26 as u128, 64u16);
        // D s_43_28: cast zx s_43_23 -> bv
        let s_43_28: Bits = Bits::new(s_43_23 as u128, 2u16);
        // C s_43_29: const #1s : i
        let s_43_29: i128 = 1;
        // C s_43_30: const #1u : u64
        let s_43_30: u64 = 1;
        // C s_43_31: cast zx s_43_30 -> bv
        let s_43_31: Bits = Bits::new(s_43_30 as u128, 64u16);
        // C s_43_32: lsl s_43_31 s_43_29
        let s_43_32: Bits = s_43_31 << s_43_29;
        // C s_43_33: sub s_43_32 s_43_31
        let s_43_33: Bits = ((s_43_32) - (s_43_31));
        // D s_43_34: and s_43_28 s_43_33
        let s_43_34: Bits = ((s_43_28) & (s_43_33));
        // D s_43_35: lsl s_43_34 s_43_24
        let s_43_35: Bits = s_43_34 << s_43_24;
        // C s_43_36: lsl s_43_33 s_43_24
        let s_43_36: Bits = s_43_33 << s_43_24;
        // C s_43_37: cmpl s_43_36
        let s_43_37: Bits = !s_43_36;
        // D s_43_38: and s_43_27 s_43_37
        let s_43_38: Bits = ((s_43_27) & (s_43_37));
        // D s_43_39: or s_43_38 s_43_35
        let s_43_39: Bits = ((s_43_38) | (s_43_35));
        // D s_43_40: cast reint s_43_39 -> u64
        let s_43_40: u64 = (s_43_39.value() as u64);
        // C s_43_41: const #15440u : u32
        let s_43_41: u32 = 15440;
        // N s_43_42: write-reg s_43_41 <= s_43_40
        let s_43_42: () = {
            state.write_register::<u64>(s_43_41 as isize, s_43_40);
            tracer.write_register(s_43_41 as isize, s_43_40);
        };
        // C s_43_43: const #1u : u8
        let s_43_43: bool = true;
        // C s_43_44: const #20728u : u32
        let s_43_44: u32 = 20728;
        // N s_43_45: write-reg s_43_44 <= s_43_43
        let s_43_45: () = {
            state.write_register::<bool>(s_43_44 as isize, s_43_43);
            tracer.write_register(s_43_44 as isize, s_43_43);
        };
        // N s_43_46: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u32
        let s_44_0: u32 = 0;
        // D s_44_1: read-var ga#2203:u32
        let s_44_1: u32 = fn_state.ga_2203;
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
    ) -> () {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var ns <= s_45_0
        fn_state.ns = s_45_0;
        // C s_45_2: const #0u : u8
        let s_45_2: bool = false;
        // D s_45_3: write-var nse <= s_45_2
        fn_state.nse = s_45_2;
        // N s_45_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #2u : u32
        let s_46_0: u32 = 2;
        // D s_46_1: read-var ga#2203:u32
        let s_46_1: u32 = fn_state.ga_2203;
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
    ) -> () {
        // C s_47_0: const #1u : u8
        let s_47_0: bool = true;
        // D s_47_1: write-var ns <= s_47_0
        fn_state.ns = s_47_0;
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // D s_47_3: write-var nse <= s_47_2
        fn_state.nse = s_47_2;
        // N s_47_4: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call Unreachable(s_48_0)
        let s_48_1: () = Unreachable(state, tracer, s_48_0);
        // N s_48_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #15440u : u32
        let s_49_0: u32 = 15440;
        // D s_49_1: read-reg s_49_0:u64
        let s_49_1: u64 = {
            let value = state.read_register::<u64>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #13776u : u32
        let s_49_2: u32 = 13776;
        // D s_49_3: read-reg s_49_2:[u64; 32]
        let s_49_3: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_49_2 as isize);
            tracer.read_register(s_49_2 as isize, value);
            value
        };
        // C s_49_4: const #1048u : u32
        let s_49_4: u32 = 1048;
        // D s_49_5: read-reg s_49_4:i64
        let s_49_5: i64 = {
            let value = state.read_register::<i64>(s_49_4 as isize);
            tracer.read_register(s_49_4 as isize, value);
            value
        };
        // D s_49_6: cast zx s_49_5 -> i
        let s_49_6: i128 = (i128::try_from(s_49_5).unwrap());
        // D s_49_7: read-element s_49_3[s_49_6]
        let s_49_7: u64 = s_49_3[(s_49_6) as usize];
        // C s_49_8: const #0s : i
        let s_49_8: i128 = 0;
        // D s_49_9: cast zx s_49_1 -> bv
        let s_49_9: Bits = Bits::new(s_49_1 as u128, 64u16);
        // C s_49_10: const #1s : i64
        let s_49_10: i64 = 1;
        // C s_49_11: cast zx s_49_10 -> i
        let s_49_11: i128 = (i128::try_from(s_49_10).unwrap());
        // C s_49_12: const #63s : i
        let s_49_12: i128 = 63;
        // C s_49_13: add s_49_12 s_49_11
        let s_49_13: i128 = (s_49_12 + s_49_11);
        // D s_49_14: bit-extract s_49_9 s_49_8 s_49_13
        let s_49_14: Bits = (Bits::new(
            ((s_49_9) >> (s_49_8)).value(),
            u16::try_from(s_49_13).unwrap(),
        ));
        // D s_49_15: cast reint s_49_14 -> u64
        let s_49_15: u64 = (s_49_14.value() as u64);
        // C s_49_16: const #0s : i
        let s_49_16: i128 = 0;
        // D s_49_17: cast zx s_49_7 -> bv
        let s_49_17: Bits = Bits::new(s_49_7 as u128, 64u16);
        // D s_49_18: cast zx s_49_15 -> bv
        let s_49_18: Bits = Bits::new(s_49_15 as u128, 64u16);
        // C s_49_19: const #63s : i
        let s_49_19: i128 = 63;
        // C s_49_20: const #1u : u64
        let s_49_20: u64 = 1;
        // C s_49_21: cast zx s_49_20 -> bv
        let s_49_21: Bits = Bits::new(s_49_20 as u128, 64u16);
        // C s_49_22: lsl s_49_21 s_49_19
        let s_49_22: Bits = s_49_21 << s_49_19;
        // C s_49_23: sub s_49_22 s_49_21
        let s_49_23: Bits = ((s_49_22) - (s_49_21));
        // D s_49_24: and s_49_18 s_49_23
        let s_49_24: Bits = ((s_49_18) & (s_49_23));
        // D s_49_25: lsl s_49_24 s_49_16
        let s_49_25: Bits = s_49_24 << s_49_16;
        // C s_49_26: lsl s_49_23 s_49_16
        let s_49_26: Bits = s_49_23 << s_49_16;
        // C s_49_27: cmpl s_49_26
        let s_49_27: Bits = !s_49_26;
        // D s_49_28: and s_49_17 s_49_27
        let s_49_28: Bits = ((s_49_17) & (s_49_27));
        // D s_49_29: or s_49_28 s_49_25
        let s_49_29: Bits = ((s_49_28) | (s_49_25));
        // D s_49_30: cast reint s_49_29 -> u64
        let s_49_30: u64 = (s_49_29.value() as u64);
        // C s_49_31: const #13776u : u32
        let s_49_31: u32 = 13776;
        // D s_49_32: read-reg s_49_31:[u64; 32]
        let s_49_32: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_49_31 as isize);
            tracer.read_register(s_49_31 as isize, value);
            value
        };
        // C s_49_33: const #1048u : u32
        let s_49_33: u32 = 1048;
        // D s_49_34: read-reg s_49_33:i64
        let s_49_34: i64 = {
            let value = state.read_register::<i64>(s_49_33 as isize);
            tracer.read_register(s_49_33 as isize, value);
            value
        };
        // D s_49_35: cast zx s_49_34 -> i
        let s_49_35: i128 = (i128::try_from(s_49_34).unwrap());
        // D s_49_36: mutate-element s_49_32[s_49_35] <= s_49_30
        let s_49_36: [u64; 32usize] = {
            let mut local = s_49_32.clone();
            local[(s_49_35) as usize] = s_49_30;
            local
        };
        // D s_49_37: cast cvt s_49_36 -> [u64; 0]
        let s_49_37: alloc::vec::Vec<u64> = alloc::vec::Vec::from(s_49_36);
        // D s_49_38: cast cvt s_49_37 -> [u64; 32]
        let s_49_38: [u64; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_49_37);
            buf
        };
        // C s_49_39: const #13776u : u32
        let s_49_39: u32 = 13776;
        // N s_49_40: write-reg s_49_39 <= s_49_38
        let s_49_40: () = {
            state.write_register::<[u64; 32usize]>(s_49_39 as isize, s_49_38);
            tracer.write_register(s_49_39 as isize, s_49_38);
        };
        // C s_49_41: const #20728u : u32
        let s_49_41: u32 = 20728;
        // D s_49_42: read-reg s_49_41:u8
        let s_49_42: bool = {
            let value = state.read_register::<bool>(s_49_41 as isize);
            tracer.read_register(s_49_41 as isize, value);
            value
        };
        // C s_49_43: const #11136u : u32
        let s_49_43: u32 = 11136;
        // D s_49_44: read-reg s_49_43:[u8; 32]
        let s_49_44: [bool; 32usize] = {
            let value = state.read_register::<[bool; 32usize]>(s_49_43 as isize);
            tracer.read_register(s_49_43 as isize, value);
            value
        };
        // C s_49_45: const #1048u : u32
        let s_49_45: u32 = 1048;
        // D s_49_46: read-reg s_49_45:i64
        let s_49_46: i64 = {
            let value = state.read_register::<i64>(s_49_45 as isize);
            tracer.read_register(s_49_45 as isize, value);
            value
        };
        // D s_49_47: cast zx s_49_46 -> i
        let s_49_47: i128 = (i128::try_from(s_49_46).unwrap());
        // D s_49_48: mutate-element s_49_44[s_49_47] <= s_49_42
        let s_49_48: [bool; 32usize] = {
            let mut local = s_49_44.clone();
            local[(s_49_47) as usize] = s_49_42;
            local
        };
        // D s_49_49: cast cvt s_49_48 -> [u8; 0]
        let s_49_49: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_49_48);
        // D s_49_50: cast cvt s_49_49 -> [u8; 32]
        let s_49_50: [bool; 32usize] = {
            let mut buf = [Default::default(); 32usize];
            buf.copy_from_slice(&s_49_49);
            buf
        };
        // C s_49_51: const #11136u : u32
        let s_49_51: u32 = 11136;
        // N s_49_52: write-reg s_49_51 <= s_49_50
        let s_49_52: () = {
            state.write_register::<[bool; 32usize]>(s_49_51 as isize, s_49_50);
            tracer.write_register(s_49_51 as isize, s_49_50);
        };
        // N s_49_53: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var collect_prev_brshadow#34:u8
        let s_50_0: bool = fn_state.collect_prev_brshadow_34;
        // D s_50_1: write-var gs#3626 <= s_50_0
        fn_state.gs_3626 = s_50_0;
        // N s_50_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call StatisticalProfilingEnabled(s_51_0)
        let s_51_1: bool = StatisticalProfilingEnabled(state, tracer, s_51_0);
        // D s_51_2: write-var gs#3625 <= s_51_1
        fn_state.gs_3625 = s_51_1;
        // N s_51_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #14736u : u32
        let s_52_0: u32 = 14736;
        // D s_52_1: read-reg s_52_0:struct
        let s_52_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_52_0 as isize);
            tracer.read_register(s_52_0 as isize, value);
            value
        };
        // D s_52_2: call _get_PMSIDR_EL1_Type_PBT(s_52_1)
        let s_52_2: bool = u_get_PMSIDR_EL1_Type_PBT(state, tracer, s_52_1);
        // D s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: call IsZero(s_52_3)
        let s_52_4: bool = IsZero(state, tracer, s_52_3);
        // D s_52_5: not s_52_4
        let s_52_5: bool = !s_52_4;
        // D s_52_6: write-var gs#3624 <= s_52_5
        fn_state.gs_3624 = s_52_5;
        // N s_52_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #2u : u32
        let s_53_0: u32 = 2;
        // D s_53_1: read-var branch_type:u32
        let s_53_1: u32 = fn_state.branch_type;
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // N s_53_4: branch s_53_3 b55 b54
        if s_53_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var collect_prev_br_eret:u8
        let s_54_0: bool = fn_state.collect_prev_br_eret;
        // D s_54_1: write-var collect_prev_br <= s_54_0
        fn_state.collect_prev_br = s_54_0;
        // N s_54_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var is_isb:u8
        let s_55_0: bool = fn_state.is_isb;
        // D s_55_1: not s_55_0
        let s_55_1: bool = !s_55_0;
        // N s_55_2: branch s_55_1 b58 b56
        if s_55_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var collect_prev_br_isb:u8
        let s_56_0: bool = fn_state.collect_prev_br_isb;
        // D s_56_1: write-var gs#3621 <= s_56_0
        fn_state.gs_3621 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#3621:u8
        let s_57_0: bool = fn_state.gs_3621;
        // D s_57_1: write-var collect_prev_br <= s_57_0
        fn_state.collect_prev_br = s_57_0;
        // N s_57_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #1u : u8
        let s_58_0: bool = true;
        // D s_58_1: write-var gs#3621 <= s_58_0
        fn_state.gs_3621 = s_58_0;
        // N s_58_2: jump b57
        return block_57(state, tracer, fn_state);
    }
}
