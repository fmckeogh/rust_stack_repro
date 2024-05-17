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
use u__UNKNOWN_bits::*;
use GranuleProtectionCheck::*;
use IsFault__1::*;
use HaveRME::*;
use PhysMemRead::*;
use HandleExternalTTWAbort::*;
use PhysMemWrite::*;
use reverse_endianness::*;
use common::*;
pub fn AArch64_MemSwapTableDesc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    fault_in: ProductType1d757adad216cdef,
    prev_desc: Bits,
    new_desc: Bits,
    ee: bool,
    descaccess: ProductType9878976b5bcce9c9,
    descpaddr: ProductTypece7c66ccb2cab13e,
    translation_info: ProductTypeb525737120e184b3,
) -> ProductTypeb4cea7287e2eb9d6 {
    #[derive(Default)]
    struct FunctionState {
        memstatusshadow_290: ProductTypef8c3639b88223255,
        ordered_new_descshadow_289: Bits,
        fault: ProductType1d757adad216cdef,
        return_value: ProductTypeb4cea7287e2eb9d6,
        memstatus: ProductTypef8c3639b88223255,
        ga_12541: ProductType2b2aba4822138824,
        mem_desc: Bits,
        ga_12531: ProductType396b95aa74979079,
        fault_in: ProductType1d757adad216cdef,
        prev_desc: Bits,
        new_desc: Bits,
        ee: bool,
        descaccess: ProductType9878976b5bcce9c9,
        descpaddr: ProductTypece7c66ccb2cab13e,
        translation_info: ProductTypeb525737120e184b3,
    }
    let fn_state = FunctionState {
        fault_in,
        prev_desc,
        new_desc,
        ee,
        descaccess,
        descpaddr,
        translation_info,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_0_0: read-var fault_in:struct
        let s_0_0: ProductType1d757adad216cdef = fn_state.fault_in;
        // D s_0_1: write-var fault <= s_0_0
        fn_state.fault = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call HaveRME(s_0_2)
        let s_0_3: bool = HaveRME(state, tracer, s_0_2);
        // N s_0_4: branch s_0_3 b23 b1
        if s_0_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_2_0: read-var prev_desc:bv
        let s_2_0: Bits = fn_state.prev_desc;
        // D s_2_1: size-of s_2_0
        let s_2_1: u16 = s_2_0.length();
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #8s : i
        let s_2_4: i128 = 8;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: div s_2_5 s_2_4
        let s_2_6: i128 = ((s_2_5) / (s_2_4));
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // D s_2_8: read-var translation_info:struct
        let s_2_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_2_9: create-sum enum = 1:"s_2_8"
        let s_2_9: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_1(s_2_8);
        // D s_2_10: cast zx s_2_7 -> i
        let s_2_10: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_11: read-var descpaddr:struct
        let s_2_11: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_2_12: read-var descaccess:struct
        let s_2_12: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_2_13: call PhysMemRead(s_2_11, s_2_10, s_2_12, s_2_9)
        let s_2_13: ProductType2b2aba4822138824 = PhysMemRead(
            state,
            tracer,
            s_2_11,
            s_2_10,
            s_2_12,
            s_2_9,
        );
        // D s_2_14: write-var ga#12541 <= s_2_13
        fn_state.ga_12541 = s_2_13;
        // D s_2_15: read-var ga#12541.0:struct
        let s_2_15: ProductTypef8c3639b88223255 = fn_state.ga_12541._0;
        // D s_2_16: read-var ga#12541.1:struct
        let s_2_16: Bits = fn_state.ga_12541._1;
        // D s_2_17: write-var memstatus <= s_2_15
        fn_state.memstatus = s_2_15;
        // D s_2_18: write-var mem_desc <= s_2_16
        fn_state.mem_desc = s_2_16;
        // D s_2_19: read-var ee:u8
        let s_2_19: bool = fn_state.ee;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // C s_2_21: const #1u : u8
        let s_2_21: bool = true;
        // C s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 1u16);
        // D s_2_23: cmp-eq s_2_20 s_2_22
        let s_2_23: bool = ((s_2_20) == (s_2_22));
        // N s_2_24: branch s_2_23 b22 b3
        if s_2_23 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_4_0: read-var memstatus:struct
        let s_4_0: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_4_1: call IsFault__2(s_4_0)
        let s_4_1: bool = IsFault__2(state, tracer, s_4_0);
        // N s_4_2: branch s_4_1 b19 b5
        if s_4_1 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_6_0: read-var mem_desc:bv
        let s_6_0: Bits = fn_state.mem_desc;
        // D s_6_1: read-var prev_desc:bv
        let s_6_1: Bits = fn_state.prev_desc;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b10 b7
        if s_6_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_8_0: read-var fault:struct
        let s_8_0: ProductType1d757adad216cdef = fn_state.fault;
        // D s_8_1: read-var mem_desc:bv
        let s_8_1: Bits = fn_state.mem_desc;
        // D s_8_2: create-product struct = ["s_8_0", "s_8_1"]
        let s_8_2: ProductTypeb4cea7287e2eb9d6 = ProductTypeb4cea7287e2eb9d6 {
            _0: s_8_0,
            _1: s_8_1,
        };
        // D s_8_3: write-var return_value <= s_8_2
        fn_state.return_value = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_9_0: read-var return_value:struct
        let s_9_0: ProductTypeb4cea7287e2eb9d6 = fn_state.return_value;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_10_0: read-var ee:u8
        let s_10_0: bool = fn_state.ee;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #1u : u8
        let s_10_2: bool = true;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b18 b11
        if s_10_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_11_0: read-var new_desc:bv
        let s_11_0: Bits = fn_state.new_desc;
        // D s_11_1: write-var ordered_new_descshadow#289 <= s_11_0
        fn_state.ordered_new_descshadow_289 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_12_0: read-var prev_desc:bv
        let s_12_0: Bits = fn_state.prev_desc;
        // D s_12_1: size-of s_12_0
        let s_12_1: u16 = s_12_0.length();
        // D s_12_2: cast zx s_12_1 -> i
        let s_12_2: i128 = (i128::try_from(s_12_1).unwrap());
        // D s_12_3: cast reint s_12_2 -> i64
        let s_12_3: i64 = (s_12_2 as i64);
        // C s_12_4: const #8s : i
        let s_12_4: i128 = 8;
        // D s_12_5: cast zx s_12_3 -> i
        let s_12_5: i128 = (i128::try_from(s_12_3).unwrap());
        // D s_12_6: div s_12_5 s_12_4
        let s_12_6: i128 = ((s_12_5) / (s_12_4));
        // D s_12_7: cast reint s_12_6 -> i64
        let s_12_7: i64 = (s_12_6 as i64);
        // D s_12_8: read-var translation_info:struct
        let s_12_8: ProductTypeb525737120e184b3 = fn_state.translation_info;
        // D s_12_9: create-sum enum = 1:"s_12_8"
        let s_12_9: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_1(s_12_8);
        // D s_12_10: cast zx s_12_7 -> i
        let s_12_10: i128 = (i128::try_from(s_12_7).unwrap());
        // D s_12_11: read-var descpaddr:struct
        let s_12_11: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_12_12: read-var descaccess:struct
        let s_12_12: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_12_13: read-var ordered_new_descshadow#289:bv
        let s_12_13: Bits = fn_state.ordered_new_descshadow_289;
        // D s_12_14: call PhysMemWrite(s_12_11, s_12_10, s_12_12, s_12_9, s_12_13)
        let s_12_14: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_12_11,
            s_12_10,
            s_12_12,
            s_12_9,
            s_12_13,
        );
        // D s_12_15: write-var memstatusshadow#290 <= s_12_14
        fn_state.memstatusshadow_290 = s_12_14;
        // D s_12_16: read-var memstatusshadow#290:struct
        let s_12_16: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_290;
        // D s_12_17: call IsFault__2(s_12_16)
        let s_12_17: bool = IsFault__2(state, tracer, s_12_16);
        // N s_12_18: branch s_12_17 b15 b13
        if s_12_17 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_13_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_14_0: read-var new_desc:bv
        let s_14_0: Bits = fn_state.new_desc;
        // D s_14_1: write-var mem_desc <= s_14_0
        fn_state.mem_desc = s_14_0;
        // N s_14_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_15_0: read-var prev_desc:bv
        let s_15_0: Bits = fn_state.prev_desc;
        // D s_15_1: size-of s_15_0
        let s_15_1: u16 = s_15_0.length();
        // D s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (i128::try_from(s_15_1).unwrap());
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #8s : i
        let s_15_4: i128 = 8;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: div s_15_5 s_15_4
        let s_15_6: i128 = ((s_15_5) / (s_15_4));
        // D s_15_7: cast reint s_15_6 -> i64
        let s_15_7: i64 = (s_15_6 as i64);
        // D s_15_8: cast zx s_15_7 -> i
        let s_15_8: i128 = (i128::try_from(s_15_7).unwrap());
        // D s_15_9: read-var memstatusshadow#290:struct
        let s_15_9: ProductTypef8c3639b88223255 = fn_state.memstatusshadow_290;
        // C s_15_10: const #1u : u8
        let s_15_10: bool = true;
        // D s_15_11: read-var descpaddr:struct
        let s_15_11: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_15_12: read-var descaccess:struct
        let s_15_12: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_15_13: read-var fault:struct
        let s_15_13: ProductType1d757adad216cdef = fn_state.fault;
        // D s_15_14: call HandleExternalTTWAbort(s_15_9, s_15_10, s_15_11, s_15_12, s_15_8, s_15_13)
        let s_15_14: ProductType1d757adad216cdef = HandleExternalTTWAbort(
            state,
            tracer,
            s_15_9,
            s_15_10,
            s_15_11,
            s_15_12,
            s_15_8,
            s_15_13,
        );
        // D s_15_15: write-var fault <= s_15_14
        fn_state.fault = s_15_14;
        // D s_15_16: read-var fault.16:struct
        let s_15_16: u32 = fn_state.fault._16;
        // D s_15_17: call IsFault__1(s_15_16)
        let s_15_17: bool = IsFault__1(state, tracer, s_15_16);
        // N s_15_18: branch s_15_17 b17 b16
        if s_15_17 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_16_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_17_0: read-var prev_desc:bv
        let s_17_0: Bits = fn_state.prev_desc;
        // D s_17_1: size-of s_17_0
        let s_17_1: u16 = s_17_0.length();
        // D s_17_2: cast zx s_17_1 -> i
        let s_17_2: i128 = (i128::try_from(s_17_1).unwrap());
        // D s_17_3: cast reint s_17_2 -> i64
        let s_17_3: i64 = (s_17_2 as i64);
        // D s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // D s_17_5: cast reint s_17_4 -> i64
        let s_17_5: i64 = (s_17_4 as i64);
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_7: call __UNKNOWN_bits(s_17_6)
        let s_17_7: Bits = u__UNKNOWN_bits(state, tracer, s_17_6);
        // D s_17_8: read-var fault:struct
        let s_17_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_17_9: create-product struct = ["s_17_8", "s_17_7"]
        let s_17_9: ProductTypeb4cea7287e2eb9d6 = ProductTypeb4cea7287e2eb9d6 {
            _0: s_17_8,
            _1: s_17_7,
        };
        // D s_17_10: write-var return_value <= s_17_9
        fn_state.return_value = s_17_9;
        // N s_17_11: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_18_0: read-var new_desc:bv
        let s_18_0: Bits = fn_state.new_desc;
        // D s_18_1: call reverse_endianness(s_18_0)
        let s_18_1: Bits = reverse_endianness(state, tracer, s_18_0);
        // D s_18_2: write-var ordered_new_descshadow#289 <= s_18_1
        fn_state.ordered_new_descshadow_289 = s_18_1;
        // N s_18_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_19_0: read-var prev_desc:bv
        let s_19_0: Bits = fn_state.prev_desc;
        // D s_19_1: size-of s_19_0
        let s_19_1: u16 = s_19_0.length();
        // D s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (i128::try_from(s_19_1).unwrap());
        // D s_19_3: cast reint s_19_2 -> i64
        let s_19_3: i64 = (s_19_2 as i64);
        // C s_19_4: const #8s : i
        let s_19_4: i128 = 8;
        // D s_19_5: cast zx s_19_3 -> i
        let s_19_5: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_6: div s_19_5 s_19_4
        let s_19_6: i128 = ((s_19_5) / (s_19_4));
        // D s_19_7: cast reint s_19_6 -> i64
        let s_19_7: i64 = (s_19_6 as i64);
        // D s_19_8: cast zx s_19_7 -> i
        let s_19_8: i128 = (i128::try_from(s_19_7).unwrap());
        // D s_19_9: read-var memstatus:struct
        let s_19_9: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // C s_19_10: const #0u : u8
        let s_19_10: bool = false;
        // D s_19_11: read-var descpaddr:struct
        let s_19_11: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_19_12: read-var descaccess:struct
        let s_19_12: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_19_13: read-var fault:struct
        let s_19_13: ProductType1d757adad216cdef = fn_state.fault;
        // D s_19_14: call HandleExternalTTWAbort(s_19_9, s_19_10, s_19_11, s_19_12, s_19_8, s_19_13)
        let s_19_14: ProductType1d757adad216cdef = HandleExternalTTWAbort(
            state,
            tracer,
            s_19_9,
            s_19_10,
            s_19_11,
            s_19_12,
            s_19_8,
            s_19_13,
        );
        // D s_19_15: write-var fault <= s_19_14
        fn_state.fault = s_19_14;
        // D s_19_16: read-var fault.16:struct
        let s_19_16: u32 = fn_state.fault._16;
        // D s_19_17: call IsFault__1(s_19_16)
        let s_19_17: bool = IsFault__1(state, tracer, s_19_16);
        // N s_19_18: branch s_19_17 b21 b20
        if s_19_17 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_20_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_21_0: read-var prev_desc:bv
        let s_21_0: Bits = fn_state.prev_desc;
        // D s_21_1: size-of s_21_0
        let s_21_1: u16 = s_21_0.length();
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // D s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_5: cast reint s_21_4 -> i64
        let s_21_5: i64 = (s_21_4 as i64);
        // D s_21_6: cast zx s_21_5 -> i
        let s_21_6: i128 = (i128::try_from(s_21_5).unwrap());
        // D s_21_7: call __UNKNOWN_bits(s_21_6)
        let s_21_7: Bits = u__UNKNOWN_bits(state, tracer, s_21_6);
        // D s_21_8: read-var fault:struct
        let s_21_8: ProductType1d757adad216cdef = fn_state.fault;
        // D s_21_9: create-product struct = ["s_21_8", "s_21_7"]
        let s_21_9: ProductTypeb4cea7287e2eb9d6 = ProductTypeb4cea7287e2eb9d6 {
            _0: s_21_8,
            _1: s_21_7,
        };
        // D s_21_10: write-var return_value <= s_21_9
        fn_state.return_value = s_21_9;
        // N s_21_11: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_22_0: read-var mem_desc:bv
        let s_22_0: Bits = fn_state.mem_desc;
        // D s_22_1: call reverse_endianness(s_22_0)
        let s_22_1: Bits = reverse_endianness(state, tracer, s_22_0);
        // D s_22_2: write-var mem_desc <= s_22_1
        fn_state.mem_desc = s_22_1;
        // N s_22_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // D s_23_0: read-var descpaddr:struct
        let s_23_0: ProductTypece7c66ccb2cab13e = fn_state.descpaddr;
        // D s_23_1: read-var descaccess:struct
        let s_23_1: ProductType9878976b5bcce9c9 = fn_state.descaccess;
        // D s_23_2: call GranuleProtectionCheck(s_23_0, s_23_1)
        let s_23_2: ProductType396b95aa74979079 = GranuleProtectionCheck(
            state,
            tracer,
            s_23_0,
            s_23_1,
        );
        // D s_23_3: write-var fault.6 <= s_23_2
        fn_state.fault._6 = s_23_2;
        // D s_23_4: read-var fault.6:struct
        let s_23_4: ProductType396b95aa74979079 = fn_state.fault._6;
        // D s_23_5: write-var ga#12531 <= s_23_4
        fn_state.ga_12531 = s_23_4;
        // D s_23_6: read-var ga#12531.0:struct
        let s_23_6: u32 = fn_state.ga_12531._0;
        // C s_23_7: const #0u : u32
        let s_23_7: u32 = 0;
        // D s_23_8: cmp-eq s_23_6 s_23_7
        let s_23_8: bool = ((s_23_6) == (s_23_7));
        // N s_23_9: branch s_23_8 b25 b24
        if s_23_8 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // N s_24_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb4cea7287e2eb9d6 {
        // C s_25_0: const #12u : u32
        let s_25_0: u32 = 12;
        // D s_25_1: write-var fault.16 <= s_25_0
        fn_state.fault._16 = s_25_0;
        // D s_25_2: read-var descpaddr.3:struct
        let s_25_2: ProductTypeda0231e9dc169f81 = fn_state.descpaddr._3;
        // D s_25_3: write-var fault.12 <= s_25_2
        fn_state.fault._12 = s_25_2;
        // D s_25_4: read-var fault.15:struct
        let s_25_4: bool = fn_state.fault._15;
        // D s_25_5: write-var fault.7 <= s_25_4
        fn_state.fault._7 = s_25_4;
        // D s_25_6: read-var prev_desc:bv
        let s_25_6: Bits = fn_state.prev_desc;
        // D s_25_7: size-of s_25_6
        let s_25_7: u16 = s_25_6.length();
        // D s_25_8: cast zx s_25_7 -> i
        let s_25_8: i128 = (i128::try_from(s_25_7).unwrap());
        // D s_25_9: cast reint s_25_8 -> i64
        let s_25_9: i64 = (s_25_8 as i64);
        // D s_25_10: cast zx s_25_9 -> i
        let s_25_10: i128 = (i128::try_from(s_25_9).unwrap());
        // D s_25_11: cast reint s_25_10 -> i64
        let s_25_11: i64 = (s_25_10 as i64);
        // D s_25_12: cast zx s_25_11 -> i
        let s_25_12: i128 = (i128::try_from(s_25_11).unwrap());
        // D s_25_13: call __UNKNOWN_bits(s_25_12)
        let s_25_13: Bits = u__UNKNOWN_bits(state, tracer, s_25_12);
        // D s_25_14: read-var fault:struct
        let s_25_14: ProductType1d757adad216cdef = fn_state.fault;
        // D s_25_15: create-product struct = ["s_25_14", "s_25_13"]
        let s_25_15: ProductTypeb4cea7287e2eb9d6 = ProductTypeb4cea7287e2eb9d6 {
            _0: s_25_14,
            _1: s_25_13,
        };
        // D s_25_16: write-var return_value <= s_25_15
        fn_state.return_value = s_25_15;
        // N s_25_17: jump b9
        return block_9(state, tracer, fn_state);
    }
}
